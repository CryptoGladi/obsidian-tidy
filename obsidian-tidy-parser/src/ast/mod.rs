pub mod node;

use node::Node;
use pulldown_cmark::{
    CowStr, Event as MarkEvent, LinkType, Options as MarkOptions, Parser as MarkParser,
    Tag as MarkTag,
};

pub(crate) enum Frame<'a> {
    Root(Vec<Node<'a>>),
    Tagged {
        tag: MarkTag<'a>,
        children: Vec<Node<'a>>,
    },
}

impl<'a> Frame<'a> {
    pub(crate) fn children_mut(&mut self) -> &mut Vec<Node<'a>> {
        match self {
            Frame::Root(children) => children,
            Frame::Tagged { children, .. } => children,
        }
    }

    pub fn tag_mut(&mut self) -> Option<&mut >
}

pub struct ASTBuilder<I> {
    inner: I,
}

impl<I> ASTBuilder<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<'a, I> ASTBuilder<I>
where
    I: Iterator<Item = MarkEvent<'a>>,
{
    pub fn build(self) -> Node<'a> {
        let mut stack = vec![Frame::Root(Vec::new())];

        for event in self.inner {
            match event {
                MarkEvent::Start(tag) => stack.push(Frame::Tagged {
                    tag,
                    children: Vec::new(),
                }),
                MarkEvent::End(tag) => {
                    let frame = stack.pop().unwrap();
                    
                    frame.
                }
                _ => todo!(),
            }
        }

        todo!()
    }
}

pub trait ASTBuildExt<'a>: Iterator {
    fn build_ast(self) -> Node<'a>;
}

impl<'a, I> ASTBuildExt<'a> for I
where
    I: Iterator<Item = MarkEvent<'a>>,
{
    fn build_ast(self) -> Node<'a> {
        ASTBuilder::new(self).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse() {
        let document = "Это тестовый документ";

        let options = MarkOptions::all();
        let ast = MarkParser::new_ext(document, options).build_ast();
    }
}
