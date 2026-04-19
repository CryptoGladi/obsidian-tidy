/// Extension trait for checking if a string is in kebab-case format.

#[cfg(test)]
use proptest::prelude::*;

pub trait IsKebabCase {
    fn is_kebab_case(&self) -> bool;
}

impl<S> IsKebabCase for S
where
    S: AsRef<str>,
{
    fn is_kebab_case(&self) -> bool {
        let value = self.as_ref();

        if value.starts_with('-') || value.ends_with('-') || value.contains("--") {
            return false;
        }

        value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}

#[cfg(test)]
pub fn kebab_strategy() -> impl Strategy<Value = String> {
    let valid_char = proptest::char::range('a', 'z').prop_union(proptest::char::range('0', '9'));

    let segment = proptest::collection::vec(valid_char, 1..10)
        .prop_map(|chars| chars.into_iter().collect::<String>());

    proptest::collection::vec(segment, 1..5).prop_map(|segments| segments.join("-"))
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn kebab_case(s in kebab_strategy()) {
            prop_assert!(s.is_kebab_case());
        }
    }

    #[test]
    fn invalid_kebab_case() {
        assert!(!"Hello-world".is_kebab_case());
        assert!(!"hello-World".is_kebab_case());
        assert!(!"hello_world".is_kebab_case());
        assert!(!"-hello-world".is_kebab_case());
        assert!(!"hello-world-".is_kebab_case());
        assert!(!"hello--world".is_kebab_case());
        assert!(!"hello world".is_kebab_case());
        assert!(!"hello@world".is_kebab_case());
        assert!(!"русский-язык".is_kebab_case());
    }
}
