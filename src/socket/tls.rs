#![allow(clippy::module_name_repetitions)]

use alloc::string::String;
use embedded_io::{ErrorType, Read, Write};
use embedded_tls::{blocking::TlsConnection, Aes128GcmSha256, NoVerify, TlsConfig, TlsContext};

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use regex::Regex;

use crate::{
    traits::io::{EasySocket, Open, OptionType},
    types::TlsSocketOptions,
};

use super::{
    state::{Connected, NotReady, Ready, SocketState},
    tcp::TcpSocket,
};

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new("\r|\0").unwrap();
}

/// A TLS socket.
/// This is a wrapper around a [`TcpSocket`] that provides a TLS connection.
pub struct TlsSocket<'a, S: SocketState = NotReady> {
    /// The TLS connection
    tls_connection: TlsConnection<'a, TcpSocket<Connected>, Aes128GcmSha256>,
    /// The TLS config
    tls_config: TlsConfig<'a, Aes128GcmSha256>,
    /// marker for the socket state
    _marker: core::marker::PhantomData<S>,
}

impl<'a> TlsSocket<'_> {
    /// Create a new TLS socket.
    /// This will create a new TLS connection using the provided [`TcpSocket`].
    ///
    /// # Parameters
    /// - `socket`: The TCP socket to use for the TLS connection
    /// - `record_read_buf`: A buffer to use for reading records
    /// - `record_write_buf`: A buffer to use for writing records
    ///
    /// # Returns
    /// A new TLS socket in the [`NotReady`] state. Use [`TlsSocket::open()`] to get a
    /// ready socket.
    ///
    /// # Example
    /// ```no_run
    /// let mut read_buf = TlsSocket::new_buffer();
    /// let mut write_buf = TlsSocket::new_buffer();
    /// let tls_socket = TlsSocket::new(tcp_socket, &mut read_buf, &mut write_buf);
    /// let tls_socket = tls_socket.open(&options)?;
    /// ```
    ///
    /// # Notes
    /// In most cases you can pass `None` for the `cert` parameter.
    pub fn new(
        socket: TcpSocket<Connected>,
        record_read_buf: &'a mut [u8],
        record_write_buf: &'a mut [u8],
    ) -> TlsSocket<'a, NotReady> {
        let tls_config: TlsConfig<'_, Aes128GcmSha256> = TlsConfig::new();

        let tls_connection: TlsConnection<TcpSocket<Connected>, Aes128GcmSha256> =
            TlsConnection::new(socket, record_read_buf, record_write_buf);
        TlsSocket {
            tls_connection,
            tls_config,
            _marker: core::marker::PhantomData,
        }
    }

    /// Create a new buffer.
    /// It is a utility function to create the read/write buffer to pass to [`Self::new()`].
    ///
    /// # Returns
    /// A new buffer of `16_384` bytes.
    ///
    /// # Example
    /// ```no_run
    /// let mut read_buf = TlsSocket::new_buffer();
    /// let mut write_buf = TlsSocket::new_buffer();
    /// let tls_socket = TlsSocket::new(tcp_socket, &mut read_buf, &mut write_buf);
    /// ```
    #[must_use]
    pub fn new_buffer() -> [u8; 16_384] {
        [0; 16_384]
    }
}

impl TlsSocket<'_, Ready> {
    /// Write all data to the TLS connection.
    ///
    /// Writes until all data is written or an error occurs.
    ///
    /// # Parameters
    /// - `buf`: The buffer containing the data to be sent.
    ///
    /// # Returns
    /// - `Ok(())` if the write was successful.
    /// - `Err(TlsError)` if the write was unsuccessful.
    ///
    /// # Errors
    /// [`embedded_tls::TlsError`] if the write fails.
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), embedded_tls::TlsError> {
        self.tls_connection.write_all(buf)
    }

    /// Read data from the TLS connection and converts it to a [`String`].
    ///
    /// # Returns
    /// - `Ok(String)` if the read was successful.
    /// - `Err(TlsError)` if the read was unsuccessful.
    ///
    /// # Errors
    /// [`embedded_tls::TlsError`] if the read fails.
    pub fn read_string(&mut self) -> Result<String, embedded_tls::TlsError> {
        let mut buf = TlsSocket::new_buffer();
        let _ = self.read(&mut buf)?;

        let text = String::from_utf8_lossy(&buf);
        let text = REGEX.replace_all(&text, "");
        Ok(text.into_owned())
    }
}

