use core::fmt::Debug;

use alloc::string::String;

use embedded_nal::SocketAddr;
use psp::sys::in_addr;

/// Trait for resolving hostnames
///
/// A type implementing this trait can resolve a hostname to an IP address.
///

pub trait ResolveHostname {
    type Error: Debug;
    /// Resolve a hostname to an IP address
    ///
    /// # Errors
    /// An error will be returned if the hostname could not be resolved.
    fn resolve_hostname(&mut self, hostname: &str) -> Result<SocketAddr, Self::Error>;
}

/// Trait for resolving IP addresses
///
/// A type implementing this trait can resolve an IP address to a hostname.
pub trait ResolveAddr {
    type Error: Debug;
    /// Resolve an IP address to a hostname
    ///
    /// # Errors
    /// An error will be returned if the IP address could not be resolved.
    fn resolve_addr(&mut self, addr: in_addr) -> Result<String, Self::Error>;
}

/// Trait for resolving hostnames and IP addresses.
///
/// This trait combines [`ResolveHostname`] and [`ResolveAddr`].
pub trait DnsResolver: ResolveHostname + ResolveAddr {}
