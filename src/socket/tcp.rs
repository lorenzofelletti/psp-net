use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_io::ErrorType;

use embedded_nal::SocketAddr;
use psp::sys;

use core::ffi::c_void;

use crate::traits::io::{EasySocket, Open, OptionType};
use crate::traits::SocketBuffer;
use crate::types::SocketOptions;

use super::super::netc;

use super::error::SocketError;
use super::ToSockaddr;

// TODO: review implementation
#[repr(C)]
/// A TCP socket
///
/// # Fields
/// - [`Self::fd`]: The socket file descriptor
/// - [`Self::is_connected`]: Whether the socket is connected
/// - [`Self::buffer`]: The buffer to store data to send
///
/// # Safety
/// This is a wrapper around a raw socket file descriptor.
///
/// The socket is closed when the struct is dropped.
pub struct TcpSocket {
    fd: i32,
    is_connected: bool,
    buffer: Box<dyn SocketBuffer>,
}

impl TcpSocket {
    #[allow(dead_code)]
    /// Create a TCP socket
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

    #[allow(dead_code)]
    /// Connect to a remote host
    ///
    /// # Parameters
    /// - `remote`: The remote host to connect to
    ///
    /// # Returns
    /// - `Ok(())` if the connection was successful
    /// - `Err(String)` if the connection was unsuccessful.
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
            psp::dprintln!("Flushing");
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
    fn open(options: Self::Options) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut socket = Self::new()?;
        socket.connect(options.remote())?;

        Ok(socket)
    }
}

impl embedded_io::Read for TcpSocket {
    /// Read from the socket
    fn read<'m>(&'m mut self, buf: &'m mut [u8]) -> Result<usize, Self::Error> {
        if !self.is_connected {
            return Err(SocketError::NotConnected);
        }
        self._read(buf)
    }
}

impl embedded_io::Write for TcpSocket {
    /// Write to the socket
    fn write<'m>(&'m mut self, buf: &'m [u8]) -> Result<usize, Self::Error> {
        if !self.is_connected {
            return Err(SocketError::NotConnected);
        }
        self._write(buf)
    }

    fn flush(&mut self) -> Result<(), SocketError> {
        self._flush()
    }
}

impl EasySocket for TcpSocket {}
