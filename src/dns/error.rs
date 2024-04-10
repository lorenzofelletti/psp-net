use alloc::string::String;

/// An error that can occur when using a DNS resolver
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsError {
    /// The DNS resolver failed to create
    FailedToCreate,
    /// The hostname could not be resolved
    HostnameResolutionFailed(String),
    /// The IP address could not be resolved
    AddressResolutionFailed(String),
}
