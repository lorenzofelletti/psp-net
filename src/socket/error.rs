use core::fmt::Display;

use alloc::{borrow::ToOwned, string::String};

/// An error that can occur with a socket
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SocketError {
    /// Unsupported address family
    UnsupportedAddressFamily,
    /// Socket error with errno
    Errno(i32),
    /// Socket error with errno and a description
    ErrnoWithDescription(i32, String),
    /// Other error
    Other(String),
    /// Unknown error
    #[default]
    Unknown,
}

impl SocketError {
    /// Create a new [`SocketError::ErrnoWithDescription`]
    #[must_use]
    pub fn new_errno_with_description(errno: i32, description: &str) -> Self {
        SocketError::ErrnoWithDescription(errno, description.to_owned())
    }
}

impl Display for SocketError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl embedded_io::Error for SocketError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            SocketError::UnsupportedAddressFamily => embedded_io::ErrorKind::Unsupported,
            _ => embedded_io::ErrorKind::Other,
        }
    }
}

// re-exports
pub type TlsError = embedded_tls::TlsError;
