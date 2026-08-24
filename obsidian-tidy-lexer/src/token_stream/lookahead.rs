use alloc::vec::Vec;
use core::mem::{ManuallyDrop, MaybeUninit};

/// Disables the automatic rollback of peeked elements to the buffer.
///
/// When a `LookaheadGuard` is dropped normally, its peeked elements are
/// returned to the buffer (rollback). Wrapping it in `NoRollback` prevents
/// this behavior, allowing explicit control over the elements.
///
/// This is semantically equivalent to `ManuallyDrop`, but makes the intent
/// clearer in the context of transactional lookahead operations.
type NoRollback<T> = ManuallyDrop<T>;

pub struct LookaheadGuard<'guard, I, const N: usize>
where
    I: Iterator,
{
    data: [MaybeUninit<I::Item>; N],
    lookahead: &'guard mut Lookahead<I>,
}

impl<'guard, I, const N: usize> LookaheadGuard<'guard, I, N>
where
    I: Iterator,
{
    pub const fn new(data: [MaybeUninit<I::Item>; N], lookahead: &'guard mut Lookahead<I>) -> Self {
        Self { data, lookahead }
    }

    pub const fn data(&self) -> &[I::Item; N] {
        // SAFETY:
        // - MaybeUninit<T> has the same layout and alignment as T
        // - All N elements are guaranteed initialized by peek_many
        // - The reference is valid for the lifetime of self
        unsafe { &*(self.data.as_ptr().cast::<[I::Item; N]>()) }
    }

    /// Commits the transaction and advances the buffer with new items.
    ///
    /// The original N tokens are **dropped**.
    /// New items are appended to the buffer in the same order.
    pub fn commit_with<T>(self, items: T)
    where
        T: IntoIterator<Item = I::Item>,
        T::IntoIter: DoubleEndedIterator,
    {
        let items = items.into_iter(); // If panic, then call drop
        let mut this = NoRollback::new(self);

        for item in &mut this.data {
            // SAFETY: all elements are initialized
            unsafe {
                item.assume_init_drop();
            }
        }

        for item in items.rev() {
            this.lookahead.buffer.push(item);
        }
    }

    /// Replaces the N peeked elements with the provided items and returns the first item.
    ///
    /// The original N elements are **dropped**.
    /// The first item from `items` is returned to the caller.
    /// Remaining items are pushed to the buffer in their original order
    /// (so they'll be returned by subsequent `next()` calls in that order).
    ///
    /// Returns `None` if `items` is empty.
    pub fn commit_returning_first<T>(self, items: T) -> Option<I::Item>
    where
        T: IntoIterator<Item = I::Item>,
        T::IntoIter: DoubleEndedIterator,
    {
        let mut items_iter = items.into_iter();
        let first = items_iter.next();

        let mut this = NoRollback::new(self);

        for item in &mut this.data {
            // SAFETY: all elements are initialized
            unsafe {
                item.assume_init_drop();
            }
        }

        for item in items_iter.rev() {
            this.lookahead.buffer.push(item);
        }

        first
    }

    pub fn commit(self) {
        self.commit_with([]);
    }

    pub fn commit_into(self) -> [I::Item; N] {
        let this = NoRollback::new(self);

        // SAFETY: all elements are initialized
        unsafe { core::ptr::read(this.data.as_ptr().cast::<[I::Item; N]>()) }
    }

    /// Commits the transaction, allows transforming the peeked elements,
    /// and returns the first element of the result.
    ///
    /// The `transform` closure receives the N peeked elements (by value, no cloning!)
    /// and returns a new sequence of items. The first item is returned to the caller,
    /// and the remaining items are pushed to the buffer.
    ///
    /// # Panic Safety
    ///
    /// If `transform` panics, the N peeked elements are **lost** (dropped),
    /// but the internal buffer remains valid. The `Lookahead` can continue
    /// to work, but without these N elements.
    ///
    /// This behavior is consistent with other commit methods.
    ///
    /// # Example
    /// ```ignore
    /// guard.commit_with_peeked(|[a, b, c]| {
    ///     vec![new_item, a, b, c]  // No cloning!
    /// })
    /// ```
    pub fn commit_with_peeked<F>(self, transform: F) -> Option<I::Item>
    where
        F: FnOnce([I::Item; N]) -> alloc::vec::Vec<I::Item>,
    {
        let mut this = ManuallyDrop::new(self);

        // SAFETY: all elements are initialized by peek_many
        let peeked = unsafe { core::ptr::read(this.data.as_ptr().cast::<[I::Item; N]>()) };

        let result = transform(peeked);
        let mut iter = result.into_iter();
        let first = iter.next();

        for item in iter.rev() {
            this.lookahead.buffer.push(item);
        }

        first
    }
}

