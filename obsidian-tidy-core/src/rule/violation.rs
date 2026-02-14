use std::ops::{Bound, Range, RangeBounds};
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Violation {
    message: String,
    location: Range<usize>,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    #[error("Location must have a start bound")]
    UnboundedStart,

    #[error("Location must have a end bound")]
    UnboundedEnd,

    #[error("Invalid range: start ({start}) must be <= end ({end})")]
    InvalidRange { start: usize, end: usize },
}

impl Violation {
    #[instrument(skip_all, err)]
    pub fn new(
        message: impl Into<String>,
        location: impl RangeBounds<usize>,
    ) -> Result<Self, Error> {
        let start = match location.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => return Err(Error::UnboundedStart),
        };

        let end = match location.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => return Err(Error::UnboundedEnd),
        };

        if start > end {
            return Err(Error::InvalidRange { start, end });
        }

        Ok(Self {
            message: message.into(),
            location: start..end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        let violation = Violation::new("Super error", 43..50).unwrap();

        assert_eq!(
            violation,
            Violation {
                message: "Super error".to_string(),
                location: 43..50
            }
        )
    }

    #[test]
    #[traced_test]
    fn new_with_inclusive() {
        let violation = Violation::new("Super error", 43..=50).unwrap();
        let violation1 = Violation::new("Super error", 43..=43).unwrap();

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
    #[traced_test]
    fn new_with_unbounded_start() {
        let result = Violation::new("Super error", ..50);
        assert_eq!(result, Err(Error::UnboundedStart));
    }

    #[test]
    #[traced_test]
    fn new_with_unbounded_end() {
        let result = Violation::new("Super error", 20..);
        assert_eq!(result, Err(Error::UnboundedEnd));
    }

    #[test]
    #[traced_test]
    fn new_with_invalid_range() {
        let result = Violation::new("Super error", 50..20);
        assert_eq!(result, Err(Error::InvalidRange { start: 50, end: 20 }));
    }
}
