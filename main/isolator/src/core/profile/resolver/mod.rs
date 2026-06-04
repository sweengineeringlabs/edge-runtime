//! Profile resolver — maps profile specs to implementations.

#[allow(clippy::module_inception)]
pub(crate) mod resolver;
pub(crate) use resolver::ProfileResolver;
