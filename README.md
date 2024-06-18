# PSP Net

[![CI](https://github.com/lorenzofelletti/psp-net/actions/workflows/publish.yaml/badge.svg)](https://github.com/lorenzofelletti/psp-net/actions/workflows/publish.yaml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/psp-net.svg)](https://crates.io/crates/psp-net)
[![docs.rs](https://docs.rs/psp-net/badge.svg)](https://docs.rs/psp-net)

PSP Net is a [rust-psp](https://github.com/overdrivenpotato/rust-psp)-based netwoking crate for Sony PlayStation Portable (PSP).

The aim of this crate is to provide a simpler way to create PSP applications using networking.

It provides abstractions for:
- TCP and UDP socket
- TLS socket
- DNS resolution.

The TCP Socket provided by this crate is compatible with [embedded-tls](https://github.com/drogue-iot/embedded-tls) TLS socket library.

# Rust Version Policy
This crate require the use of nightly Rust.

You can set Rust nightly in your project by running `rustup override set nightly`.
To set a specific nightly version, use `rustup override set nightly-<version>`.

## MSRV
The minimum supported Rust version for this crate is (nightly) `1.78.0`.
The maximum supported Rust version for this crate is (nightly) `1.80.0`.
