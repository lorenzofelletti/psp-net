use crate::{traits::io::OptionType, SocketAddr};

/// Socket options, such as remote address to connect to.
///
/// This is used by [`TcpSocket`](super::socket::tcp::TcpSocket) and
/// [`UdpSocket`](super::socket::udp::UdpSocket) when used as
/// [`EasySocket`](super::traits::io::EasySocket)s.
///
/// # Fields
/// - [`remote`]: Remote address to connect to
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
    pub fn new(remote: SocketAddr) -> SocketOptions {
        Self { remote }
    }

    /// Get the remote address
    pub fn remote(&self) -> SocketAddr {
        self.remote
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TlsSocketOptions {
    // TODO: decomment once found a way to configure server_name in open
    // pub server_name: String,
    pub seed: u64,
}

impl TlsSocketOptions {
    /// Create a new socket options
    pub fn new(/*server_name: String,*/ seed: u64) -> Self {
        Self {
            /*server_name,*/ seed,
        }
    }

    // /// Get the remote address
    // pub fn server_name(&self) -> String {
    //     self.server_name.clone()
    // }

    /// Get the seed
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
