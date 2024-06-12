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

use super::{super::netc, error::SocketError, ToSockaddr, ToSocketAddr};

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
/// # Notes
/// - Remote host ([`Self::remote`]) is set when the socket is bound calling [`bind()`](UdpSocket::bind)
/// - In addition to supporting the creation (with [`new`](Self::new)) and manual management of the socket,
///   this struct implements [`EasySocket`] trait, which allows for an easier management of the socket,
///   providing the [`open`](Self::open) method as an alternative to [`new`](Self::new).
///   This method return a [`UdpSocket`] already connected, and ready to send/receive data (using the
///   [`write`](embedded_io::Write::write) and [`read`](embedded_io::Read::read) methods).
/// - The socket is closed when the struct is dropped. Closing via drop is best-effort.
#[repr(C)]
pub struct UdpSocket {
    /// The socket file descriptor
    fd: i32,
    /// The remote host to connect to
    remote: Option<sockaddr>,
    /// The state of the socket
    state: UdpSocketState,
    /// The buffer to store data to send
    buffer: Box<dyn SocketBuffer>,
}

impl UdpSocket {
    /// Create a socket
    ///
    /// # Notes
    /// - Creating a new socket is not sufficient to start sending/receiving data.
    ///   You must call [`Self::open()`] (if you're using it in [`EasySocket`] mode), or
    ///   [`Self::bind()`] and/or [`Self::connect()`].
    ///
    /// # Errors
    /// - [`SocketError::Errno`] if the socket could not be created
    #[allow(dead_code)]
    pub fn new() -> Result<UdpSocket, SocketError> {
        let fd = unsafe { sys::sceNetInetSocket(i32::from(netc::AF_INET), netc::SOCK_DGRAM, 0) };
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
    /// - `addr`: The address to bind to, if `None` binds to `0.0.0.0:0`
    ///
    /// # Returns
    /// - `Ok(())` if the binding was successful
    /// - `Err(String)` if the binding was unsuccessful.
    ///
    /// # Errors
    /// - [`SocketError::AlreadyBound`] if the socket is already bound
    /// - [`SocketError::Errno`] if the binding was unsuccessful
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
    /// To bind the socket use [`bind()`](UdpSocket::bind).
    ///
    /// # Returns
    /// - `Ok(())` if the connection was successful
    /// - `Err(SocketError)` if the connection was unsuccessful
    ///
    /// # Errors
    /// - [`SocketError::NotBound`] if the socket is not bound
    /// - [`SocketError::AlreadyConnected`] if the socket is already connected
    /// - Any other [`SocketError`] if the connection was unsuccessful
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
    ///
    /// # Returns
    /// - `Ok(usize)` if the read was successful. The number of bytes read
    /// - `Err(SocketError)` if the read was unsuccessful.
    ///
    /// # Errors
    /// - [`SocketError::NotConnected`] if the socket is not connected
    /// - Any other [`SocketError`] if the read was unsuccessful
    #[allow(unused)]
    pub fn _read(&mut self, buf: &mut [u8]) -> Result<usize, SocketError> {
        if self.state != UdpSocketState::Connected {
            return Err(SocketError::NotConnected);
        }
        let mut sockaddr = self.remote.ok_or(SocketError::Other)?;
        let result = unsafe {
            sys::sceNetInetRecv(self.fd, buf.as_mut_ptr().cast::<c_void>(), buf.len(), 0)
        };
        if (result as i32) < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(result as usize)
        }
    }

    /// Write to a socket in state [`UdpSocketState::Bound`]
    ///
    /// # Returns
    /// - `Ok(usize)` if the write was successful. The number of bytes read
    /// - `Err(SocketError)` if the read was unsuccessful.
    ///
    /// # Errors
    /// - [`SocketError::NotBound`] if the socket is not bound
    /// - [`SocketError::AlreadyConnected`] if the socket is already connected
    /// - Any other [`SocketError`] if the read was unsuccessful
    #[allow(unused)]
    pub fn _read_from(&mut self, buf: &mut [u8]) -> Result<usize, SocketError> {
        match self.state {
            UdpSocketState::Unbound => return Err(SocketError::NotBound),
            UdpSocketState::Bound => {}
            UdpSocketState::Connected => return Err(SocketError::AlreadyConnected),
        }
        let mut sockaddr = self.remote.ok_or(SocketError::Other)?;
        let result = unsafe {
            sys::sceNetInetRecvfrom(
                self.fd,
                buf.as_mut_ptr().cast::<c_void>(),
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
    ///
    /// ///
    /// # Returns
    /// - `Ok(usize)` if the send was successful. The number of bytes sent
    /// - `Err(SocketError)` if the send was unsuccessful.
    ///
    /// # Errors
    /// If the socket is not in state [`UdpSocketState::Connected`] this will return an error.
    /// It may also error if the socket fails to send the data.
    #[allow(unused)]
    pub fn _write_to(
        &mut self,
        buf: &[u8],
        len: usize,
        to: SocketAddr,
    ) -> Result<usize, SocketError> {
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
                buf.as_ptr().cast::<c_void>(),
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

    /// Write to a socket in state [`UdpSocketState::Connected`]
    ///
    /// # Returns
    /// - `Ok(usize)` if the send was successful. The number of bytes sent
    /// - `Err(SocketError)` if the send was unsuccessful.
    ///
    /// # Errors
    /// If the socket is not in state [`UdpSocketState::Connected`] this will return an error.
    /// It may also error if the socket fails to send the data.
    #[allow(unused)]
    pub fn _write(&mut self, buf: &[u8]) -> Result<usize, SocketError> {
        if self.state != UdpSocketState::Connected {
            return Err(SocketError::NotConnected);
        }

        self.buffer.append_buffer(buf);

        self.send()
    }

    /// Flush the send buffer
    ///
    /// # Errors
    /// - [`SocketError::NotConnected`] if the socket is not connected
    /// - Any other [`SocketError`] if the flush was unsuccessful
    pub fn _flush(&mut self) -> Result<(), SocketError> {
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
                self.buffer.as_slice().as_ptr().cast::<c_void>(),
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
    #[must_use]
    pub fn get_state(&self) -> UdpSocketState {
        self.state
    }

    fn socket_len() -> socklen_t {
        core::mem::size_of::<netc::sockaddr>() as u32
    }

    /// Get the file descriptor of the socket
    #[must_use]
    pub fn fd(&self) -> i32 {
        self.fd
    }

    /// Get the remote address of the socket
    #[must_use]
    pub fn remote(&self) -> Option<SocketAddr> {
        self.remote.map(|sockaddr| sockaddr.to_socket_addr())
    }

    /// Get the state of the socket
    #[must_use]
    pub fn state(&self) -> UdpSocketState {
        self.state
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
    type Options<'a> = SocketOptions;
}

impl ErrorType for UdpSocket {
    type Error = SocketError;
}

impl<'a> Open<'a> for UdpSocket {
    /// Open the socket
    fn open(mut self, options: &'a Self::Options<'a>) -> Result<Self, Self::Error> {
        self.bind(None)?;
        self.connect(options.remote())?;

        Ok(self)
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

    /// Flush the socket
    fn flush(&mut self) -> Result<(), Self::Error> {
        match self.get_state() {
            UdpSocketState::Unbound => Err(SocketError::NotBound),
            UdpSocketState::Bound => Err(SocketError::NotConnected),
            UdpSocketState::Connected => self._flush(),
        }
    }
}

impl EasySocket for UdpSocket {}
