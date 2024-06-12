use alloc::{string::String, vec::Vec};

use crate::socket::SocketAddr;

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
    pub server_name: String,
    pub(crate) cert: Option<Vec<u8>>,
    pub(crate) enable_rsa_signatures: bool,
}

impl<'a> TlsSocketOptions {
    /// Create a new socket options
    #[must_use]
    pub fn new(seed: u64, server_name: String, cert: Option<Vec<u8>>) -> Self {
        Self {
            seed,
            server_name,
            cert,
            enable_rsa_signatures: true,
        }
    }

    /// Disable RSA signatures
    ///
    /// By default, RSA signatures are enabled.
    pub fn disable_rsa_signatures(&mut self) {
        self.enable_rsa_signatures = false;
    }

    /// Get the seed
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn server_name(&self) -> &str {
        &self.server_name
    }
}

// re-exports
pub type Certificate<'a> = embedded_tls::Certificate<'a>;
