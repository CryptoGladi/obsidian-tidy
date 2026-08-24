use alloc::borrow::Cow;
use core::range::Range;
use derive_more::IsVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, IsVariant, Serialize, Deserialize)]
pub enum CalloutFoldable {
    /// `[!tip]+` — развёрнутый
    Expanded,

    /// `[!tip]-` — свёрнутый
    Collapsed,

    /// `[!tip]` — обычный
    #[default]
    None,
}

static_assertions::assert_impl_all!(CalloutFoldable: Copy, Clone);

impl From<char> for CalloutFoldable {
    fn from(value: char) -> Self {
        match value {
            '+' => CalloutFoldable::Expanded,
            '-' => CalloutFoldable::Collapsed,
            _ => CalloutFoldable::None,
        }
    }
}

/// Only for interceptor
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Callout<'input> {
    pub kind: Cow<'input, str>,

    #[serde(with = "crate::__private::range_serde")]
    pub header_offset: Range<usize>,

    pub foldable: CalloutFoldable,
}

impl Callout<'_> {
    #[must_use]
    pub fn kind(&self) -> &str {
        self.kind.as_ref()
    }

    #[must_use]
    pub const fn header_offset(&self) -> Range<usize> {
        self.header_offset
    }

    #[must_use]
    pub const fn foldable(&self) -> CalloutFoldable {
        self.foldable
    }
}
