use alloc::string::String;

use crate::socket::SocketAddr;

use super::Certificate;

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
    remote: SocketAddr,
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

/// TLS socket options.
///
/// This is used by [`TlsSocket`](super::socket::tls::TlsSocket) when used as a
/// [`EasySocket`](super::traits::io::EasySocket).
#[derive(Clone, Debug)]
pub struct TlsSocketOptions<'a> {
    seed: u64,
    server_name: String,
    cert: Option<Certificate<'a>>,
    ca: Option<Certificate<'a>>,
    enable_rsa_signatures: bool,
    reset_max_fragment_length: bool,
}

impl<'a> TlsSocketOptions<'a> {
    /// Create a new socket options
    ///
    /// # Arguments
    /// - `seed`: The seed to use for the RNG
    /// - `server_name`: The server name to use
    #[must_use]
    pub fn new(seed: u64, server_name: String) -> Self {
        Self {
            seed,
            server_name,
            cert: None,
            ca: None,
            enable_rsa_signatures: true,
            reset_max_fragment_length: false,
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

    /// Get the certificate
    #[must_use]
    pub fn cert(&self) -> Option<&Certificate<'a>> {
        self.cert.as_ref()
    }

    /// Return whether RSA signatures are enabled
    #[must_use]
    pub fn rsa_signatures_enabled(&self) -> bool {
        self.enable_rsa_signatures
    }

    /// Return whether the max fragment length should be reset
    #[must_use]
    pub fn reset_max_fragment_length(&self) -> bool {
        self.reset_max_fragment_length
    }

    /// Set whether the max fragment length should be reset
    ///
    /// By default, the max fragment length is not reset.
    pub fn set_reset_max_fragment_length(&mut self, reset_max_fragment_length: bool) {
        self.reset_max_fragment_length = reset_max_fragment_length;
    }

    /// Get the CA
    #[must_use]
    pub fn ca(&self) -> Option<&Certificate<'a>> {
        self.ca.as_ref()
    }

    /// Set the CA (certificate authority)
    ///
    /// # Arguments
    /// - `ca`: The CA
    pub fn set_ca(&mut self, ca: Option<Certificate<'a>>) {
        self.ca = ca;
    }
}
