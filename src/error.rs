//! Error and Result

// TODO: these are placeholder Error & Result types
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;
