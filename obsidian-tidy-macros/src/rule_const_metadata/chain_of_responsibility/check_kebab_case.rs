use super::Handler;
use crate::rule_const_metadata::kebab_case::IsKebabCase;
use proc_macro2::Span;
use syn::Error;

pub struct CheckKebabCase<S>
where
    S: IsKebabCase,
{
    next: Option<Box<dyn Handler<Data = S>>>,
    error_message: String,
}

impl<S> CheckKebabCase<S>
where
    S: IsKebabCase,
{
    pub fn new(error_message: impl Into<String>) -> Self {
        Self {
            error_message: error_message.into(),
            ..Default::default()
        }
    }
}

impl<S> Default for CheckKebabCase<S>
where
    S: IsKebabCase,
{
    fn default() -> Self {
        Self {
            error_message: "It is string must be in kebab-case".to_string(),
            next: None,
        }
    }
}

impl<S> Handler for CheckKebabCase<S>
where
    S: IsKebabCase,
{
    type Data = S;

    fn handle(&self, data: &Self::Data, span: Span) -> syn::Result<()> {
        if !data.is_kebab_case() {
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
    #[should_panic]

    fn non_kebab_case() {
        let handler = CheckKebabCase::default();
        handler.handle(&"Muds_sdc", Span::call_site()).unwrap();
    }

    #[test]
    fn kebab_case() {
        let handler = CheckKebabCase::default();
        handler.handle(&"super-data", Span::call_site()).unwrap();
    }

    #[test]
    fn custom_error_message() {
        const ERROR_MESSAGE: &'static str = "My error message";

        let handler = CheckKebabCase::new(ERROR_MESSAGE);
        let error = handler.handle(&"SD_ds", Span::call_site()).err().unwrap();

        assert_eq!(error.to_string(), ERROR_MESSAGE)
    }
}
