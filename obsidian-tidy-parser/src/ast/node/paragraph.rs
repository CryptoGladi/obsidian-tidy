use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Paragraph;

impl Paragraph {
    pub fn new() -> Self {
        Self
    }
}
