use alloc::string::String;

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

#[derive(Clone, Debug)]
/// TLS socket options
///
/// # Fields
/// - [`seed`](Self::seed): Seed for the RNG
pub struct TlsSocketOptions<'a> {
    pub seed: u64,
    pub server_name: String,
    pub(crate) cert: Option<Certificate<'a>>,
    pub(crate) enable_rsa_signatures: bool,
}

impl<'a> TlsSocketOptions<'a> {
    /// Create a new socket options
    #[must_use]
    pub fn new(seed: u64, server_name: String) -> Self {
        Self {
            seed,
            server_name,
            cert: None,
            enable_rsa_signatures: true,
        }
    }

    /// Disable RSA signatures
    ///
    /// By default, RSA signatures are enabled.
    pub fn disable_rsa_signatures(&mut self) {
        self.enable_rsa_signatures = false;
    }

    /// Set the certificate
    ///
    /// # Arguments
    /// - `cert`: The certificate
    pub fn set_cert(&mut self, cert: Certificate<'a>) {
        self.cert = Some(cert);
    }

    /// Get the seed
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Get the server name
    #[must_use]
    pub fn server_name(&self) -> &str {
        &self.server_name
    }
}

// re-exports
pub type Certificate<'a> = embedded_tls::Certificate<'a>;
