use crate::prelude::Node;
use crate::token_stream::token::Tag;
use alloc::vec::Vec;
use core::range::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame<'input> {
    pub tag: Tag<'input>,
    pub offset: Range<usize>,
    children: Vec<Node<'input>>,
}

impl<'input> Frame<'input> {
    #[must_use]
    pub const fn new(tag: Tag<'input>, offset: Range<usize>, children: Vec<Node<'input>>) -> Self {
        Self {
            tag,
            offset,
            children,
        }
    }

    #[must_use]
    pub fn children(&self) -> &[Node<'input>] {
        &self.children
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stack<'a> {
    root: Vec<Node<'a>>,
    frames: Vec<Frame<'a>>,
}

impl<'a> Stack<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            root: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn push(&mut self, frame: Frame<'a>) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Option<Frame<'a>> {
        self.frames.pop()
    }

    #[must_use]
    pub const fn count_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn push_parent(&mut self, node: Node<'a>) {
        match self.frames.last_mut() {
            Some(parent) => parent.children.push(node),
            None => self.root.push(node),
        }
    }

    #[must_use]
    pub fn into_root(self) -> Option<Vec<Node<'a>>> {
        self.frames.is_empty().then_some(self.root)
    }

    pub fn into_frames(self) -> impl Iterator<Item = Frame<'a>> {
        self.frames.into_iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::NodeKind;

    const OFFSET: Range<usize> = Range { start: 1, end: 3 };

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::default();

        assert!(stack.pop().is_none());

        let frame = Frame::new(Tag::Paragraph, OFFSET, Vec::new());
        stack.push(frame.clone());

        assert_eq!(stack.count_frames(), 1);

        let popped_frame = stack.pop().unwrap();
        assert_eq!(frame, popped_frame);

        assert!(stack.pop().is_none());
    }

    #[test]
    fn check_lifo() {
        let mut stack = Stack::default();

        let frame1 = Frame::new(Tag::Paragraph, OFFSET, Vec::new());
        let frame2 = Frame::new(Tag::Emphasis, OFFSET, Vec::new());
        let frame3 = Frame::new(Tag::HtmlBlock, OFFSET, Vec::new());

        for frame in [&frame1, &frame2, &frame3] {
            stack.push(frame.clone());
        }

        let popped_frame1 = stack.pop().unwrap();
        let popped_frame2 = stack.pop().unwrap();
        let popped_frame3 = stack.pop().unwrap();

        assert_eq!(popped_frame1, frame3);
        assert_eq!(popped_frame2, frame2);
        assert_eq!(popped_frame3, frame1);
    }

    #[test]
    fn into_frames() {
        let mut stack = Stack::default();

        let frame1 = Frame::new(Tag::Paragraph, OFFSET, Vec::new());
        let frame2 = Frame::new(Tag::Emphasis, OFFSET, Vec::new());
        let frame3 = Frame::new(Tag::HtmlBlock, OFFSET, Vec::new());

        for frame in [&frame1, &frame2, &frame3] {
            stack.push(frame.clone());
        }

        let tags: Vec<_> = stack.into_frames().map(|frame| frame.tag).collect();

        assert_eq!(tags[0], frame3.tag);
        assert_eq!(tags[1], frame2.tag);
        assert_eq!(tags[2], frame1.tag);
        assert_eq!(tags.len(), 3);
    }

    #[test]
    fn count_frames() {
        let mut stack = Stack::default();

        (1..=5)
            .into_iter()
            .map(|_| Frame::new(Tag::Emphasis, OFFSET, Vec::new()))
            .for_each(|frame| {
                stack.push(frame);
            });

        assert_eq!(stack.count_frames(), 5);
    }

    #[test]
    fn push_parent() {
        let mut stack = Stack::default();

        // get root and add node
        stack.push_parent(Node::new(NodeKind::SoftBreak, (0..1).into()));

        let frame = Frame::new(Tag::Paragraph, OFFSET, Vec::new());
        stack.push(frame);

        let _ = stack.pop().unwrap();

        assert!(stack.frames.is_empty());
        assert_eq!(stack.root.len(), 1);
    }

    #[test]
    fn into_root() {
        let mut stack = Stack::default();

        let frame = Frame::new(Tag::Paragraph, OFFSET, Vec::new());
        stack.push(frame);
        let _ = stack.pop().unwrap();

        let root = stack.into_root();
        assert!(root.is_some());
    }

    #[test]
    fn into_root_but_stack_not_empty() {
        let mut stack = Stack::default();

        let frame = Frame::new(Tag::Paragraph, OFFSET, Vec::new());
        stack.push(frame);

        assert!(stack.into_root().is_none());
    }
}
