#![allow(clippy::module_name_repetitions)]

use core::fmt;

#[cfg(feature = "macros")]
pub mod macros;
mod request;
mod response;
pub mod types;

/// Enum for different supported HTTP versions
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HttpVersion {
    /// HTTP/1
    V1,
    /// HTTP/1.1
    #[default]
    V1_1,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpVersion::V1 => write!(f, "HTTP/1"),
            HttpVersion::V1_1 => write!(f, "HTTP/1.1"),
        }
    }
}

// re-exports
pub type Method = request::Method;
pub type Request = request::Request;

pub type Response<'a, 'b> = httparse::Response<'a, 'b>;
