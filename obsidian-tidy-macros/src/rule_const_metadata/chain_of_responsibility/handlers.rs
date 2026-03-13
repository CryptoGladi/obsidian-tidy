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

        let mut last = self.0.pop().unwrap();
        while let Some(mut current) = self.0.pop() {
            current.set_next(last);
            last = current;
        }

        Some(last)
    }
}
