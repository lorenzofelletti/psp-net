use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec as a_vec,
};
use dns_protocol::{Flags, Question, ResourceRecord};
use embedded_io::{Read, Write};
use embedded_nal::{IpAddr, Ipv4Addr, SocketAddr};
use psp::sys::in_addr;

use crate::socket::udp::UdpSocketState;

use super::{
    socket::{udp::UdpSocket, ToSocketAddr},
    traits,
};

pub const DNS_PORT: u16 = 53;
lazy_static::lazy_static! {
    static ref GOOGLE_DNS_HOST: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), DNS_PORT);
}

/// Create a DNS query for an A record
#[allow(unused)]
#[must_use]
pub fn create_a_type_query(domain: &str) -> Question {
    Question::new(domain, dns_protocol::ResourceType::A, 1)
}

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

/// A DNS resolver
pub struct DnsResolver {
    /// The UDP socket that is used to send and receive DNS messages
    udp_socket: UdpSocket,
    /// The DNS server address
    dns: SocketAddr,
}

impl DnsResolver {
    /// Create a new DNS resolver
    ///
    /// # Parameters
    /// - `dns`: The [`SocketAddr`] of the DNS server
    ///
    /// # Errors
    /// - [`DnsError::FailedToCreate`]: The DNS resolver failed to create. This may
    ///   happen if the socket could not be created or bound to the specified address
    #[allow(unused)]
    pub fn new(dns: SocketAddr) -> Result<Self, DnsError> {
        let mut udp_socket = UdpSocket::new().map_err(|_| DnsError::FailedToCreate)?;
        udp_socket
            .bind(Some(dns))
            .map_err(|_| DnsError::FailedToCreate)?;

        Ok(DnsResolver { udp_socket, dns })
    }

    /// Try to create a new DNS resolver with default settings
    /// The default settings are to use Google's DNS server at `8.8.8.8:53`
    ///
    /// # Errors
    /// - [`DnsError::FailedToCreate`]: The DNS resolver failed to create. This may
    ///   happen if the socket could not be created or bound to the specified address
    pub fn try_default() -> Result<Self, DnsError> {
        let dns = *GOOGLE_DNS_HOST;
        let mut udp_socket = UdpSocket::new().map_err(|_| DnsError::FailedToCreate)?;
        udp_socket
            .bind(Some(dns))
            .map_err(|_| DnsError::FailedToCreate)?;

        Ok(DnsResolver { udp_socket, dns })
    }

    /// Resolve a hostname to an IP address
    ///
    /// # Parameters
    /// - `host`: The hostname to resolve
    ///
    /// # Returns
    /// - `Ok(in_addr)`: The IP address of the hostname
    /// - `Err(())`: If the hostname could not be resolved
    ///
    /// # Errors
    /// - [`DnsError::HostnameResolutionFailed`]: The hostname could not be resolved.
    ///   This may happen if the connection of the socket fails, or if the DNS server
    ///   does not answer the query, or any other error occurs
    pub fn resolve(&mut self, host: &str) -> Result<in_addr, DnsError> {
        // connect to the DNS server, if not already
        if self.udp_socket.get_state() != UdpSocketState::Connected {
            self.udp_socket
                .connect(self.dns)
                .map_err(|e| DnsError::HostnameResolutionFailed(e.to_string()))?;
        }

        // create a new query
        let mut questions = [super::dns::create_a_type_query(host)];
        let query = dns_protocol::Message::new(
            0x42,
            Flags::standard_query(),
            &mut questions,
            &mut [],
            &mut [],
            &mut [],
        );

        // create a new buffer with the size of the message
        let mut tx_buf = a_vec![0u8; query.space_needed()];
        // serialize the message into the buffer
        query.write(&mut tx_buf).map_err(|_| {
            DnsError::HostnameResolutionFailed("Could not serialize query".to_owned())
        })?;

        // send the message to the DNS server
        let _ = self
            .udp_socket
            .write(&tx_buf)
            .map_err(|e| DnsError::HostnameResolutionFailed(e.to_string()))?;

        let mut rx_buf = [0u8; 1024];

        // receive the response from the DNS server
        let data_len = self
            .udp_socket
            .read(&mut rx_buf)
            .map_err(|e| DnsError::HostnameResolutionFailed(e.to_string()))?;

        if data_len == 0 {
            return Err(DnsError::HostnameResolutionFailed(
                "No data received".to_owned(),
            ));
        }

        // parse the response
        let mut answers = [ResourceRecord::default(); 16];
        let mut authority = [ResourceRecord::default(); 16];
        let mut additional = [ResourceRecord::default(); 16];
        let message = dns_protocol::Message::read(
            &rx_buf[..data_len],
            &mut questions,
            &mut answers,
            &mut authority,
            &mut additional,
        )
        .map_err(|_| DnsError::HostnameResolutionFailed("Could not parse response".to_owned()))?;

        if message.answers().is_empty() {
            return Err(DnsError::HostnameResolutionFailed(
                "No answers received".to_owned(),
            ));
        }
        let answer = message.answers()[0];

        match answer.data().len() {
            4 => {
                let mut octets = [0u8; 4];
                octets.copy_from_slice(answer.data());
                let addr = in_addr(u32::from_be_bytes(octets));
                Ok(addr)
            }
            _ => Err(DnsError::HostnameResolutionFailed(
                "Could not parse IP address".to_owned(),
            )),
        }
    }
}

impl traits::dns::ResolveHostname for DnsResolver {
    type Error = DnsError;

    /// Resolve a hostname to an IP address
    ///
    /// # Parameters
    /// - `host`: The hostname to resolve
    ///
    /// # Returns
    /// - `Ok(SocketAddr)`: The IP address of the hostname
    /// - `Err(DnsError)`: If the hostname could not be resolved
    ///
    /// # Errors
    /// - [`DnsError::HostnameResolutionFailed`]: The hostname could not be resolved.
    ///   This may happen if the connection of the socket fails, or if the DNS server
    ///   does not answer the query, or any other error occurs
    fn resolve_hostname(&mut self, hostname: &str) -> Result<SocketAddr, DnsError> {
        self.resolve(hostname).map(|addr| addr.to_socket_addr())
    }
}

impl traits::dns::ResolveAddr for DnsResolver {
    type Error = DnsError;

    fn resolve_addr(&mut self, _addr: in_addr) -> Result<String, DnsError> {
        todo!("resolve_addr")
    }
}

impl traits::dns::DnsResolver for DnsResolver {}
