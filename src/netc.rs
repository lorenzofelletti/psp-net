#[allow(unused)]
pub const AF_INET: u8 = 2;
/// Stream socket (TCP)
#[allow(unused)]
pub const SOCK_STREAM: i32 = 1;
/// Datagram socket (UDP)
#[allow(unused)]
pub const SOCK_DGRAM: i32 = 2;

pub use psp::sys::in_addr;

pub use psp::sys::sockaddr;

/// A structure like Linux's `sockaddr_in`
#[repr(C)]
#[allow(nonstandard_style)]
pub struct sockaddr_in {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

impl Clone for sockaddr_in {
    fn clone(&self) -> Self {
        sockaddr_in {
            sin_len: self.sin_len,
            sin_family: self.sin_family,
            sin_port: self.sin_port,
            sin_addr: in_addr(self.sin_addr.0),
            sin_zero: self.sin_zero,
        }
    }
}
