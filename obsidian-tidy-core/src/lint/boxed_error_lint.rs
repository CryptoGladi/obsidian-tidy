use super::{Category, Content, DynLint, Lint, Violation};
use anyhow::Error as AnyhowError;
use std::{ops::Deref, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error(transparent)]
pub struct WrappedAnyhowError(#[from] Arc<AnyhowError>);

#[derive(Debug)]
pub struct BoxedErrorLint {
    inner: DynLint<WrappedAnyhowError>,
}

impl BoxedErrorLint {
    pub fn new<L>(lint: L) -> Self
    where
        L: Lint + 'static,
        L::Error: std::error::Error + Send + Sync + 'static,
    {
        let boxed = Arc::new(ErasingLint(lint));
        Self { inner: boxed }
    }
}

impl Deref for BoxedErrorLint {
    type Target = DynLint<WrappedAnyhowError>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<BoxedErrorLint> for DynLint<WrappedAnyhowError> {
    fn from(value: BoxedErrorLint) -> Self {
        value.inner
    }
}

impl Clone for BoxedErrorLint {
    fn clone(&self) -> Self {
        let inner = Arc::clone(&self.inner);

        Self { inner }
    }
}

struct ErasingLint<L: Lint>(L);

impl<L: Lint> Lint for ErasingLint<L>
where
    L::Error: std::error::Error + Send + Sync + 'static,
{
    type Error = WrappedAnyhowError;

    fn name(&self) -> &str {
        self.0.name()
    }
    fn description(&self) -> &str {
        self.0.description()
    }
    fn category(&self) -> Category {
        self.0.category()
    }

    fn check(&self, content: &Content) -> Result<Vec<Violation>, Self::Error> {
        self.0
            .check(content)
            .map_err(|e| WrappedAnyhowError(Arc::new(anyhow::Error::from(e))))
    }
}
