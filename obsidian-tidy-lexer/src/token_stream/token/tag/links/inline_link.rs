use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InlineLink<'input> {
    destination: Cow<'input, str>,
    title: Option<Cow<'input, str>>,
}

impl<'input> InlineLink<'input> {
    pub fn destination(&'input self) -> &'input str {
        self.destination.as_ref()
    }

    pub fn title(&'input self) -> Option<&'input str> {
        self.title.as_ref().map(Cow::as_ref)
    }
}

crate::__private::impl_as_target_self!(InlineLink<'_>);
