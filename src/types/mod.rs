use crate::{traits::io::OptionType, SocketAddr};

/// Socket options, such as remote address to connect to.
///
/// This is used by [`TcpSocket`](super::socket::tcp::TcpSocket) and
/// [`UdpSocket`](super::socket::udp::UdpSocket) when used as
/// [`EasySocket`](super::traits::io::EasySocket)s.
///
/// # Fields
/// - [`remote`](Self::remote): Remote address to connect to
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SocketOptions {
    /// Remote address to connect to
    pub remote: SocketAddr,
}

impl OptionType for SocketOptions {
    type Options = Self;
}

impl SocketOptions {
    /// Create a new socket options
    #[must_use]
    pub fn new(remote: SocketAddr) -> SocketOptions {
        Self { remote }
    }

    /// Get the remote address
    #[must_use]
    pub fn remote(&self) -> SocketAddr {
        self.remote
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// TLS socket options
///
/// # Fields
/// - [`seed`](Self::seed): Seed for the RNG
pub struct TlsSocketOptions {
    pub seed: u64,
}

impl TlsSocketOptions {
    /// Create a new socket options
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Get the seed
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
