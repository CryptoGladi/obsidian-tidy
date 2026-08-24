//! This module provides unified traits for extracting target types from both
//! smart pointers (containers) and primitive types.
//!
//! # Why these traits are needed
//! Standard Rust macros (`macro_rules!`) operate purely at the token level and
//! cannot inspect or branch based on types during expansion. To allow a macro to
//! handle different types seamlessly, we must offload type resolution to the
//! compiler's trait system using **Associated Types**.
//!
//! By having exactly one `type Target` per implementing type, the compiler can
//! uniquely infer the resulting type without manual type hints in the macro expansion.
//!
//! # Why we cannot use [`core::ops::Deref`]
//! While `Deref` works perfectly for containers like `Box<T>`, `Rc<T>`, and `Arc<T>`,
//! it is not implemented for primitive types (e.g., `i32`, `bool`, `char`) in the
//! standard library.
//!
//! Attempting a blanket implementation like `impl<T: Deref> AsRefTarget for T`
//! combined with explicit impls for primitives triggers Rust's orphan rules and
//! overlapping implementation errors (e.g., "upstream crates may add a new impl
//! of trait [`core::ops::Deref`] for type `i32` in future versions").
//! Explicitly implementing our own trait for both containers and primitives is
//! the only stable way to bypass this restriction.
//!
//! # Why we cannot use [`core::convert::AsRef`] / [`AsMut`]
//! Standard `AsRef<T>` and `AsMut<T>` use a generic type parameter `T` instead of
//! an associated type. This allows a single type to implement `AsRef` for multiple
//! different targets (for example, a custom type could implement both `AsRef<str>`
//! and `AsRef<[u8]>`).
//!
//! Because of this multi-mapping capability, a declarative macro cannot unambiguously
//! deduce the target type without explicit type annotations from the user, breaking
//! the ergonomics and automation of the macro.

use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

/// Unified trait for extracting an immutable reference to the target type.
///
/// See module-level documentation for details on why this is used instead of [`Deref`] or [`AsRef`].
///
/// [`Deref`]: `core::ops::Deref`
/// [`AsRef`]: `core::convert::AsRef`
pub trait AsRefTarget {
    /// The resolved inner or underlying type.
    type Target: ?Sized;

    /// Returns an immutable reference to the target type.
    fn as_target_ref(&self) -> &Self::Target;
}

