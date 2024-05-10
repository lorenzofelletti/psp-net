use crate::{traits::io::OptionType, SocketAddr};

#[derive(Clone, Copy)]
pub struct SocketOptions {
    remote: SocketAddr,
}

impl OptionType for SocketOptions {
    type Options = Self;
}

impl SocketOptions {
    pub fn new(remote: SocketAddr) -> SocketOptions {
        Self { remote }
    }

    pub fn remote(&self) -> SocketAddr {
        self.remote
    }
}

#[derive(Clone, Copy)]
pub struct TlsSocketOptions {
    remote: SocketAddr,
    seed: u64,
}

impl TlsSocketOptions {
    pub fn new(remote: SocketAddr, seed: u64) -> Self {
        Self { remote, seed }
    }

    pub fn remote(&self) -> SocketAddr {
        self.remote
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }
}
