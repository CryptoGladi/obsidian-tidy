pub mod check_empty_string;
pub mod check_kebab_case;
pub mod check_len_string;
pub mod check_only_ascii;
pub mod handlers;

use proc_macro2::Span;

pub use check_empty_string::CheckEmptyString;
pub use check_kebab_case::CheckKebabCase;
pub use check_len_string::CheckLenString;
pub use check_only_ascii::CheckOnlyAscii;

pub trait Handler {
    type Data;

    fn handle(&self, data: &Self::Data, span: Span) -> syn::Result<()>;

    fn set_next(&mut self, next: Box<dyn Handler<Data = Self::Data>>);
    fn next(&self) -> Option<&Box<dyn Handler<Data = Self::Data>>>;
}

pub fn run_chain<D>(handler: &dyn Handler<Data = D>, data: &D, span: Span) -> syn::Result<()> {
    let mut current = Some(handler);
    let mut errors = Vec::new();

    while let Some(h) = current {
        if let Err(error) = h.handle(data, span) {
            errors.push(error);
        }

        current = h.next().map(|b| b.as_ref());
    }

    if !errors.is_empty() {
        let mut combined_error = errors.remove(0);
        for error in errors {
            combined_error.combine(error);
        }
        return Err(combined_error);
    }

    Ok(())
}
