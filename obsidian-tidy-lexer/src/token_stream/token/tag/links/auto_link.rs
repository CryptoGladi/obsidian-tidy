use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Autolink<'input> {
    pub(crate) url: Cow<'input, str>,
    pub(crate) is_email: bool,
}

impl<'input> Autolink<'input> {
    pub fn url(&'input self) -> &'input str {
        self.url.as_ref()
    }

    pub fn is_mail(&self) -> bool {
        self.is_email
    }
}

crate::__private::impl_as_target_self!(Autolink<'_>);

#[cfg(test)]
mod tests {}
