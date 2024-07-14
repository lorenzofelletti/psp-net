#![no_std]
#![feature(trait_alias)]
#![feature(type_changing_struct_update)]
#![doc = include_str!("../README.md")]

extern crate alloc;

pub mod constants;
pub mod dns;
pub mod netc;
pub mod socket;
pub mod traits;
pub mod types;
pub mod utils;
