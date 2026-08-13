use serde::{Serialize, Serializer};
use std::range::Range;

#[derive(Serialize)]
struct RangeRef<'a, T> {
    start: &'a T,
    end: &'a T,
}

pub fn serialize<T, S>(range: &Range<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    RangeRef {
        start: &range.start,
        end: &range.end,
    }
    .serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[derive(Serialize)]
    struct TestStruct<T>
    where
        T: Serialize,
    {
        #[serde(with = "self")]
        range: std::range::Range<T>,
    }

    impl<T> TestStruct<T>
    where
        T: Serialize,
    {
        pub fn new(start: T, end: T) -> Self {
            Self {
                range: std::range::Range { start, end },
            }
        }
    }

    proptest! {
        #[test]
        fn serializing(start: usize, end: usize) {
            let range = TestStruct::new(start, end);
            let json = serde_json::to_string(&range).unwrap();

            let result = format!(r#"{{"range":{{"start":{},"end":{}}}}}"#, start, end);
            assert_eq!(json, result);
        }
    }
}
