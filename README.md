# psp-net
PlayStation Portable (PSP) [rust-psp](https://github.com/overdrivenpotato/rust-psp)-based netwoking crate.

The aim of this crate is to provide a simpler way to create PSP applications using networking.

It provides many useful features, notably:
- A TCP and UDP socket
- A TLS socket
- A DNS resolver.

The TCP Socket provided by this crate is compatible with [embedded-tls](https://github.com/drogue-iot/embedded-tls) TLS socket library.

# Notes
This crate require the use of nightly Rust.
