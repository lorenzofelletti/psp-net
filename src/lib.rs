#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

extern crate alloc;

pub mod constants;
pub mod dns;
pub mod netc;
pub mod socket;
pub mod traits;
pub mod utils;

// re-export
pub type SocketAddr = embedded_nal::SocketAddr;
pub type TlsError = embedded_tls::TlsError;
