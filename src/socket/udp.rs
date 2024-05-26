use alloc::{boxed::Box, vec::Vec};
use embedded_io::{ErrorType, Read, Write};
use embedded_nal::{IpAddr, Ipv4Addr, SocketAddr};
use psp::sys::{self, sockaddr, socklen_t};

use core::ffi::c_void;

use crate::{
    traits::{
        io::{EasySocket, Open, OptionType},
        SocketBuffer,
    },
    types::SocketOptions,
};

use super::{super::netc, error::SocketError, ToSockaddr};

/// The state of a [`UdpSocket`]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UdpSocketState {
    /// The socket is not yet bound (the bind method has not been called)
    Unbound,
    /// The socket is bound (the bind method has been called)
    Bound,
    /// The socket is connected
    Connected,
}

/// A UDP socket
///
/// # Fields
/// - [`fd`](Self::fd): The socket file descriptor
/// - [`remote`](Self::remote): The remote host to connect to
/// - [`state`](Self::state): The state of the socket
/// - [`buffer`](Self::buffer): The buffer to store data to send
///
/// # Notes
/// - Remote host ([`Self::remote`]) is set when the socket is bound calling [`bind()`](UdpSocket::bind)
/// - In addition to supporting the creation (with [`new`](Self::new)) and manual management of the socket,
///   this struct implements [`EasySocket`] trait, which allows for an easier management of the socket,
///   providing the [`open`](Self::open) method as an alternative to [`new`](Self::new).
///   This method return a [`UdpSocket`] already connected, and ready to send/receive data (using the
///   [`write`](embedded_io::Write::write) and [`read`](embedded_io::Read::read) methods).
#[repr(C)]
pub struct UdpSocket {
    fd: i32,
    remote: Option<sockaddr>,
    state: UdpSocketState,
    buffer: Box<dyn SocketBuffer>,
}

