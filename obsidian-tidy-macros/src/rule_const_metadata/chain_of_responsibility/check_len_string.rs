use super::Handler;
use proc_macro2::Span;
use syn::Error;

/// Handler that checks if a string's length (in characters) does not exceed a maximum.
pub struct CheckLenString<S>
where
    S: AsRef<str>,
{
    next: Option<Box<dyn Handler<Data = S>>>,
    error_message: String,
    max_len: usize,
}

impl<S> CheckLenString<S>
where
    S: AsRef<str>,
{
    pub fn new(error_message: impl Into<String>, max_len: usize) -> Self {
        Self {
            error_message: error_message.into(),
            max_len,
            next: None,
        }
    }

    #[allow(unused, reason = "This function is needed for unit tests")]
    pub fn with_default_message(max_len: usize) -> Self {
        Self {
            error_message: "This string has too many characters".to_string(),
            max_len,
            next: None,
        }
    }
}

impl<S> Handler for CheckLenString<S>
where
    S: AsRef<str>,
{
    type Data = S;

    fn handle(&self, data: &Self::Data, span: Span) -> syn::Result<()> {
        let str = data.as_ref();

        // Don't use str.len()
        if str.chars().count() > self.max_len {
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

    fn empty_string() {
        let handler = CheckLenString::with_default_message(30);
        handler.handle(&"", Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn very_long_string() {
        let handler = CheckLenString::with_default_message(30);
        let long_string: String = std::iter::repeat('A').take(100).collect();

        handler.handle(&long_string, Span::call_site()).unwrap();
    }

    #[test]
    fn check_unicode() {
        let handler = CheckLenString::with_default_message(5);

        handler.handle(&"こんにちは", Span::call_site()).unwrap();
    }

    #[test]
    fn custom_error_message() {
        const ERROR_MESSAGE: &'static str = "My error message";

        let handler = CheckLenString::new(ERROR_MESSAGE, 30);
        let long_string: String = std::iter::repeat('A').take(100).collect();

        let error = handler
            .handle(&long_string, Span::call_site())
            .err()
            .unwrap();

        assert_eq!(error.to_string(), ERROR_MESSAGE)
    }
}
