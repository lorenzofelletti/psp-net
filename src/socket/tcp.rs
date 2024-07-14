use alloc::vec::Vec;
use embedded_io::{ErrorType, Read, Write};

use embedded_nal::SocketAddr;
use psp::sys;

use core::ffi::c_void;

use crate::traits::io::{EasySocket, Open, OptionType};
use crate::traits::SocketBuffer;
use crate::types::{SocketOptions, SocketRecvFlags, SocketSendFlags};

use super::super::netc;

use super::error::SocketError;
use super::ToSockaddr;

/// A TCP socket
///
/// # Safety
/// This is a wrapper around a raw socket file descriptor.
///
/// The socket is closed when the struct is dropped.
/// Closing via drop is best-effort.
///
/// # Notes
/// The structure implements [`EasySocket`]. This allows you to interact with
/// the socket using a simplified API. However, you are still free to use it
/// like a normal Linux socket like you would do in C.
///
/// Using it as an easy socket allows you to use it in the following way:
/// ```no_run
/// use psp::net::TcpSocket;
///
/// let mut socket = TcpSocket::new().unwrap();
/// let socket_options = SocketOptions{ remote: addr };
/// socket.open(socket_options).unwrap();
/// socket.write(b"hello world").unwrap();
/// socket.flush().unwrap();
/// // no need to call close, as drop will do it
/// ```
#[repr(C)]
pub struct TcpSocket<B: SocketBuffer = Vec<u8>> {
    /// The socket file descriptor
    fd: i32,
    /// Whether the socket is connected
    is_connected: bool,
    /// The buffer to store data to send
    buffer: B,
    /// flags for send calls
    send_flags: SocketSendFlags,
    /// flags for recv calls
    recv_flags: SocketRecvFlags,
}

impl TcpSocket {
    /// Create a TCP socket
    ///
    /// # Returns
    /// A new TCP socket
    ///
    /// # Errors
    /// - [`SocketError::Errno`] if the socket could not be created
    #[allow(dead_code)]
    pub fn new() -> Result<TcpSocket, SocketError> {
        let fd = unsafe { sys::sceNetInetSocket(i32::from(netc::AF_INET), netc::SOCK_STREAM, 0) };
        if fd < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(TcpSocket {
                fd,
                is_connected: false,
                buffer: Vec::default(),
                send_flags: SocketSendFlags::empty(),
                recv_flags: SocketRecvFlags::empty(),
            })
        }
    }

    /// Connect to a remote host
    ///
    /// # Parameters
    /// - `remote`: The remote host to connect to
    ///
    /// # Returns
    /// - `Ok(())` if the connection was successful
    /// - `Err(String)` if the connection was unsuccessful.
    ///
    /// # Errors
    /// - [`SocketError::UnsupportedAddressFamily`] if the address family is not supported (only IPv4 is supported)
    /// - Any other [`SocketError`] if the connection was unsuccessful
    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn connect(&mut self, remote: SocketAddr) -> Result<(), SocketError> {
        if self.is_connected {
            return Err(SocketError::AlreadyConnected);
        }
        match remote {
            SocketAddr::V4(v4) => {
                let sockaddr = v4.to_sockaddr();

                if unsafe {
                    sys::sceNetInetConnect(
                        self.fd,
                        &sockaddr,
                        core::mem::size_of::<netc::sockaddr_in>() as u32,
                    )
                } < 0
                {
                    let errno = unsafe { sys::sceNetInetGetErrno() };
                    Err(SocketError::Errno(errno))
                } else {
                    self.is_connected = true;
                    Ok(())
                }
            }
            SocketAddr::V6(_) => Err(SocketError::UnsupportedAddressFamily),
        }
    }

    /// Read from the socket
    ///
    /// # Returns
    /// - `Ok(usize)` if the read was successful. The number of bytes read
    /// - `Err(SocketError)` if the read was unsuccessful.
    ///
    /// # Errors
    /// - A [`SocketError`] if the read was unsuccessful
    ///
    /// # Notes
    /// "Low level" read function. Read data from the socket and store it in
    /// the buffer. This should not be used if you want to use this socket
    /// [`EasySocket`] style.
    pub fn _read(&self, buf: &mut [u8]) -> Result<usize, SocketError> {
        let result = unsafe {
            sys::sceNetInetRecv(
                self.fd,
                buf.as_mut_ptr().cast::<c_void>(),
                buf.len(),
                self.recv_flags.as_i32(),
            )
        };
        if result < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(result as usize)
        }
    }

    /// Write to the socket
    ///
    /// # Errors
    /// - A [`SocketError`] if the write was unsuccessful
    pub fn _write(&mut self, buf: &[u8]) -> Result<usize, SocketError> {
        if !self.is_connected {
            return Err(SocketError::NotConnected);
        }

        self.buffer.append_buffer(buf);
        self.send()
    }

    fn _flush(&mut self) -> Result<(), SocketError> {
        if !self.is_connected {
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
                self.send_flags.as_i32(),
            )
        };
        if result < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            self.buffer.shift_left_buffer(result as usize);
            Ok(result as usize)
        }
    }

    /// Return the underlying socket's file descriptor
    #[must_use]
    pub fn fd(&self) -> i32 {
        self.fd
    }

    /// Return whether the socket is connected
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    /// Flags used when sending data
    #[must_use]
    pub fn send_flags(&self) -> SocketSendFlags {
        self.send_flags
    }

    /// Set the flags used when sending data
    pub fn set_send_flags(&mut self, send_flags: SocketSendFlags) {
        self.send_flags = send_flags;
    }

    /// Flags used when receiving data
    #[must_use]
    pub fn recv_flags(&self) -> SocketRecvFlags {
        self.recv_flags
    }

    /// Set the flags used when receiving data
    pub fn set_recv_flags(&mut self, recv_flags: SocketRecvFlags) {
        self.recv_flags = recv_flags;
    }
}

