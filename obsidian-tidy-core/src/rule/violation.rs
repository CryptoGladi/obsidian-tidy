use serde::{Deserialize, Serialize};
use std::ops::{Bound, Range, RangeBounds};
use thiserror::Error;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Violation {
    message: String,
    location: Range<usize>,
}

impl Violation {
    pub fn new(message: impl Into<String>, location: impl RangeBounds<usize>) -> Self {
        let start = match location.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s.saturating_add(1),
            Bound::Unbounded => 0,
        };

        let end = match location.end_bound() {
            Bound::Included(&e) => e.saturating_add(1),
            Bound::Excluded(&e) => e,
            Bound::Unbounded => usize::MAX,
        };

        if start > end {
            return Self {
                message: message.into(),
                location: start..(start.saturating_add(1)),
            };
        }

        Self {
            message: message.into(),
            location: start..end,
        }
    }

    #[inline]
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[inline]
    #[must_use]
    pub const fn location(&self) -> &Range<usize> {
        &self.location
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let violation = Violation::new("Super error", 43..50);

        assert_eq!(
            violation,
            Violation {
                message: "Super error".to_string(),
                location: 43..50
            }
        );
    }

    #[test]
    fn new_with_inclusive() {
        let violation = Violation::new("Super error", 43..=50);
        let violation1 = Violation::new("Super error", 43..=43);

        assert_eq!(
            violation,
            Violation {
                message: "Super error".to_string(),
                location: 43..51
            }
        );

        assert_eq!(
            violation1,
            Violation {
                message: "Super error".to_string(),
                location: 43..44
            }
        );
    }

    #[test]
    fn new_with_unbounded_start() {
        let result = Violation::new("Super error", ..50);

        assert_eq!(result.location.start, 0);
    }

    #[test]
    fn new_with_unbounded_end() {
        let result = Violation::new("Super error", 20..);
        assert_eq!(result.location.end, usize::MAX);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn new_with_invalid_range() {
        let result = Violation::new("Super error", 50..20);

        assert_eq!(result, Violation::new("Super error", 50..=50));
    }

    #[test]
    fn new_all_file() {
        let result = Violation::new("Super error", ..);

        assert_eq!(result.location.start, 0);
        assert_eq!(result.location.end, usize::MAX);
    }

    #[test]
    fn overflow_check() {
        // It is usize::MAX + 1!
        let result = Violation::new("Super error", 50..usize::MAX);

        assert_eq!(result.location.end, usize::MAX);
    }
}
