use heck::ToKebabCase;

/// Extension trait for checking if a string is in kebab-case format.
pub trait IsKebabCase {
    fn is_kebab_case(&self) -> bool;
}

impl<S> IsKebabCase for S
where
    S: AsRef<str>,
{
    fn is_kebab_case(&self) -> bool {
        let value = self.as_ref();

        value.to_kebab_case() == value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kebab_case() {
        assert!("hello-world".is_kebab_case());
        assert!("hello-world-123".is_kebab_case());
        assert!("a".is_kebab_case());
        assert!("hello-world-test".is_kebab_case());

        assert!(!"Hello-world".is_kebab_case());
        assert!(!"hello-World".is_kebab_case());
        assert!(!"hello_world".is_kebab_case());
        assert!(!"-hello-world".is_kebab_case());
        assert!(!"hello-world-".is_kebab_case());
        assert!(!"hello--world".is_kebab_case());
        assert!(!"hello world".is_kebab_case());
        assert!(!"hello@world".is_kebab_case());
    }
}
