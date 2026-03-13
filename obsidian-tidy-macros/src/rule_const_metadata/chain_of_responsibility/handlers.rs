//! Builder for constructing a chain of validation handlers.
//!
//! The [`Handlers`] struct collects multiple handlers and then builds them into
//! a linked list using [`Handlers::build_chain`]. The chain is built in reverse
//! order so that the first handler added becomes the first to be executed.

use super::Handler;

#[derive(Default)]
pub struct Handlers<D>(Vec<Box<dyn Handler<Data = D>>>);

impl<D> Handlers<D> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add<H>(mut self, handler: H) -> Self
    where
        H: Handler<Data = D> + 'static,
    {
        self.0.push(Box::new(handler));
        self
    }

    pub fn build_chain(mut self) -> Option<Box<dyn Handler<Data = D>>> {
        if self.0.is_empty() {
            return None;
        }

        #[allow(clippy::unwrap_used, reason = "`self.0` is not empty")]
        let mut last = self.0.pop().unwrap();
        while let Some(mut current) = self.0.pop() {
            current.set_next(last);
            last = current;
        }

        Some(last)
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;

    use super::*;
    use crate::{
        rule_const_metadata::chain_of_responsibility::run_chain,
        test_utils::test_handler::TestHandler,
    };
    use std::sync::mpsc::channel;

    #[test]
    fn empty() {
        let result = Handlers::<()>::default().build_chain();
        assert!(result.is_none());
    }

    #[test]
    fn build_chain() {
        let (sender, receiver) = channel();

        let test_handler1 = TestHandler::new(sender.clone(), 1);
        let test_handler2 = TestHandler::new(sender.clone(), 2);
        let test_handler3 = TestHandler::new(sender, 3);

        let chain = Handlers::new()
            .add(test_handler1)
            .add(test_handler2)
            .add(test_handler3)
            .build_chain()
            .unwrap();

        run_chain(chain.as_ref(), &(), Span::call_site()).unwrap();
        drop(chain); // Close all senders

        assert_eq!(receiver.iter().collect::<Vec<_>>(), [1, 2, 3])
    }
}
