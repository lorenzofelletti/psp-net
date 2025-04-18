use alloc::string::String;
use thiserror::Error;

/// An error that can occur with a socket
#[derive(Debug, Clone, PartialEq, Eq, Default, Error)]
pub enum SocketError {
    /// Unsupported address family
    #[error("Unsupported address family")]
    UnsupportedAddressFamily,
    /// Socket error with errno
    #[error("Errno: {0}")]
    Errno(i32),
    /// Socket error with errno and a description
    #[error("Errno: {0} ({1})")]
    ErrnoWithDescription(i32, String),
    /// Other error
    #[error("{0}")]
    Other(String),
    /// Unknown error
    #[default]
    #[error("Unknown error")]
    Unknown,
}

impl SocketError {
    /// Create a new [`SocketError::ErrnoWithDescription`]
    #[must_use]
    pub fn new_errno_with_description<S>(errno: i32, description: S) -> Self
    where
        S: Into<String>,
    {
        SocketError::ErrnoWithDescription(errno, description.into())
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

/// An error that can occur with a TLS socket.
///
/// It can either be a [`TlsError`] or a [`SocketError`] from the
/// underlying socket.
#[derive(Debug, Clone, Error)]
pub enum TlsSocketError {
    /// TLS error
    #[error("TLS error: {}", 0)]
    TlsError(TlsError),
    /// An error with the under
    #[error("Socket error: {0}")]
    SocketError(#[from] SocketError),
}

impl TlsSocketError {
    /// Returns `true` if the error is a [`TlsError`].
    ///
    /// [`TlsError`]: TlsSocketError::TlsError
    #[must_use]
    pub fn is_tls_error(&self) -> bool {
        matches!(self, Self::TlsError(..))
    }

    /// Returns `true` if the error is a [`SocketError`].
    ///
    /// [`SocketError`]: TlsSocketError::SocketError
    #[must_use]
    pub fn is_socket_error(&self) -> bool {
        matches!(self, Self::SocketError(..))
    }
}

impl From<TlsError> for TlsSocketError {
    fn from(value: TlsError) -> Self {
        TlsSocketError::TlsError(value)
    }
}

impl embedded_io::Error for TlsSocketError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            TlsSocketError::TlsError(tls_error) => tls_error.kind(),
            TlsSocketError::SocketError(socket_error) => socket_error.kind(),
        }
    }
}

// re-exports
pub type TlsError = embedded_tls::TlsError;