impl<B: ?Sized + ToOwned> AsRefTarget for Cow<'_, B> {
    type Target = B;

    fn as_target_ref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<B: ?Sized> AsRefTarget for Box<B> {
    type Target = B;

    fn as_target_ref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<B: ?Sized> AsRefTarget for Rc<B> {
    type Target = B;

    fn as_target_ref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<B: ?Sized> AsRefTarget for Arc<B> {
    type Target = B;

    fn as_target_ref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRefTarget for String {
    type Target = str;

    fn as_target_ref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<T> AsRefTarget for Vec<T> {
    type Target = [T];

    fn as_target_ref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'b, T: ?Sized> AsRefTarget for &'b T {
    type Target = T;

    #[inline]
    fn as_target_ref(&self) -> &Self::Target {
        *self
    }
}

impl<T: ?Sized> AsRefTarget for &mut T {
    type Target = T;

    #[inline]
    fn as_target_ref(&self) -> &Self::Target {
        self
    }
}

macro_rules! impl_as_ref_target_self {
    ($($t:ty),* $(,)?) => {
        $(
            impl $crate::__private::AsRefTarget for $t {
                type Target = Self;

                #[inline]
                fn as_target_ref(&self) -> &Self::Target { self }
            }
        )*
    };
}

pub(crate) use impl_as_ref_target_self;

impl_as_ref_target_self!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, char, f32, f64
);

/// Unified trait for extracting a mutable reference to the target type.
///
/// See module-level documentation for details on why this is used instead of [`DerefMut`] or [`AsMut`].
///
/// [`DerefMut`]: `core::ops::DerefMut`
/// [`AsRef`]: `core::convert::AsRef`
pub trait AsMutTarget {
    /// The resolved inner or underlying type.
    type Target: ?Sized;

    /// Returns a mutable reference to the target type.
    fn as_target_mut(&mut self) -> &mut Self::Target;
}

impl<B: ?Sized + ToOwned> AsMutTarget for Cow<'_, B> {
    type Target = B::Owned;

    fn as_target_mut(&mut self) -> &mut Self::Target {
        self.to_mut()
    }
}

impl<B: ?Sized> AsMutTarget for Box<B> {
    type Target = B;

    fn as_target_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<B> AsMutTarget for Rc<B>
where
    B: Clone,
{
    type Target = B;

    fn as_target_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(self)
    }
}

impl<B> AsMutTarget for Arc<B>
where
    B: Clone,
{
    type Target = B;

    fn as_target_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(self)
    }
}

impl AsMutTarget for String {
    type Target = str;

    #[inline]
    fn as_target_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}

impl<T> AsMutTarget for Vec<T> {
    type Target = [T];

    fn as_target_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: ?Sized> AsMutTarget for &mut T {
    type Target = T;

    #[inline]
    fn as_target_mut(&mut self) -> &mut Self::Target {
        *self
    }
}

macro_rules! impl_as_mut_target_self {
    ($($t:ty),* $(,)?) => {
        $(
            impl $crate::__private::AsMutTarget for $t {
                type Target = Self;

                #[inline]
                fn as_target_mut(&mut self) -> &mut Self::Target { self }
            }
        )*
    };
}

pub(crate) use impl_as_mut_target_self;

impl_as_mut_target_self!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, char, f32, f64
);

macro_rules! impl_as_target_self {
    ($($t:ty),* $(,)?) => {
        $crate::__private::impl_as_ref_target_self!($($t),*);
        $crate::__private::impl_as_mut_target_self!($($t),*);
    };
}

pub(crate) use impl_as_target_self;

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_type_eq_all!(<Cow<'static, str> as AsRefTarget>::Target, str);
    static_assertions::assert_type_eq_all!(<String as AsRefTarget>::Target, str);
    static_assertions::assert_type_eq_all!(<i32 as AsRefTarget>::Target, i32);
    static_assertions::assert_type_eq_all!(<&'static str as AsRefTarget>::Target, str);
    static_assertions::assert_type_eq_all!(<&'static mut String as AsRefTarget>::Target, String);

    static_assertions::assert_type_eq_all!(<Cow<'static, str> as AsMutTarget>::Target, String);
    static_assertions::assert_type_eq_all!(<String as AsMutTarget>::Target, str);
    static_assertions::assert_type_eq_all!(<i32 as AsMutTarget>::Target, i32);
    static_assertions::assert_type_eq_all!(<&'static mut String as AsMutTarget>::Target, String);

    // === Unit tests for AsRefTarget ===

    #[test]
    fn cow_borrowed() {
        let cow: Cow<'_, str> = Cow::Borrowed("hello");
        let target: &str = cow.as_target_ref();
        assert_eq!(target, "hello");
    }

    #[test]
    fn cow_owned() {
        let cow: Cow<'_, str> = Cow::Owned(String::from("world"));
        let target: &str = cow.as_target_ref();
        assert_eq!(target, "world");
    }

    #[test]
    fn cow_slice() {
        let cow: Cow<'_, [i32]> = Cow::Borrowed(&[1, 2, 3]);
        let target: &[i32] = cow.as_target_ref();
        assert_eq!(target, &[1, 2, 3]);
    }

    #[test]
    fn box_str() {
        let boxed: Box<str> = String::from("test").into_boxed_str();
        let target: &str = boxed.as_target_ref();
        assert_eq!(target, "test");
    }

    #[test]
    fn box_slice() {
        let boxed: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
        let target: &[i32] = boxed.as_target_ref();
        assert_eq!(target, &[1, 2, 3]);
    }

    #[test]
    fn rc_str() {
        let rc: Rc<str> = Rc::from("rc test");
        let target: &str = rc.as_target_ref();
        assert_eq!(target, "rc test");
    }

    #[test]
    fn arc_str() {
        let arc: Arc<str> = Arc::from("arc test");
        let target: &str = arc.as_target_ref();
        assert_eq!(target, "arc test");
    }

    #[test]
    fn string() {
        let string = String::from("string test");
        let target: &str = string.as_target_ref();
        assert_eq!(target, "string test");
    }

    #[test]
    fn reference_str() {
        let s = "reference";
        let r: &str = s;
        let target: &str = r.as_target_ref();
        assert_eq!(target, "reference");
    }

    #[test]
    fn mut_reference_str() {
        let mut s = String::from("mutable");
        let r: &mut String = &mut s;
        let target: &String = r.as_target_ref();
        assert_eq!(target, "mutable");
    }

    #[test]
    fn primitives() {
        assert_eq!(42i32.as_target_ref(), &42i32);
        assert_eq!(100u64.as_target_ref(), &100u64);
        assert_eq!(true.as_target_ref(), &true);
        assert_eq!('a'.as_target_ref(), &'a');
        assert_eq!(3.14f64.as_target_ref(), &3.14f64);
    }

    // === Unit tests for AsMutTarget ===

    #[test]
    fn cow_borrowed_to_mut() {
        let mut cow: Cow<'_, str> = Cow::Borrowed("hello");
        let target: &mut String = cow.as_target_mut();
        target.push_str(" world");
        assert_eq!(cow, "hello world");
    }

    #[test]
    fn cow_owned_to_mut() {
        let mut cow: Cow<'_, str> = Cow::Owned(String::from("world"));
        let target: &mut String = cow.as_target_mut();
        target.push_str("!");
        assert_eq!(cow, "world!");
    }

    #[test]
    fn box_str_to_mut() {
        let mut boxed: Box<str> = String::from("test").into_boxed_str();
        let target: &mut str = boxed.as_target_mut();
        target.make_ascii_uppercase();
        assert_eq!(&*boxed, "TEST");
    }

    #[test]
    fn rc_str_to_mut() {
        let mut rc: Rc<str> = Rc::from("rc test");
        let target: &mut str = Rc::make_mut(&mut rc);
        target.make_ascii_uppercase();
        assert_eq!(&*rc, "RC TEST");
    }

    #[test]
    fn arc_str_to_mut() {
        let mut arc: Arc<str> = Arc::from("arc test");
        let target: &mut str = Arc::make_mut(&mut arc);
        target.make_ascii_uppercase();
        assert_eq!(&*arc, "ARC TEST");
    }

    #[test]
    fn primitives_to_mut() {
        let mut i = 42i32;
        *i.as_target_mut() = 100;
        assert_eq!(i, 100);

        let mut b = false;
        *b.as_target_mut() = true;
        assert!(b);
    }

    // === Edge cases ===

    #[test]
    fn empty_string() {
        let empty = String::new();
        assert_eq!(empty.as_target_ref(), "");
    }

    #[test]
    fn empty_slice() {
        let empty: &[i32] = &[];
        assert!(empty.as_target_ref().is_empty());
    }

    #[test]
    fn unicode_string() {
        let unicode = String::from("Привет мир 你好");
        assert_eq!(unicode.as_target_ref(), "Привет мир 你好");
    }

    #[test]
    fn cow_conversion_on_mut() {
        // Test that Cow::Borrowed converts to Cow::Owned when mutated
        let original = String::from("original");
        let mut cow: Cow<'_, str> = Cow::Borrowed(&original);

        let target = cow.as_target_mut();
        target.push_str(" modified");

        // Original should be unchanged
        assert_eq!(original, "original");
        // Cow should now be owned
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(cow, "original modified");
    }

    #[test]
    fn rc_clone_shares_data() {
        let rc1: Rc<str> = Rc::from("shared");
        let rc2 = Rc::clone(&rc1);

        assert_eq!(rc1.as_target_ref(), rc2.as_target_ref());
        assert_eq!(Rc::strong_count(&rc1), 2);
    }

    #[test]
    fn arc_clone_shares_data() {
        let arc1: Arc<str> = Arc::from("shared");
        let arc2 = Arc::clone(&arc1);

        assert_eq!(arc1.as_target_ref(), arc2.as_target_ref());
        assert_eq!(Arc::strong_count(&arc1), 2);
    }
}
