use derive_more::IsVariant;
use serde::Serialize;
use std::borrow::Cow;
use std::range::Range;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, IsVariant, Serialize)]
pub enum CalloutFoldable {
    /// `[!tip]+` — развёрнутый
    Expanded,

    /// `[!tip]-` — свёрнутый
    Collapsed,

    /// `[!tip]` — обычный
    #[default]
    None,
}

static_assertions::assert_impl_all!(CalloutFoldable: Copy);

impl From<char> for CalloutFoldable {
    fn from(value: char) -> Self {
        match value {
            '+' => CalloutFoldable::Expanded,
            '-' => CalloutFoldable::Collapsed,
            _ => CalloutFoldable::None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Callout<'input> {
    pub kind: Cow<'input, str>,

    #[serde(with = "crate::__private::range_serde")]
    pub header_offset: Range<usize>,

    pub foldable: CalloutFoldable,
}
