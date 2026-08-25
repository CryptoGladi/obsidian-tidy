use core::range::Range;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize)]
struct RangeRef<'a, T> {
    start: &'a T,
    end: &'a T,
}

#[derive(Deserialize)]
struct RangeOwned<T> {
    start: T,
    end: T,
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

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Range<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let raw = RangeOwned::<T>::deserialize(deserializer)?;

    Ok(Range {
        start: raw.start,
        end: raw.end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde::de::DeserializeOwned;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct TestStruct<T>
    where
        T: Serialize + DeserializeOwned,
    {
        #[serde(with = "super")]
        range: core::range::Range<T>,
    }

    impl<T> TestStruct<T>
    where
        T: Serialize + DeserializeOwned,
    {
        pub fn new(start: T, end: T) -> Self {
            Self {
                range: core::range::Range { start, end },
            }
        }
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn serializing(start: usize, end: usize) {
            let range = TestStruct::new(start, end);
            let json = serde_json::to_string(&range).unwrap();

            let result = format!(r#"{{"range":{{"start":{start},"end":{end}}}}}"#);
            proptest::prop_assert_eq!(json, result);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn deserializing(start: usize, end: usize) {
            let range = TestStruct::new(start, end);
            let json = format!(r#"{{"range":{{"start":{start},"end":{end}}}}}"#);

            let result = serde_json::from_str(&json).unwrap();

            proptest::prop_assert_eq!(range, result);
        }
    }
}