impl<S: SocketState> ErrorType for TlsSocket<'_, S> {
    /// The error type for the TLS socket.
    type Error = embedded_tls::TlsError;
}

impl<S: SocketState> OptionType for TlsSocket<'_, S> {
    /// The options type for the TLS socket.
    type Options<'b> = TlsSocketOptions<'b>;
}

impl<'a, 'b> Open<'a, 'b> for TlsSocket<'b, NotReady>
where
    'a: 'b,
{
    type Return = TlsSocket<'a, Ready>;
    /// Open the TLS connection.
    ///
    /// # Parameters
    /// - `options`: The TLS options, of type [`TlsSocketOptions`].
    ///
    /// # Returns
    /// A new [`TlsSocket<Ready>`], or an error if opening fails.
    ///
    /// # Example
    /// ```no_run
    /// let tls_socket = TlsSocket::new(tcp_socket, &mut read_buf, &mut write_buf);
    /// let tls_socket = tls_socket.open(&options)?;
    /// ```
    ///
    /// # Notes
    /// The function takes ownership of the socket ([`TcpSocket<NotReady>`]), and returns a new socket of type [`TlsSocket<Ready>`].
    /// Therefore, you must assign the returned socket to a variable in order to use it.
    fn open(self, options: &'b Self::Options<'_>) -> Result<Self::Return, embedded_tls::TlsError>
    where
        'b: 'a,
    {
        let mut rng = ChaCha20Rng::seed_from_u64(options.seed());

        let mut tls_socket: TlsSocket<Ready> = TlsSocket {
            tls_connection: self.tls_connection,
            tls_config: self.tls_config,
            _marker: core::marker::PhantomData,
        };

        tls_socket.tls_config = tls_socket
            .tls_config
            .with_server_name(options.server_name());

        if options.rsa_signatures_enabled() {
            tls_socket.tls_config = tls_socket.tls_config.enable_rsa_signatures();
        }

        if options.reset_max_fragment_length() {
            tls_socket.tls_config = tls_socket.tls_config.reset_max_fragment_length();
        }

        if let Some(cert) = options.cert() {
            tls_socket.tls_config = tls_socket.tls_config.with_cert(cert.clone());
        }

        if let Some(ca) = options.ca() {
            tls_socket.tls_config = tls_socket.tls_config.with_ca(ca.clone());
        }

        let tls_context = TlsContext::new(&tls_socket.tls_config, &mut rng);
        tls_socket
            .tls_connection
            .open::<ChaCha20Rng, NoVerify>(tls_context)?;

        Ok(tls_socket)
    }
}

impl embedded_io::Read for TlsSocket<'_, Ready> {
    /// Read data from the TLS connection.
    ///
    /// # Parameters
    /// - `buf`: The buffer where the data will be stored.
    ///
    /// # Returns
    /// - `Ok(usize)` if the read was successful. The number of bytes read
    /// - `Err(SocketError)` if the read was unsuccessful.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.tls_connection.read(buf)
    }
}

impl embedded_io::Write for TlsSocket<'_, Ready> {
    /// Write data to the TLS connection.
    ///
    /// # Parameters
    /// - `buf`: The buffer containing the data to be sent.
    ///
    /// # Returns
    /// - `Ok(usize)` if the write was successful. The number of bytes written
    /// - `Err(SocketError)` if the write was unsuccessful.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tls_connection.write(buf)
    }

    /// Flush the TLS connection.
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.tls_connection.flush()
    }
}

impl EasySocket for TlsSocket<'_, Ready> {}
