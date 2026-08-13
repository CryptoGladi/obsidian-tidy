use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Root;

impl Root {
    pub fn new() -> Self {
        Self
    }
}
