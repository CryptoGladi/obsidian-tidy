use super::Handler;
use proc_macro2::Span;
use syn::Error;

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
            ..Default::default()
        }
    }
}

impl<S> Default for CheckLenString<S>
where
    S: AsRef<str>,
{
    fn default() -> Self {
        Self {
            error_message: "It is string is very long".to_string(),
            max_len: 30,
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

        if str.chars().count() > self.max_len {
            return Err(Error::new(span, self.error_message.clone()));
        }

        Ok(())
    }

    fn next(&self) -> Option<&Box<dyn Handler<Data = Self::Data>>> {
        self.next.as_ref()
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
        let handler = CheckLenString::default();
        handler.handle(&"", Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn very_long_string() {
        let handler = CheckLenString::default();
        let long_string: String = std::iter::repeat('A').take(100).collect();

        handler.handle(&long_string, Span::call_site()).unwrap();
    }

    #[test]
    fn check_unicode() {
        let mut handler = CheckLenString::default();
        handler.max_len = 5;

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
