//! Infrastructure Layer - I/O Implementations and Technical Details
//!
//! This layer contains concrete implementations of domain ports (traits).

pub mod fs_adapter;
pub mod logger;
pub mod regex_service;

pub use fs_adapter::FsAdapter;
pub use logger::SimpleLogger;