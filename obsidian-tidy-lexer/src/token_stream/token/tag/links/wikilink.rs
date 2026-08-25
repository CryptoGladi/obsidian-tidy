use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wikilink<'input> {
    destination: Cow<'input, str>,

    /// `true` if the wikilink was piped.
    ///
    /// * `true` - `[[foo|bar]]`
    /// * `false` - `[[foo]]`
    has_pothole: bool,
}

impl<'input> Wikilink<'input> {
    pub fn destination(&'input self) -> &'input str {
        self.destination.as_ref()
    }

    pub fn has_pothole(&self) -> bool {
        self.has_pothole
    }
}

crate::__private::impl_as_target_self!(Wikilink<'_>);
