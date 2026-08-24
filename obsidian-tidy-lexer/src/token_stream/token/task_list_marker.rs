#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaskListMarker {
    is_done: bool,
}

crate::__private::impl_as_target_self!(TaskListMarker);

impl TaskListMarker {
    #[must_use]
    pub const fn new(is_done: bool) -> Self {
        Self { is_done }
    }

    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.is_done
    }
}

// TODO make test
