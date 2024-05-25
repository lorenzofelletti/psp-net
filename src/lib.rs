#![cfg_attr(not(feature = "std"), no_std)]
#![feature(trait_alias)]

extern crate alloc;

pub mod constants;
pub mod dns;
pub mod netc;
pub mod socket;
pub mod traits;
pub mod types;
pub mod utils;

// re-exports
pub type SocketAddr = embedded_nal::SocketAddr;
pub type TlsError = embedded_tls::TlsError;

pub trait Write = embedded_io::Write;
pub trait Read = embedded_io::Read;
pub trait ErrorType = embedded_io::ErrorType;
