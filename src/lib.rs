#![cfg_attr(not(test), no_std)]
#![feature(trait_alias)]
#![doc = include_str!("../README.md")]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![feature(slice_pattern)]

extern crate alloc;

pub mod constants;
#[cfg(feature = "psp")]
pub mod dns;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "macros")]
pub mod macros;
#[cfg(feature = "psp")]
pub mod netc;
#[cfg(feature = "psp")]
pub mod socket;
pub mod traits;
pub mod types;
#[cfg(feature = "psp")]
pub mod utils;
