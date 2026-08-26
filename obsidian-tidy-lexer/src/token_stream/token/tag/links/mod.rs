mod autolink;
mod inline_link;
mod reference_link;
mod wiki_link;

pub use autolink::{Autolink, AutolinkKind};
pub use inline_link::InlineLink;
pub use reference_link::ReferenceLink;
pub use wiki_link::WikiLink;

use crate::__private::impl_enum;

impl_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(bound(deserialize = "'input: 'de")))]
    pub enum Link<'input> {
        Inline(InlineLink<'input>),
        Reference(ReferenceLink<'input>),
        Autolink(Autolink<'input>),
        WikiLink(WikiLink<'input>),
    }
}

crate::__private::impl_as_target_self!(Link<'_>);