impl<I, const N: usize> Drop for LookaheadGuard<'_, I, N>
where
    I: Iterator,
{
    /// Automatic rollback: returns all peeked elements back to the buffer.
    ///
    /// This is called when the guard is dropped without an explicit commit.
    /// To prevent this behavior, wrap the guard in [`NoRollback`] before dropping.
    fn drop(&mut self) {
        for i in (0..N).rev() {
            unsafe {
                // SAFETY: all elements are initialized
                #[expect(
                    clippy::indexing_slicing,
                    reason = "Index `i` comes from `(0..N).rev()`, so it is guaranteed to be within array bounds"
                )]
                let item =
                    core::mem::replace(&mut self.data[i], MaybeUninit::uninit()).assume_init();

                self.lookahead.buffer.push(item);
            }
        }
    }
}

/// A transactional lookahead buffer for iterators.
///
/// Allows peeking N elements ahead without consuming them.
/// If the transaction is not explicitly committed via [`LookaheadGuard::commit`],
/// the peeked elements are automatically rolled back when the guard is dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookahead<I>
where
    I: Iterator,
{
    inner: I,
    buffer: Vec<I::Item>,
}

impl<I> Lookahead<I>
where
    I: Iterator,
{
    pub const fn new(inner: I) -> Self {
        Self {
            inner,
            buffer: Vec::new(),
        }
    }

    #[expect(
        clippy::elidable_lifetime_names,
        reason = "Explicit `'guard` lifetime matches `peek_many` and emphasizes connection to the `Lookahead` borrow"
    )]
    pub fn peek<'guard>(&'guard mut self) -> Option<LookaheadGuard<'guard, I, 1>> {
        self.peek_many::<1>()
    }

    #[expect(
        clippy::elidable_lifetime_names,
        reason = "Explicit `'guard` lifetime links the returned `LookaheadGuard` to the `Lookahead` borrow"
    )]
    #[expect(
        clippy::indexing_slicing,
        reason = "Index `i` is bound by `0..N` loop over an array of size `N`, making panic impossible"
    )]
    pub fn peek_many<'guard, const N: usize>(
        &'guard mut self,
    ) -> Option<LookaheadGuard<'guard, I, N>> {
        let mut data: [MaybeUninit<I::Item>; N] = core::array::from_fn(|_| MaybeUninit::uninit());

        for i in 0..N {
            if let Some(item) = self.buffer.pop() {
                data[i] = MaybeUninit::new(item);
            } else if let Some(item) = self.inner.next() {
                data[i] = MaybeUninit::new(item);
            } else {
                for j in (0..i).rev() {
                    // SAFETY: all elements are initialized
                    unsafe {
                        let item =
                            core::mem::replace(&mut data[j], MaybeUninit::uninit()).assume_init();

                        self.buffer.push(item);
                    }
                }

                return None;
            }
        }

        Some(LookaheadGuard::new(data, self))
    }
}

