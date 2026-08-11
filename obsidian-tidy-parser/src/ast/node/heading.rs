use super::Tag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Heading {
    level: HeadingLevel,
}

impl Heading {
    pub fn new(level: HeadingLevel) -> Self {
        Self { level }
    }
}

impl<'a> Tag<'a, Heading> {
    pub fn level(&self) -> HeadingLevel {
        self.kind.level
    }
}
