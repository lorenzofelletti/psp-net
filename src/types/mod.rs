use crate::{traits::io::OptionType, SocketAddr};

/// Socket options, such as remote address to connect to.
/// This is used by [`TcpSocket`](super::socket::tcp::TcpSocket) and
/// [`UdpSocket`](super::socket::udp::UdpSocket) when used as
/// [`EasySocket`](super::traits::io::EasySocket)s.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SocketOptions {
    pub remote: SocketAddr,
}

impl OptionType for SocketOptions {
    type Options = Self;
}

impl SocketOptions {
    /// Create a new socket options
    pub fn new(remote: SocketAddr) -> SocketOptions {
        Self { remote }
    }

    /// Get the remote address
    pub fn remote(&self) -> SocketAddr {
        self.remote
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TlsSocketOptions {
    pub remote: SocketAddr,
    pub seed: u64,
}

impl TlsSocketOptions {
    /// Create a new socket options
    pub fn new(remote: SocketAddr, seed: u64) -> Self {
        Self { remote, seed }
    }

    /// Get the remote address
    pub fn remote(&self) -> SocketAddr {
        self.remote
    }

    /// Get the seed
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
