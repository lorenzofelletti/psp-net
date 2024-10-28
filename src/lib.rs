#![no_std]
#![feature(trait_alias)]
#![doc = include_str!("../README.md")]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![feature(slice_pattern)]

extern crate alloc;

pub mod constants;
pub mod dns;
#[cfg(feature = "http")]
pub mod http;
pub mod netc;
pub mod socket;
pub mod traits;
pub mod types;
pub mod utils;
