//! This module contains the socket types.
//! Currently, three socket types are provided:
//! - [`udp::UdpSocket`] – a UDP socket
//! - [`tcp::TcpSocket`] – a TCP socket
//! - [`tls::TlsSocket`] – a TCP socket wrapper that provides a TLS connection

#![allow(clippy::module_name_repetitions)]

use core::net::Ipv4Addr;
use psp::sys::{in_addr, sockaddr};

use super::netc;

pub mod error;
#[cfg(feature = "macros")]
pub mod macros;
mod sce;
pub mod state;
pub mod tcp;
pub mod tls;
pub mod udp;

/// Convert a [`SocketAddrV4`] to a [`sockaddr`]
fn socket_addr_v4_to_sockaddr(addr: SocketAddrV4) -> sockaddr {
    let octets = addr.ip().octets();
    let sin_addr = u32::from_le_bytes(octets);
    let port = addr.port().to_be();

    let sockaddr_in = netc::sockaddr_in {
        sin_len: core::mem::size_of::<netc::sockaddr_in>() as u8,
        sin_family: netc::AF_INET,
        sin_port: port,
        sin_addr: netc::in_addr(sin_addr),
        sin_zero: [0u8; 8],
    };

    unsafe { core::mem::transmute::<netc::sockaddr_in, netc::sockaddr>(sockaddr_in) }
}

/// Convert to a [`sockaddr`]
pub trait ToSockaddr {
    /// Convert to a [`sockaddr`]
    fn to_sockaddr(&self) -> sockaddr;
}

impl ToSockaddr for SocketAddrV4 {
    fn to_sockaddr(&self) -> sockaddr {
        socket_addr_v4_to_sockaddr(*self)
    }
}

/// Convert to a [`SocketAddr`]
pub trait ToSocketAddr {
    /// Convert to a [`SocketAddr`]
    fn to_socket_addr(&self) -> SocketAddr;
}

impl ToSocketAddr for in_addr {
    fn to_socket_addr(&self) -> SocketAddr {
        let octets = self.0.to_be_bytes();
        let ip = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);
        SocketAddr::V4(SocketAddrV4::new(ip, 0))
    }
}

impl ToSocketAddr for sockaddr {
    fn to_socket_addr(&self) -> SocketAddr {
        let sockaddr_in =
            unsafe { core::mem::transmute::<netc::sockaddr, netc::sockaddr_in>(*self) };

        let octets = sockaddr_in.sin_addr.0.to_be_bytes();

        let port = u16::to_be(sockaddr_in.sin_port);

        SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
            port,
        ))
    }
}

// re-exports
pub type SocketAddr = core::net::SocketAddr;
pub type SocketAddrV4 = core::net::SocketAddrV4;
