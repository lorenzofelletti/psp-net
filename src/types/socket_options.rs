#![allow(clippy::module_name_repetitions)]

use alloc::string::String;
use psp::sys;

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
#[derive(Clone, Debug, Default)]
pub struct TlsSocketOptions<'a> {
    /// The seed to use for the RNG
    seed: u64,
    /// The server name to use
    server_name: String,
    /// The certificate
    cert: Option<Certificate<'a>>,
    /// The CA
    ca: Option<Certificate<'a>>,
    /// Whether RSA signatures should be enabled
    enable_rsa_signatures: bool,
    /// Whether the max fragment length should be reset
    reset_max_fragment_length: bool,
}

impl<'a> TlsSocketOptions<'a> {
    /// Create a new socket options
    ///
    /// # Arguments
    /// - `seed`: The seed to use for the RNG
    /// - `server_name`: The server name to use
    ///
    /// # Returns
    /// - A new socket options object
    ///
    /// # Notes
    /// By default
    /// - RSA signatures are enabled
    /// - The max fragment length is not reset
    /// - The certificate and CA are not set.
    #[must_use]
    pub fn new<S>(seed: u64, server_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            seed,
            server_name: server_name.into(),
            cert: None,
            ca: None,
            enable_rsa_signatures: true,
            reset_max_fragment_length: false,
        }
    }

    /// Create a new socket options with a seed based on the current time
    ///
    /// # Returns
    /// - A new socket options object
    ///
    /// # Notes
    /// Like [`TlsSocketOptions::new`], but the seed is based on the current time.
    /// Uses [`sys::sceRtcGetCurrentTick`] to get the current time.
    #[must_use]
    pub fn new_with_seed_from_time<S>(server_name: S) -> Self
    where
        S: Into<String>,
    {
        let seed = unsafe {
            let mut seed: u64 = 0;
            sys::sceRtcGetCurrentTick(&mut seed);
            seed
        };

        Self::new(seed, server_name)
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
    pub fn set_cert(&mut self, cert: Option<Certificate<'a>>) {
        self.cert = cert;
    }

    /// Get the seed
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Set the seed
    ///
    /// # Arguments
    /// - `seed`: The seed
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
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

    /// Set whether RSA signatures should be enabled
    ///
    /// # Arguments
    /// - `enable_rsa_signatures`: Whether RSA signatures should be enabled
    pub fn set_enable_rsa_signatures(&mut self, enable_rsa_signatures: bool) {
        self.enable_rsa_signatures = enable_rsa_signatures;
    }

    /// Set the server name
    ///
    /// # Arguments
    /// - `server_name`: The server name
    pub fn set_server_name<S>(&mut self, server_name: S)
    where
        S: Into<String>,
    {
        self.server_name = server_name.into();
    }
}
