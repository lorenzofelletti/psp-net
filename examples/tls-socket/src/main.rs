#![no_std]
#![no_main]
#![feature(slice_pattern)]

extern crate alloc;

use alloc::string::ToString;
use psp_net::{
    constants::HTTPS_PORT, dns::DnsResolver, tls_socket, traits::dns::ResolveHostname,
    types::SocketRecvFlags,
};

psp::module!("tls-socket", 1, 1);

const GOOGLE_HOST: &str = "google.com";

fn psp_main() {
    psp::enable_home_button();

    // setup network
    let res = psp_net::utils::load_net_modules();
    if let Err(e) = res {
        panic!("Failed to load net modules: {:?}", e);
    }
    psp::dprintln!("Initializing network...");
    let res = psp_net::utils::net_init();
    if let Err(e) = res {
        panic!("Failed to initialize network: {:?}", e);
    }

    unsafe {
        psp::sys::sceNetApctlConnect(1);
        loop {
            let mut state: psp::sys::ApctlState = core::mem::zeroed();
            psp::sys::sceNetApctlGetState(&mut state);
            if let psp::sys::ApctlState::GotIp = state {
                break;
            }
            psp::sys::sceKernelDelayThread(50_000);
        }
    }

    let mut resolver = DnsResolver::try_default().expect("failed to create resolver");
    let mut remote = resolver
        .resolve_hostname(GOOGLE_HOST)
        .expect("failed to resolve hostname");
    remote.set_port(HTTPS_PORT);

    tls_socket! {
        result: _maybe_socket,
        host GOOGLE_HOST => &remote.ip().to_string(),
        recv_flags SocketRecvFlags::MSG_PEEK,
    };

    let mut socket = _maybe_socket.expect("failed to create socket");

    let buf = "GET / HTTP/1.1\r\nHost: google.com\r\n\r\n"
        .as_bytes()
        .to_vec();

    psp_net::write!(buf => socket).expect("failed to write to socket");

    let res = psp_net::read!(string from socket).expect("failed to read from socket");

    psp::dprintln!("{}", res);
}