impl UdpSocket {
    /// Create a socket
    #[allow(dead_code)]
    pub fn new() -> Result<UdpSocket, SocketError> {
        let fd = unsafe { sys::sceNetInetSocket(netc::AF_INET as i32, netc::SOCK_DGRAM, 0) };
        if fd < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(UdpSocket {
                fd,
                remote: None,
                state: UdpSocketState::Unbound,
                buffer: Box::<Vec<u8>>::default(),
            })
        }
    }

    /// Bind the socket
    ///
    /// # Parameters
    /// - `addr`: The address to bind to, if `None` bind to `0.0.0.0:0`
    ///
    /// # Returns
    /// - `Ok(())` if the binding was successful
    /// - `Err(String)` if the binding was unsuccessful.
    #[allow(unused)]
    pub fn bind(&mut self, addr: Option<SocketAddr>) -> Result<(), SocketError> {
        if self.state != UdpSocketState::Unbound {
            return Err(SocketError::AlreadyBound);
        }

        let default_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let addr = addr.unwrap_or(default_addr);
        match addr {
            SocketAddr::V4(v4) => {
                let sockaddr = v4.to_sockaddr();

                if unsafe {
                    sys::sceNetInetBind(
                        self.fd,
                        &sockaddr,
                        core::mem::size_of::<netc::sockaddr>() as u32,
                    )
                } != 0
                {
                    let errno = unsafe { sys::sceNetInetGetErrno() };
                    Err(SocketError::Errno(errno))
                } else {
                    self.remote = Some(sockaddr);
                    self.state = UdpSocketState::Bound;
                    Ok(())
                }
            }
            SocketAddr::V6(_) => Err(SocketError::UnsupportedAddressFamily),
        }
    }

    /// Connect to a remote host
    ///
    /// # Notes
    /// The socket must be in state [`UdpSocketState::Bound`] to connect to a remote host.
    #[allow(unused)]
    pub fn connect(&mut self, addr: SocketAddr) -> Result<(), SocketError> {
        match self.state {
            UdpSocketState::Unbound => return Err(SocketError::NotBound),
            UdpSocketState::Bound => {}
            UdpSocketState::Connected => return Err(SocketError::AlreadyConnected),
        }

        match addr {
            SocketAddr::V4(v4) => {
                let sockaddr = v4.to_sockaddr();

                if unsafe { sys::sceNetInetConnect(self.fd, &sockaddr, Self::socket_len()) } != 0 {
                    let errno = unsafe { sys::sceNetInetGetErrno() };
                    Err(SocketError::Errno(errno))
                } else {
                    self.remote = Some(sockaddr);
                    self.state = UdpSocketState::Connected;
                    Ok(())
                }
            }
            SocketAddr::V6(_) => Err(SocketError::UnsupportedAddressFamily),
        }
    }

    /// Read from a socket in state [`UdpSocketState::Connected`]
    #[allow(unused)]
    fn _read(&mut self, buf: &mut [u8]) -> Result<usize, SocketError> {
        if self.state != UdpSocketState::Connected {
            return Err(SocketError::NotConnected);
        }
        let mut sockaddr = self.remote.ok_or(SocketError::Other)?;
        let result =
            unsafe { sys::sceNetInetRecv(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
        if (result as i32) < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(result as usize)
        }
    }

    /// Write to a socket in state [`UdpSocketState::Bound`]
    #[allow(unused)]
    fn _read_from(&mut self, buf: &mut [u8]) -> Result<usize, SocketError> {
        match self.state {
            UdpSocketState::Unbound => return Err(SocketError::NotBound),
            UdpSocketState::Bound => {}
            UdpSocketState::Connected => return Err(SocketError::AlreadyConnected),
        }
        let mut sockaddr = self.remote.ok_or(SocketError::Other)?;
        let result = unsafe {
            sys::sceNetInetRecvfrom(
                self.fd,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                0,
                &mut sockaddr,
                &mut Self::socket_len(),
            )
        };
        if (result as i32) < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(result as usize)
        }
    }

    /// Write to a socket in state [`UdpSocketState::Bound`]
    #[allow(unused)]
    #[allow(clippy::cast_possible_truncation)]
    fn _write_to(&mut self, buf: &[u8], len: usize, to: SocketAddr) -> Result<usize, SocketError> {
        match self.state {
            UdpSocketState::Unbound => return Err(SocketError::NotBound),
            UdpSocketState::Bound => {}
            UdpSocketState::Connected => return Err(SocketError::AlreadyConnected),
        }

        let sockaddr = match to {
            SocketAddr::V4(v4) => Ok(super::socket_addr_v4_to_sockaddr(v4)),
            SocketAddr::V6(_) => Err(SocketError::UnsupportedAddressFamily),
        }?;
        let socklen = core::mem::size_of::<netc::sockaddr>() as u32;

        self.buffer.append_buffer(buf);

        let result = unsafe {
            sys::sceNetInetSendto(
                self.fd,
                buf.as_ptr() as *const c_void,
                len,
                0,
                &sockaddr,
                socklen,
            )
        };
        if (result as i32) < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            self.buffer.shift_left_buffer(result as usize);
            Ok(result as usize)
        }
    }

    #[allow(unused)]
    /// Write to a socket in state [`UdpSocketState::Connected`]
    fn _write(&mut self, buf: &[u8]) -> Result<usize, SocketError> {
        if self.state != UdpSocketState::Connected {
            return Err(SocketError::NotConnected);
        }

        self.buffer.append_buffer(buf);

        self.send()
    }

    fn _flush(&mut self) -> Result<(), SocketError> {
        if self.state != UdpSocketState::Connected {
            return Err(SocketError::NotConnected);
        }

        while !self.buffer.is_empty() {
            self.send()?;
        }
        Ok(())
    }

    fn send(&mut self) -> Result<usize, SocketError> {
        let result = unsafe {
            sys::sceNetInetSend(
                self.fd,
                self.buffer.as_slice().as_ptr() as *const c_void,
                self.buffer.len(),
                0,
            )
        };
        if (result as i32) < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            self.buffer.shift_left_buffer(result as usize);
            Ok(result as usize)
        }
    }

    /// Get the state of the socket
    ///
    /// # Returns
    /// The state of the socket, one of [`UdpSocketState`]
    pub fn get_state(&self) -> UdpSocketState {
        self.state
    }

    fn socket_len() -> socklen_t {
        core::mem::size_of::<netc::sockaddr>() as u32
    }
}

impl Drop for UdpSocket {
    /// Close the socket
    fn drop(&mut self) {
        unsafe {
            sys::sceNetInetClose(self.fd);
        }
    }
}

impl OptionType for UdpSocket {
    type Options = SocketOptions;
}

impl ErrorType for UdpSocket {
    type Error = SocketError;
}

impl Open for UdpSocket {
    fn open(&mut self, options: Self::Options) -> Result<(), Self::Error> {
        self.bind(None)?;
        self.connect(options.remote())?;

        Ok(())
    }
}

impl Read for UdpSocket {
    /// Read from the socket
    ///
    /// # Notes
    /// If the socket is in state [`UdpSocketState::Unbound`] this will return an error,
    /// otherwise it will attempt to read from the socket. You can check the state of the socket
    /// using [`get_socket_state`](Self::get_socket_state).
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        match self.get_state() {
            UdpSocketState::Unbound => Err(SocketError::NotBound),
            UdpSocketState::Bound => self._read_from(buf),
            UdpSocketState::Connected => self._read(buf),
        }
    }
}

impl Write for UdpSocket {
    /// Write to the socket
    ///
    /// # Notes
    /// If the socket is not in state [`UdpSocketState::Connected`] this will return an error.
    /// To connect to a remote host use [`connect`](UdpSocket::connect) first.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        match self.get_state() {
            UdpSocketState::Unbound => Err(SocketError::NotBound),
            UdpSocketState::Bound => Err(SocketError::NotConnected),
            UdpSocketState::Connected => self._write(buf),
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        match self.get_state() {
            UdpSocketState::Unbound => Err(SocketError::NotBound),
            UdpSocketState::Bound => Err(SocketError::NotConnected),
            UdpSocketState::Connected => self._flush(),
        }
    }
}

impl EasySocket for UdpSocket {}
