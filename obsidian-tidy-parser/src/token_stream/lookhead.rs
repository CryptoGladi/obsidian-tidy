use std::collections::VecDeque;
use std::mem::{ManuallyDrop, MaybeUninit};

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
    pub fn new(data: [MaybeUninit<I::Item>; N], lookahead: &'guard mut Lookahead<I>) -> Self {
        Self { data, lookahead }
    }

    pub fn data(&self) -> &[I::Item; N] {
        // SAFETY:
        // - MaybeUninit<T> has the same layout and alignment as T
        // - All N elements are guaranteed initialized by peek_many
        // - The reference is valid for the lifetime of self
        unsafe { &*(self.data.as_ptr() as *const [I::Item; N]) }
    }

    /// Commits the transaction and advances the buffer with new items.
    ///
    /// The original N tokens are **dropped**.
    /// New items are appended to the buffer in the same order.
    pub fn commit_with<T>(mut self, items: T)
    where
        T: IntoIterator<Item = I::Item>,
        T::IntoIter: DoubleEndedIterator,
    {
        let mut this = ManuallyDrop::new(self);
        let items = items.into_iter(); // If panic, then call drop

        for item in &mut this.data {
            // SAFETY: all elements are initialized
            unsafe {
                item.assume_init_drop();
            }
        }

        for item in items.rev() {
            this.lookahead.buffer.push_front(item);
        }
    }

    pub fn commit(mut self) {
        self.commit_with([]);
    }

    pub fn commit_into(self) -> [I::Item; N] {
        let mut this = ManuallyDrop::new(self);

        unsafe { std::ptr::read(this.data.as_ptr() as *const [I::Item; N]) }
    }
}

impl<I, const N: usize> Drop for LookaheadGuard<'_, I, N>
where
    I: Iterator,
{
    fn drop(&mut self) {
        for i in (0..N).rev() {
            unsafe {
                // SAFETY: all elements are initialized
                let item =
                    std::mem::replace(&mut self.data[i], MaybeUninit::uninit()).assume_init();

                self.lookahead.buffer.push_front(item);
            }
        }
    }
}

/// A transactional lookahead buffer for iterators.
///
/// Allows peeking N elements ahead without consuming them.
/// If the transaction is not explicitly committed via [`LookaheadGuard::commit`],
/// the peeked elements are automatically rolled back when the guard is dropped.
pub struct Lookahead<I>
where
    I: Iterator,
{
    inner: I,
    buffer: VecDeque<I::Item>,
}

impl<I> Lookahead<I>
where
    I: Iterator,
{
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            buffer: VecDeque::new(),
        }
    }

    pub fn peek<'guard>(&'guard mut self) -> Option<LookaheadGuard<'guard, I, 1>> {
        self.peek_many::<1>()
    }

    pub fn peek_many<'guard, const N: usize>(
        &'guard mut self,
    ) -> Option<LookaheadGuard<'guard, I, N>> {
        let mut data: [MaybeUninit<I::Item>; N] = std::array::from_fn(|_| MaybeUninit::uninit());

        for i in 0..N {
            if let Some(item) = self.buffer.pop_front() {
                data[i] = MaybeUninit::new(item);
            } else if let Some(item) = self.inner.next() {
                data[i] = MaybeUninit::new(item);
            } else {
                for j in (0..i).rev() {
                    // SAFETY: all elements are initialized
                    unsafe {
                        let item =
                            std::mem::replace(&mut data[j], MaybeUninit::uninit()).assume_init();

                        self.buffer.push_front(item);
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
        if let Some(item) = self.buffer.pop_front() {
            Some(item)
        } else {
            self.inner.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    struct PanickingIterable<T>(std::marker::PhantomData<T>);

    impl<T> PanickingIterable<T> {
        fn new() -> Self {
            Self(std::marker::PhantomData)
        }
    }

    impl<T> IntoIterator for PanickingIterable<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;

        #[track_caller]
        fn into_iter(self) -> Self::IntoIter {
            panic!("panic in PanickingIterable()!");
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
    fn commit_with_but_panic() {
        let source = vec!["A", "B", "C", "D", "E"].into_iter();
        let mut lookahead = Lookahead::new(source);

        let guard = lookahead.peek_many::<3>().unwrap();
        let tokens = guard.data();

        assert_eq!(tokens[0], "A");
        assert_eq!(tokens[1], "B");
        assert_eq!(tokens[2], "C");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            guard.commit_with(PanickingIterable::new());
        }));

        assert!(result.is_err());

        assert_eq!(lookahead.next(), Some("D"));
        assert_eq!(lookahead.next(), Some("E"));
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
        let cloned_tokens = tokens.clone();

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
            drop(guard1)
        }

        let guard2 = lookahead.peek_many::<1>().unwrap();

        guard2.commit();

        assert_eq!(lookahead.next(), Some("B"));
    }

    fn setup_lookahead_env(count: usize) -> (Lookahead<std::vec::IntoIter<Rc<i32>>>, Vec<Rc<i32>>) {
        let mut items = Vec::new();
        let mut watchers = Vec::new();

        for i in 0..count {
            let item = Rc::new(i as i32 * 10);

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

            let panic_iter = PanickingIterable::new();
            guard.commit_with(panic_iter);
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
