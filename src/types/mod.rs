mod socket_flags;
#[cfg(feature = "psp")]
mod socket_options;

// re-exports
pub type Certificate<'a> = embedded_tls::Certificate<'a>;

pub use socket_flags::SocketRecvFlags;
pub use socket_flags::SocketSendFlags;
#[cfg(feature = "psp")]
pub use socket_options::SocketOptions;
#[cfg(feature = "psp")]
pub use socket_options::TlsSocketOptions;
