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

#[derive(Debug, Clone, PartialEq, IsVariant, Serialize)]
pub enum Token<'input> {
    Markdown(pulldown_cmark::Event<'input>),

    StartCallout(Callout<'input>),
    EndCallout,
}

impl<'input> Token<'input> {
    pub fn as_markdown(&self) -> Option<&pulldown_cmark::Event<'input>> {
        if let Token::Markdown(markdown) = self {
            return Some(markdown);
        }

        None
    }

    pub fn as_start_callout(&self) -> Option<&Callout<'input>> {
        if let Token::StartCallout(callout) = self {
            return Some(callout);
        }

        None
    }
}
