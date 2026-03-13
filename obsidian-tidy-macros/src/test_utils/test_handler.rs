//! This module provides a [`TestHandler`] that can be used to verify that handlers
//! are executed as expected in a chain. It sends a message through a channel
//! when its `handle` method is called, allowing tests to assert that a particular
//! handler was reached

use crate::rule_const_metadata::chain_of_responsibility::Handler;
use proc_macro2::Span;
use std::sync::mpsc::Sender;

/// A handler that sends a cloned value through a channel when invoked.
///
/// This is useful for testing whether a handler is called and in what order
/// when placed in a chain. It does not perform any validation; it simply
/// emits a signal via the channel.
///
/// # Type Parameters
///
/// - `T`: The type of the value to send
/// - `D`: The type of data expected by the handler
///
/// # Example
///
/// ```
/// use std::sync::mpsc::channel;
/// use proc_macro2::Span;
/// use obsidian_tidy_macros::rule_const_metadata::chain_of_responsibility::{
///     Handler, run_chain, test_utils::TestHandler,
/// };
///
/// let (sender, receiver) = channel();
///
/// let handler = TestHandler::new(sender, "executed");
/// handler.handle(&(), Span::call_site()).unwrap();
///
/// assert_eq!(receiver.recv().unwrap(), "executed");
/// ```
pub struct TestHandler<T, D>
where
    T: Clone,
{
    next: Option<Box<dyn Handler<Data = D>>>,
    sender: Sender<T>,
    data_for_send: T,
}

impl<T, D> TestHandler<T, D>
where
    T: Clone,
{
    pub fn new(sender: Sender<T>, data_for_send: T) -> Self {
        Self {
            next: None,
            sender,
            data_for_send,
        }
    }
}

impl<T, D> Handler for TestHandler<T, D>
where
    T: Clone,
{
    type Data = D;

    fn handle(&self, _data: &Self::Data, _span: Span) -> syn::Result<()> {
        self.sender.send(self.data_for_send.clone()).unwrap();

        Ok(())
    }

    fn next(&self) -> Option<&dyn Handler<Data = Self::Data>> {
        self.next.as_deref()
    }

    fn set_next(&mut self, next: Box<dyn Handler<Data = Self::Data>>) {
        self.next = Some(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn send() {
        let (sender, receiver) = channel();

        let test_handler = TestHandler::new(sender, "IS WORK");
        test_handler.handle(&(), Span::call_site()).unwrap();

        assert_eq!(receiver.recv().unwrap(), "IS WORK");
    }
}
