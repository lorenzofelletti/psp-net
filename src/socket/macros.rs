#[macro_export]
/// Get the current timestamp
macro_rules! timestamp {
    () => {{
        let mut seed = 0;
        unsafe {
            psp::sys::sceRtcGetCurrentTick(&mut seed);
        }
        seed
    }};
}

#[macro_export]
/// Create a new TLS socket
///
/// The macro will try to open a new TLS connection to the provided remote address.
/// The socket will be stored in a variable named as provided.
/// Please note that the variable will contain a `Result<TlsSocket<'_, Ready>, SocketError>`,
/// not a `TlsSocket<'_, Ready>` directly.
///
/// # Parameters
/// - `name`: The name of the variable where the socket will be stored
/// - `host`: The hostname and address to connect to
/// - `send_flags`: (Optional) The send flags to be used (by the underlying TCP socket)
/// - `recv_flags`: (Optional) The receive flags to be used (by the underlying TCP socket)
/// - `seed`: (Optional) The seed to use for the RNG, if not provided, the current timestamp is used
/// - `cert`: (Optional) The certificate to use
/// - `ca`: (Optional) The CA to use
/// - `enable_rsa_signatures`: (Optional, default `true`) Whether to enable RSA signatures
/// - `reset_max_fragment_length`: (Optional, default `false`) Whether to reset the max fragment length
///
/// # Safety
/// - The macro will panic if the provided IP address is invalid
///
/// # Example
/// ```no_run
/// tls_socket! {
///     name: tls_socket,
///     host "myhost.com" => "1.2.3.4",
/// }
/// let mut tls_socket = tls_socket?;
/// tls_socket.write_all("hello world".as_bytes());
/// ```
macro_rules! tls_socket {
    (
        name: $name:ident,
        host $host:expr => $remote:expr,
        send_flags $send_flags:expr,
        recv_flags $recv_flags:expr,
        seed $seed:expr,
        cert $cert:expr,
        ca $ca:expr,
        enable_rsa_signatures $enable_rsa_signatures:expr,
        reset_max_fragment_length $mfl:expr,
    ) => {
        use alloc::format;
        use core::net::Ipv4Addr;
        use core::str::FromStr;
        use $crate::socket::state::Ready;
        use $crate::types::TlsSocketOptions;
        use $crate::socket::tcp::TcpSocket;
        use $crate::socket::tls::TlsSocket;
        use $crate::traits::io::Open;
        use $crate::socket::{error::SocketError, SocketAddr, SocketAddrV4};

        let mut $name: Result<TlsSocket<Ready>, SocketError> = Err(SocketError::Unknown);

        let ip = Ipv4Addr::from_str($remote).unwrap();
        let addr = SocketAddr::V4(SocketAddrV4::new(ip, 443));
        let s = TcpSocket::new();

        if let Ok(mut s) = s {
            if let Some(send_flags) = $send_flags {
                s.set_send_flags(send_flags);
            }
            if let Some(recv_flags) = $recv_flags {
                s.set_recv_flags(recv_flags);
            }
            let s = s.connect(addr);
            if let Ok(s) = s {
                let mut read_buf = TlsSocket::new_buffer();
                let mut write_buf = TlsSocket::new_buffer();
                $name = TlsSocket::new(s, &mut read_buf, &mut write_buf);
                let mut options = TlsSocketOptions::new($seed, $host.to_string());
                options.set_cert($cert);
                options.set_ca($ca);
                options.set_enable_rsa_signatures($enable_rsa_signatures);
                options.set_reset_max_fragment_length($mfl);
                $name = $name.open(&options);
            } else {
                $name = Err(SocketError::Other(
                    format!("Failed to connect to {}: {}", $host, s.err().unwrap())));
            }
        } else {
            $name = Err(SocketError::Other(
                format!("Failed to create socket: {}", s.err().unwrap())));
        }
    };
    (
        name: $name:ident,
        host $host:expr => $remote:expr,
        $(send_flags $send_flags:expr,)?
        $(recv_flags $recv_flags:expr,)?
        $(seed $seed:expr,)?
        $(cert $cert:expr,)?
        $(ca $ca:expr,)?
        $(enable_rsa_signatures $enable_rsa_signatures:expr,)?
        $(reset_max_fragment_length $mfl:expr,)?
    ) => {
        use $crate::timestamp;

        let seed = $crate::some_or_none!($($seed)?);
        let seed = seed.unwrap_or(timestamp!());
        let cert = $crate::some_or_none!($($cert)?);
        let ca = $crate::some_or_none!($($ca)?);
        let enable_rsa_signatures = $crate::some_or_none!($($enable_rsa_signatures)?);
        let enable_rsa_signatures = enable_rsa_signatures.unwrap_or(true);
        let reset_max_fragment_length = $crate::some_or_none!($($mfl)?);
        let reset_max_fragment_length = reset_max_fragment_length.unwrap_or(false);
        let send_flags = $crate::some_or_none!($($send_flags)?);
        let recv_flags = $crate::some_or_none!($($recv_flags)?);

        tls_socket! {
            name: $name,
            host $host => $remote,
            send_flags send_flags,
            recv_flags recv_flags,
            seed seed,
            cert cert,
            ca ca,
            enable_rsa_signatures enable_rsa_signatures,
            reset_max_fragment_length reset_max_fragment_length,
        }
    };
    (
        name: $name:ident,
        host $host:expr => $remote:expr,
        $(send_flags $send_flags:expr,)?
        $(recv_flags $recv_flags:expr,)?
        opts $opts:expr,
    ) => {
        let send_flags = $crate::some_or_none!($($send_flags)?);
        let recv_flags = $crate::some_or_none!($($recv_flags)?);
        tls_socket! {
            name: $name,
            host $host => $remote,
            send_flags send_flags,
            recv_flags recv_flags,
            seed $opts.seed(),
            cert $opts.cert(),
            ca $opts.ca(),
            enable_rsa_signatures $opts.enable_rsa_signatures(),
            reset_max_fragment_length $opts.reset_max_fragment_length(),
        }
    }
}

/// Read from a TLS socket
///
/// The macro need a `&mut TlsSocket<'_, Ready>` as input.
///
/// The macro supports the following syntaxes:
/// ```no_run
/// // syntax 1
/// read!(from socket);
/// // syntax 2
/// read!(from socket => buf);
/// // syntax 3
/// read!(string from socket);
/// ```
///
/// # Example
/// Read a string from the socket
/// ```no_run
/// if let Ok(s) = read!(string from socket) {
/// println!("{}", s);
/// }
/// ```
#[macro_export]
macro_rules! read {
    (from $socket:ident) => {{
        let mut buf = [0; $crate::socket::tls::MAX_FRAGMENT_LENGTH as usize];
        $socket.read(&mut buf)
    }};
    (from $socket:ident => $buf:ident) => {{
        $socket.read(&mut $buf)
    }};
    (string from $socket:ident) => {
        $socket.read_string()
    };
}

/// Write to a TLS socket
///
/// The macro need a `&mut TlsSocket<'_, Ready>` as input.
///
/// # Example
/// ```no_run
/// write!(buf => socket)?;
/// ```
#[macro_export]
macro_rules! write {
    ($buf:ident => $socket:ident) => {{
        use core::slice::SlicePattern;
        $socket.write_all($buf.as_slice())
    }};
}
