#[cfg(feature = "serde")]
pub mod range_serde;

pub mod as_target;
pub mod impl_enum;

pub use as_target::*;
pub(crate) use impl_enum::impl_enum;
