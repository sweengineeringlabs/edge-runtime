//! Core implementations (pub(crate) only).
pub(crate) mod noop;
pub(crate) mod server;
pub(crate) mod tls;

#[cfg(test)]
mod tests;
