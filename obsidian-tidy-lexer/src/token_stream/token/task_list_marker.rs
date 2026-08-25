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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{InterceptorEnum, TokenStreamBuilder, TracingTokenStreamExt};

    fn collect_task_markers(source: &str) -> Vec<TaskListMarker> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
            .filter_map(|(token, _)| token.into_task_list_marker())
            .collect()
    }

    #[test]
    fn task_list_marker_done() {
        let source = "- [x] Completed task";
        let markers = collect_task_markers(source);

        assert_eq!(markers.len(), 1);
        assert!(markers[0].is_done(), "Marker should be marked as done");
    }

    #[test]
    fn task_list_marker_not_done() {
        let source = "- [ ] Pending task";
        let markers = collect_task_markers(source);

        assert_eq!(markers.len(), 1);
        assert!(!markers[0].is_done(), "Marker should not be marked as done");
    }

    #[test]
    fn list_with_formating() {
        let source = "- [ ] Pending **super** `task`";
        let markers = collect_task_markers(source);

        assert_eq!(markers.len(), 1);
        assert!(!markers[0].is_done(), "Marker should not be marked as done");
    }

    #[test]
    fn no_task_markers_in_regular_list() {
        let source = "- Regular item\n- Another item";
        let markers = collect_task_markers(source);

        assert!(
            markers.is_empty(),
            "Regular lists should not produce task markers"
        );
    }
}