impl<I> Iterator for Lookahead<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if let Some(item) = self.buffer.pop() {
            Some(item)
        } else {
            self.inner.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::rc::Rc;
    use alloc::vec;

    struct PanickingIntoIterator<T>(core::marker::PhantomData<T>);

    impl<T> PanickingIntoIterator<T> {
        fn new() -> Self {
            Self(core::marker::PhantomData)
        }
    }

    impl<T> IntoIterator for PanickingIntoIterator<T> {
        type Item = T;
        type IntoIter = alloc::vec::IntoIter<T>;

        #[track_caller]
        fn into_iter(self) -> Self::IntoIter {
            panic!("panic in PanickingIntoIterator!");
        }
    }

    struct PanickingIterator<T>(core::marker::PhantomData<T>);

    impl<T> PanickingIterator<T> {
        fn new() -> Self {
            Self(core::marker::PhantomData)
        }
    }

    impl<T> Iterator for PanickingIterator<T> {
        type Item = T;

        #[track_caller]
        fn next(&mut self) -> Option<Self::Item> {
            panic!("panic in PanickingIterator!");
        }
    }

    impl<T> DoubleEndedIterator for PanickingIterator<T> {
        #[track_caller]
        fn next_back(&mut self) -> Option<Self::Item> {
            panic!("panic in PanickingIterator!");
        }
    }

    #[test]
    fn lookahead_success_commit() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        guard.commit();

        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), Some("E"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn commit_with() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        guard.commit_with(["F", "P"]);

        assert_eq!(lookahead.next(), Some("F"));
        assert_eq!(lookahead.next(), Some("P"));
        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), Some("E"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn commit_with_but_panic_in_iter_into() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            guard.commit_with(PanickingIntoIterator::new());
        }));

        assert!(result.is_err());

        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
    }

    #[test]
    fn commit_returning_first() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        let first = guard.commit_returning_first(["F", "P"]).unwrap();
        assert_eq!(first, "F");

        assert_eq!(lookahead.next(), Some("P"));
        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), Some("E"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn commit_returning_first_but_peek_many_zero() {
        let source = vec!["A", "B"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<0>().unwrap();

        let first = guard.commit_returning_first(["F", "P"]).unwrap();
        assert_eq!(first, "F");

        assert_eq!(lookahead.next(), Some("P"));
        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn commit_returning_first_but_arg_zero() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        let first = guard.commit_returning_first([]);
        assert!(first.is_none());

        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), Some("E"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn commit_returning_first_but_panic_with_into_iter() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            guard.commit_returning_first(PanickingIntoIterator::new());
        }));

        assert!(result.is_err());

        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
    }

    #[test]
    fn commit_returning_first_but_panic_with_iter() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            guard.commit_returning_first(PanickingIterator::new());
        }));

        assert!(result.is_err());

        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
    }

    #[test]
    fn commit_with_peeked() {
        let source = vec!["A", "B", "C", "D"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<2>().unwrap();

        let tokens = guard.data();
        assert_eq!(tokens, &["A", "B"]);

        let first = guard.commit_with_peeked(|[token1, token2]| vec!["F", "X", token1, token2]);

        assert_eq!(first, Some("F"));

        assert_eq!(lookahead.next(), Some("X"));
        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
    }

    #[test]
    fn commit_with_peeked_panic_safety() {
        let source = vec!["A", "B", "C", "D"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<2>().unwrap();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            guard.commit_with_peeked(|_| {
                panic!("test panic");
            })
        }));

        assert!(result.is_err());

        // The buffer is valid, but elements A and B are lost.
        assert_eq!(lookahead.next(), Some("C"));
        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn commit_into() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");
        let cloned_tokens = *tokens;

        let into_tokens = guard.commit_into();
        assert_eq!(cloned_tokens, into_tokens);

        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), Some("E"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn lookahead_rollback_on_drop() {
        let source = vec!["A", "B", "C", "D"].into_iter();
        let mut lookahead = Lookahead::new(source);

        {
            let guard = lookahead.peek_many::<2>().unwrap();
            let tokens = guard.data();

            assert_eq!(tokens[0], "A");
            assert_eq!(tokens[1], "B");

            drop(guard);
        }

        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
        assert_eq!(lookahead.next(), Some("C"));
        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn lookahead_not_enough_elements() {
        let source = vec!["A", "B"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<4>();

        assert!(guard.is_none());
        drop(guard);

        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
        assert_eq!(lookahead.next(), None);
    }

    #[test]
    fn multiple_sequential_transactions() {
        let source = vec!["1", "2", "3", "4", "5"].into_iter();
        let mut lookahead = Lookahead::new(source);

        if let Some(guard) = lookahead.peek_many::<2>() {
            let items = guard.data();

            assert_eq!(items[0], "1");
            assert_eq!(items[1], "2");

            drop(guard);
        }

        if let Some(guard) = lookahead.peek_many::<3>() {
            assert_eq!(guard.data()[0], "1");
            assert_eq!(guard.data()[1], "2");
            assert_eq!(guard.data()[2], "3");

            guard.commit();
        }

        assert_eq!(lookahead.next(), Some("4"));
        assert_eq!(lookahead.next(), Some("5"));
    }

    #[test]
    fn lookahead_nested_or_sequential_bug() {
        let source = vec!["A", "B", "C", "D"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard1 = lookahead.peek_many::<2>().unwrap();
        assert_eq!(guard1.data(), &["A", "B"]);
        drop(guard1);

        let guard2 = lookahead.peek_many::<1>().unwrap();
        drop(guard2);

        assert_eq!(lookahead.next(), Some("A"));
        assert_eq!(lookahead.next(), Some("B"));
    }

    #[test]
    fn sequential_transactions_with_commit() {
        let source = vec!["A", "B", "C", "D"].into_iter();
        let mut lookahead = Lookahead::new(source);

        {
            let guard1 = lookahead.peek_many::<3>().unwrap();
            drop(guard1);
        }

        let guard2 = lookahead.peek_many::<1>().unwrap();

        guard2.commit();

        assert_eq!(lookahead.next(), Some("B"));
    }

    fn setup_lookahead_env(count: usize) -> (Lookahead<std::vec::IntoIter<Rc<i32>>>, Vec<Rc<i32>>) {
        let mut items = Vec::new();
        let mut watchers = Vec::new();

        for i in 0..count {
            let item = Rc::new(i32::try_from(i).unwrap() * 10);

            watchers.push(Rc::clone(&item));
            items.push(item);
        }

        (Lookahead::new(items.into_iter()), watchers)
    }

    #[test]
    fn rc_rollback_on_explicit_drop() {
        let (mut lookahead, watchers) = setup_lookahead_env(2);

        {
            let guard = lookahead.peek_many::<2>().unwrap();
            assert_eq!(guard.data()[0].as_ref(), &0);
            assert_eq!(guard.data()[1].as_ref(), &10);

            assert_eq!(Rc::strong_count(&watchers[0]), 2);
            assert_eq!(Rc::strong_count(&watchers[1]), 2);
        }

        // In buffer
        assert_eq!(Rc::strong_count(&watchers[0]), 2);
        assert_eq!(Rc::strong_count(&watchers[1]), 2);
    }

    #[test]
    fn rc_cleanup_on_successful_commit() {
        let (mut lookahead, watchers) = setup_lookahead_env(2);

        {
            let guard = lookahead.peek_many::<2>().unwrap();
            guard.commit();
        }

        // Buffer in clean
        assert_eq!(Rc::strong_count(&watchers[0]), 1);
        assert_eq!(Rc::strong_count(&watchers[1]), 1);
    }

    #[test]
    fn rc_panic_safety_during_transaction() {
        let (mut lookahead, watchers) = setup_lookahead_env(1);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _tx = lookahead.peek_many::<1>().unwrap();
            panic!("SHIT!");
        }));

        assert!(result.is_err());

        // In buffer
        assert_eq!(Rc::strong_count(&watchers[0]), 2);
    }

    #[test]
    fn rc_panic_commit_with() {
        let (mut lookahead, watchers) = setup_lookahead_env(1);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let guard = lookahead.peek_many::<1>().unwrap();

            let panic_iter = PanickingIntoIterator::new();
            guard.commit_with(panic_iter);
        }));

        assert!(result.is_err());

        // In buffer
        assert_eq!(Rc::strong_count(&watchers[0]), 2);
    }

    #[test]
    fn rc_panic_commit_returning_first() {
        let (mut lookahead, watchers) = setup_lookahead_env(1);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let guard = lookahead.peek_many::<1>().unwrap();

            let panic_iter = PanickingIntoIterator::new();
            guard.commit_returning_first(panic_iter);
        }));

        assert!(result.is_err());

        // In buffer
        assert_eq!(Rc::strong_count(&watchers[0]), 2);
    }

    #[test]
    fn rc_no_double_drop_after_iterator_exhaustion() {
        let (mut lookahead, watchers) = setup_lookahead_env(1);

        {
            let _tx = lookahead.peek_many::<1>().unwrap();
        }

        let item = lookahead.next().unwrap();

        assert_eq!(*item, 0);
        drop(item);

        assert_eq!(Rc::strong_count(&watchers[0]), 1);
    }
}
