#![no_std]
#![feature(trait_alias)]
#![doc = include_str!("../README.md")]

extern crate alloc;

pub mod constants;
pub mod dns;
pub mod netc;
pub mod socket;
pub mod traits;
pub mod types;
pub mod utils;
