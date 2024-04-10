use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec,
};
use dns_protocol::{Flags, ResourceRecord};
use embedded_io::{Read, Write};
use embedded_nal::SocketAddr;
use psp::sys::in_addr;

use crate::{
    dns::utils::create_ptr_type_query,
    socket::{udp::UdpSocketState, ToIpAddr, ToSocketAddr},
};

use self::{
    error::DnsError,
    utils::{create_a_type_query, GOOGLE_DNS_HOST},
};

use super::{socket::udp::UdpSocket, traits};

pub mod error;
mod utils;

/// A DNS resolver
pub struct DnsResolver {
    udp_socket: UdpSocket,
    dns: SocketAddr,
}

impl DnsResolver {
    /// Create a new DNS resolver
    #[allow(unused)]
    pub fn new(dns: SocketAddr) -> Result<Self, DnsError> {
        let mut udp_socket = UdpSocket::open().map_err(|_| DnsError::FailedToCreate)?;
        udp_socket
            .bind(Some(dns))
            .map_err(|_| DnsError::FailedToCreate)?;

        Ok(DnsResolver { udp_socket, dns })
    }

    /// Try to create a new DNS resolver with default settings
    /// The default settings are to use Google's DNS server at `8.8.8.8:53`
    pub fn try_default() -> Result<Self, DnsError> {
        let dns = *GOOGLE_DNS_HOST;
        let mut udp_socket = UdpSocket::open().map_err(|_| DnsError::FailedToCreate)?;
        udp_socket
            .bind(None)
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
    pub fn resolve_hostname(&mut self, host: &str) -> Result<in_addr, DnsError> {
        self.connect_if_not_already()?;

        // create a new query
        let mut questions = [create_a_type_query(host)];
        let query = dns_protocol::Message::new(
            0x42,
            Flags::standard_query(),
            &mut questions,
            &mut [],
            &mut [],
            &mut [],
        );

        // create a new buffer storing the query
        let tx_buf = self.serialise_query(query)?;

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

    fn resolve_addr(&mut self, addr: in_addr) -> Result<String, DnsError> {
        self.connect_if_not_already()?;

        let ip = addr.to_ip_addr().to_string();
        let mut questions = [create_ptr_type_query(&ip)];
        let query = dns_protocol::Message::new(
            0x42,
            Flags::standard_query(),
            &mut questions,
            &mut [],
            &mut [],
            &mut [],
        );

        let tx_buf = self.serialise_query(query)?;

        todo!("resolve_addr")
    }

    fn connect_if_not_already(&mut self) -> Result<(), DnsError> {
        if self.udp_socket.get_socket_state() != UdpSocketState::Connected {
            self.udp_socket
                .connect(self.dns)
                .map_err(|e| DnsError::HostnameResolutionFailed(e.to_string()))?;
        }
        Ok(())
    }

    fn serialise_query(&self, query: dns_protocol::Message) -> Result<vec::Vec<u8>, DnsError> {
        let mut tx_buf = vec![0u8; query.space_needed()];
        query.write(&mut tx_buf).map_err(|_| {
            DnsError::HostnameResolutionFailed("Could not serialize query".to_owned())
        })?;
        Ok(tx_buf)
    }
}

impl traits::dns::ResolveHostname for DnsResolver {
    type Error = DnsError;

    fn resolve_hostname(&mut self, hostname: &str) -> Result<SocketAddr, DnsError> {
        self.resolve_hostname(hostname)
            .map(|addr| addr.to_socket_addr())
    }
}

impl traits::dns::ResolveAddr for DnsResolver {
    type Error = DnsError;

    fn resolve_addr(&mut self, addr: in_addr) -> Result<String, DnsError> {
        self.resolve_addr(addr)
    }
}

impl traits::dns::DnsResolver for DnsResolver {}