impl<B: SocketBuffer> Drop for TcpSocket<B> {
    fn drop(&mut self) {
        unsafe {
            sys::sceNetInetClose(self.fd);
        }
    }
}

impl ErrorType for TcpSocket {
    type Error = SocketError;
}

impl OptionType for TcpSocket {
    type Options<'a> = SocketOptions;
}

impl<'a> Open<'a> for TcpSocket {
    type Return<'b> = Self;
    /// Return a TCP socket connected to the remote specified in `options`
    fn open(mut self, options: &'a Self::Options<'a>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        self.connect(options.remote())?;

        Ok(self)
    }
}

impl Read for TcpSocket {
    /// Read from the socket
    ///
    /// # Parameters
    /// - `buf`: The buffer where the read data will be stored
    ///
    /// # Returns
    /// - `Ok(usize)` if the read was successful. The number of bytes read
    /// - `Err(SocketError)` if the read was unsuccessful.
    ///
    /// # Errors
    /// - [`SocketError::NotConnected`] if the socket is not connected
    /// - A [`SocketError`] if the read was unsuccessful
    fn read<'m>(&'m mut self, buf: &'m mut [u8]) -> Result<usize, Self::Error> {
        if !self.is_connected {
            return Err(SocketError::NotConnected);
        }
        self._read(buf)
    }
}

impl Write for TcpSocket {
    /// Write to the socket
    ///
    /// # Errors
    /// - [`SocketError::NotConnected`] if the socket is not connected
    /// - A [`SocketError`] if the write was unsuccessful
    fn write<'m>(&'m mut self, buf: &'m [u8]) -> Result<usize, Self::Error> {
        if !self.is_connected {
            return Err(SocketError::NotConnected);
        }
        self._write(buf)
    }

    /// Flush the socket
    ///
    /// # Errors
    /// - A [`SocketError`] if the flush was unsuccessful
    fn flush(&mut self) -> Result<(), SocketError> {
        self._flush()
    }
}

impl EasySocket for TcpSocket {}
