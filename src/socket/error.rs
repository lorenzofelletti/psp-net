use core::fmt::Display;

/// An error that can occur with a socket
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketError {
    /// The socket is not connected
    NotConnected,
    /// The socket is already connected
    AlreadyConnected,
    /// The socket is already bound
    AlreadyBound,
    /// The socket is not bound
    NotBound,
    /// Unsupported address family
    UnsupportedAddressFamily,
    /// Socket error with errno
    Errno(i32),
    /// Other error
    #[default]
    Other,
}

impl Display for SocketError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl embedded_io::Error for SocketError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            SocketError::NotConnected => embedded_io::ErrorKind::NotConnected,
            SocketError::UnsupportedAddressFamily => embedded_io::ErrorKind::Unsupported,
            _ => embedded_io::ErrorKind::Other,
        }
    }
}

// re-exports
pub type TlsError = embedded_tls::TlsError;
