use dns_protocol::{Label, Question};
use embedded_nal::{IpAddr, Ipv4Addr, SocketAddr};

pub const DNS_PORT: u16 = 53;
lazy_static::lazy_static! {
    pub static ref GOOGLE_DNS_HOST: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), DNS_PORT);
}

#[allow(unused)]
/// Create a DNS query for an A record
pub fn create_a_type_query(domain: &str) -> Question {
    Question::new(domain, dns_protocol::ResourceType::A, 1)
}

pub fn create_ptr_type_query<'a>(ip: &'a str) -> Question<'a> {
    // let ip_v4_addr = Ipv4Addr::from(ip.0.to_be_bytes());
    // let label = Label::from(ip_v4_addr.to_string().as_str());
    let label = Label::from(ip);
    Question::new(label, dns_protocol::ResourceType::Ptr, 1)
}
