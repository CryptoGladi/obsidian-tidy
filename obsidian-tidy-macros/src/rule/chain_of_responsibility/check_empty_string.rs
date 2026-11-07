use super::Handler;
use proc_macro2::Span;
use syn::Error;

/// Handler that checks if a string is empty (or only whitespace).
pub struct CheckEmptyString<S>
where
    S: AsRef<str>,
{
    next: Option<Box<dyn Handler<Data = S>>>,
    error_message: String,
}

impl<S> CheckEmptyString<S>
where
    S: AsRef<str>,
{
    pub fn new(error_message: impl Into<String>) -> Self {
        Self {
            error_message: error_message.into(),
            ..Default::default()
        }
    }
}

impl<S> Default for CheckEmptyString<S>
where
    S: AsRef<str>,
{
    fn default() -> Self {
        Self {
            error_message: "It is string can't be empty string".to_string(),
            next: None,
        }
    }
}

impl<S> Handler for CheckEmptyString<S>
where
    S: AsRef<str>,
{
    type Data = S;

    fn handle(&self, data: &Self::Data, span: Span) -> syn::Result<()> {
        let str = data.as_ref();

        if str.trim().is_empty() {
            return Err(Error::new(span, self.error_message.clone()));
        }

        Ok(())
    }

    fn next(&self) -> Option<&dyn Handler<Data = Self::Data>> {
        self.next.as_deref()
    }

    fn set_next(&mut self, next: Box<dyn Handler<Data = Self::Data>>) {
        self.next = Some(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]

    fn empty_string() {
        let handler = CheckEmptyString::default();
        handler.handle(&"", Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn whitespace_string() {
        let handler = CheckEmptyString::default();
        handler.handle(&"     ", Span::call_site()).unwrap();
    }

    #[test]
    fn not_empty_string() {
        let handler = CheckEmptyString::default();
        handler.handle(&"data", Span::call_site()).unwrap();
    }

    #[test]
    fn custom_error_message() {
        const ERROR_MESSAGE: &'static str = "My error message";

        let handler = CheckEmptyString::new(ERROR_MESSAGE);
        let error = handler.handle(&"", Span::call_site()).err().unwrap();

        assert_eq!(error.to_string(), ERROR_MESSAGE)
    }
}
