use derive_more::IsVariant;
use serde::Serialize;

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

pub struct Callout<'input> {
    kind: &'input str,
    foldable: CalloutFoldable,
}

pub enum Token<'input> {
    Markdown(pulldown_cmark::Event<'input>),
    Callout(Callout<'input>),
}
