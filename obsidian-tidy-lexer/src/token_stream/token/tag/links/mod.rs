mod auto_link;
mod inline_link;
mod reference_link;
mod wiki_link;

pub use auto_link::Autolink;
pub use inline_link::InlineLink;
pub use reference_link::ReferenceLink;
pub use wiki_link::WikiLink;

use crate::__private::impl_enum;

impl_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Link<'input> {
        Inline(InlineLink<'input>),
        Reference(ReferenceLink<'input>),
        Autolink(Autolink<'input>),
        WikiLink(WikiLink<'input>),
    }
}

crate::__private::impl_as_target_self!(Link<'_>);
