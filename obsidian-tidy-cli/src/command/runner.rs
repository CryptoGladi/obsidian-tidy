use crate::Cli;
use std::{ops::Deref, sync::Arc};

pub type DynRunner<E> = Box<dyn Runner<Error = E>>;

pub trait Runner {
    type Error: std::error::Error + Send + Sync;

    fn run(&self, args: &Cli) -> Result<(), Self::Error>;
}

impl<R> Runner for Box<R>
where
    R: Runner,
{
    type Error = R::Error;

    fn run(&self, args: &Cli) -> Result<(), Self::Error> {
        self.as_ref().run(args)
    }
}

pub struct SharedRunner {
    inner: DynRunner<Arc<dyn std::error::Error + Send + Sync>>,
}

impl SharedRunner {
    pub fn new<R>(runner: R) -> Self
    where
        R: Runner + Send + Sync + 'static,
    {
        let erased = Box::new(ErasingRunner::new(runner));

        Self { inner: erased }
    }
}

impl<R> From<R> for SharedRunner
where
    R: Runner + Send + Sync + 'static,
{
    fn from(runner: R) -> Self {
        SharedRunner::new(runner)
    }
}

impl Deref for SharedRunner {
    type Target = DynRunner<Arc<dyn std::error::Error + Send + Sync>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

struct ErasingRunner<R>
where
    R: Runner,
{
    inner: R,
}

impl<R> ErasingRunner<R>
where
    R: Runner + Send + Sync,
{
    pub const fn new(runner: R) -> Self {
        Self { inner: runner }
    }
}

impl<R> Runner for ErasingRunner<R>
where
    R: Runner,
    R::Error: 'static,
{
    type Error = Arc<dyn std::error::Error + Send + Sync>;

    fn run(&self, args: &Cli) -> Result<(), Self::Error> {
        self.inner
            .run(args)
            .map_err(|err| Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>)
    }
}
