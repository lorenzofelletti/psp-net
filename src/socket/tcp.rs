#![allow(clippy::module_name_repetitions)]

use alloc::vec::Vec;
use embedded_io::{ErrorType, Read, Write};

use core::net::SocketAddr;
use psp::sys;

use core::ffi::c_void;

use crate::traits::io::{EasySocket, Open, OptionType};
use crate::traits::SocketBuffer;
use crate::types::{SocketOptions, SocketRecvFlags, SocketSendFlags};

use super::super::netc;

use super::error::SocketError;
use super::sce::SocketFileDescriptor;
use super::state::{Connected, SocketState, Unbound};
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
/// let socket = TcpSocket::new().unwrap();
/// let socket_options = SocketOptions{ remote: addr };
/// let socket = socket.open(socket_options).unwrap();
/// socket.write(b"hello world").unwrap();
/// socket.flush().unwrap();
/// // no need to call close, as drop will do it
/// ```
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TcpSocket<S: SocketState = Unbound, B: SocketBuffer = Vec<u8>> {
    /// The socket file descriptor
    pub(super) fd: SocketFileDescriptor,
    /// The buffer to store data to send
    buffer: B,
    /// flags for send calls
    send_flags: SocketSendFlags,
    /// flags for recv calls
    recv_flags: SocketRecvFlags,
    /// marker for the socket state
    _marker: core::marker::PhantomData<S>,
}

impl TcpSocket {
    /// Create a TCP socket
    ///
    /// # Returns
    /// A new TCP socket
    ///
    /// # Errors
    /// - [`SocketError::ErrnoWithDescription`] if the socket could not be created
    pub fn new() -> Result<TcpSocket<Unbound>, SocketError> {
        let fd = unsafe { sys::sceNetInetSocket(i32::from(netc::AF_INET), netc::SOCK_STREAM, 0) };
        if fd < 0 {
            Err(SocketError::new_errno_with_description(
                unsafe { sys::sceNetInetGetErrno() },
                "failed to create socket",
            ))
        } else {
            let fd = SocketFileDescriptor::new(fd);
            Ok(TcpSocket {
                fd,
                buffer: Vec::with_capacity(0),
                send_flags: SocketSendFlags::empty(),
                recv_flags: SocketRecvFlags::empty(),
                _marker: core::marker::PhantomData,
            })
        }
    }
}

impl<S: SocketState> TcpSocket<S> {
    /// Return the underlying socket's file descriptor
    #[must_use]
    pub fn fd(&self) -> i32 {
        *self.fd
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

impl TcpSocket<Unbound> {
    #[must_use]
    fn transition(self) -> TcpSocket<Connected> {
        TcpSocket {
            fd: self.fd,
            buffer: Vec::default(),
            send_flags: self.send_flags,
            recv_flags: self.recv_flags,
            _marker: core::marker::PhantomData,
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
    pub fn connect(self, remote: SocketAddr) -> Result<TcpSocket<Connected>, SocketError> {
        match remote {
            SocketAddr::V4(v4) => {
                let sockaddr = v4.to_sockaddr();

                if unsafe {
                    sys::sceNetInetConnect(
                        *self.fd,
                        &sockaddr,
                        core::mem::size_of::<netc::sockaddr_in>() as u32,
                    )
                } < 0
                {
                    let errno = unsafe { sys::sceNetInetGetErrno() };
                    Err(SocketError::Errno(errno))
                } else {
                    Ok(self.transition())
                }
            }
            SocketAddr::V6(_) => Err(SocketError::UnsupportedAddressFamily),
        }
    }
}

impl TcpSocket<Connected> {
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
    pub fn internal_read(&self, buf: &mut [u8]) -> Result<usize, SocketError> {
        let result = unsafe {
            sys::sceNetInetRecv(
                *self.fd,
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
    pub fn internal_write(&mut self, buf: &[u8]) -> Result<usize, SocketError> {
        self.buffer.append_buffer(buf);
        self.send()
    }

    fn internal_flush(&mut self) -> Result<(), SocketError> {
        while !self.buffer.is_empty() {
            self.send()?;
        }
        Ok(())
    }

    fn send(&mut self) -> Result<usize, SocketError> {
        let result = unsafe {
            sys::sceNetInetSend(
                *self.fd,
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
}

impl<S: SocketState> ErrorType for TcpSocket<S> {
    type Error = SocketError;
}

impl<S: SocketState> OptionType for TcpSocket<S> {
    type Options<'a> = SocketOptions;
}

impl Open<'_, '_> for TcpSocket<Unbound> {
    type Return = TcpSocket<Connected>;
    /// Return a TCP socket connected to the remote specified in `options`
    fn open(self, options: &'_ Self::Options<'_>) -> Result<Self::Return, Self::Error>
    where
        Self: Sized,
    {
        let socket = self.connect(options.remote())?;
        Ok(socket)
    }
}

impl Read for TcpSocket<Connected> {
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
        self.internal_read(buf)
    }
}

impl Write for TcpSocket<Connected> {
    /// Write to the socket
    ///
    /// # Errors
    /// - [`SocketError::NotConnected`] if the socket is not connected
    /// - A [`SocketError`] if the write was unsuccessful
    fn write<'m>(&'m mut self, buf: &'m [u8]) -> Result<usize, Self::Error> {
        self.internal_write(buf)
    }

    /// Flush the socket
    ///
    /// # Errors
    /// - A [`SocketError`] if the flush was unsuccessful
    fn flush(&mut self) -> Result<(), SocketError> {
        self.internal_flush()
    }
}

impl EasySocket for TcpSocket<Connected> {}
