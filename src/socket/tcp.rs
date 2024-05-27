use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_io::{ErrorType, Read, Write};

use embedded_nal::SocketAddr;
use psp::sys;

use core::ffi::c_void;

use crate::traits::io::{EasySocket, Open, OptionType};
use crate::traits::SocketBuffer;
use crate::types::SocketOptions;

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
pub struct TcpSocket {
    /// The socket file descriptor
    fd: i32,
    /// Whether the socket is connected
    is_connected: bool,
    /// The buffer to store data to send
    buffer: Box<dyn SocketBuffer>,
}

impl TcpSocket {
    /// Create a TCP socket
    #[allow(dead_code)]
    pub fn new() -> Result<TcpSocket, SocketError> {
        let fd = unsafe { sys::sceNetInetSocket(netc::AF_INET as i32, netc::SOCK_STREAM, 0) };
        if fd < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(TcpSocket {
                fd,
                is_connected: false,
                buffer: Box::<Vec<u8>>::default(),
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
    #[allow(dead_code)]
    #[allow(clippy::cast_possible_truncation)]
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

    /// Return the underlying socket's file descriptor
    #[allow(unused)]
    pub fn get_socket(&self) -> i32 {
        self.fd
    }

    /// Read from the socket
    fn _read(&self, buf: &mut [u8]) -> Result<usize, SocketError> {
        let result =
            unsafe { sys::sceNetInetRecv(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
        if (result as i32) < 0 {
            Err(SocketError::Errno(unsafe { sys::sceNetInetGetErrno() }))
        } else {
            Ok(result)
        }
    }

    /// Write to the socket
    fn _write(&mut self, buf: &[u8]) -> Result<usize, SocketError> {
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
}

impl Drop for TcpSocket {
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
    type Options = SocketOptions;
}

impl Open for TcpSocket {
    /// Return a TCP socket connected to the remote specified in `options`
    fn open(&mut self, options: Self::Options) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        self.connect(options.remote())?;

        Ok(())
    }
}

impl Read for TcpSocket {
    /// Read from the socket
    fn read<'m>(&'m mut self, buf: &'m mut [u8]) -> Result<usize, Self::Error> {
        if !self.is_connected {
            return Err(SocketError::NotConnected);
        }
        self._read(buf)
    }
}

impl Write for TcpSocket {
    /// Write to the socket
    fn write<'m>(&'m mut self, buf: &'m [u8]) -> Result<usize, Self::Error> {
        if !self.is_connected {
            return Err(SocketError::NotConnected);
        }
        self._write(buf)
    }

    /// Flush the socket
    fn flush(&mut self) -> Result<(), SocketError> {
        self._flush()
    }
}

impl EasySocket for TcpSocket {}
