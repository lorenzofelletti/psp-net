mod socket_flags;
mod socket_options;

// re-exports
pub type Certificate<'a> = embedded_tls::Certificate<'a>;

pub use socket_flags::SocketRecvFlags;
pub use socket_flags::SocketSendFlags;
pub use socket_options::SocketOptions;
pub use socket_options::TlsSocketOptions;
