use alloc::string::String;
use embedded_io::{ErrorType, Read, Write};
use embedded_tls::{
    blocking::TlsConnection, Aes128GcmSha256, Certificate, NoVerify, TlsConfig, TlsContext,
};

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use regex::Regex;

use crate::{
    traits::io::{Open, OptionType},
    types::TlsSocketOptions,
};

use super::tcp::TcpSocket;

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new("\r|\0").unwrap();
}

/// A TLS socket.
/// This is a wrapper around a [`TcpSocket`] that provides a TLS connection.
pub struct TlsSocket<'a> {
    /// The TLS connection
    tls_connection: TlsConnection<'a, TcpSocket, Aes128GcmSha256>,
    /// The TLS config
    tls_config: TlsConfig<'a, Aes128GcmSha256>,
}

impl<'a> TlsSocket<'a> {
    /// Create a new TLS socket.
    /// This will create a new TLS connection using the provided [`TcpSocket`].
    ///
    /// # Parameters
    /// - `socket`: The TCP socket to use for the TLS connection
    /// - `record_read_buf`: A buffer to use for reading records
    /// - `record_write_buf`: A buffer to use for writing records
    /// - `server_name`: The server name to connect to (e.g. "example.com")
    /// - `cert`: An optional certificate to use for the connection
    ///
    /// # Returns
    /// A new TLS socket.
    ///
    /// The returned connection is not ready yet.
    /// You must call [`Self::open()`] before you can start sending/receiving data.
    ///
    /// # Example
    /// ```no_run
    /// let mut read_buf = TlsSocket::new_buffer();
    /// let mut write_buf = TlsSocket::new_buffer();
    /// let tls_socket = TlsSocket::new(tcp_socket, &mut read_buf, &mut write_buf, "example.com", None);
    /// ```
    ///
    /// # Notes
    /// In most cases you can pass `None` for the `cert` parameter.
    pub fn new(
        socket: TcpSocket,
        record_read_buf: &'a mut [u8],
        record_write_buf: &'a mut [u8],
        server_name: &'a str,
        cert: Option<&'a [u8]>,
    ) -> Self {
        let tls_config: TlsConfig<'_, Aes128GcmSha256> = match cert {
            Some(cert) => TlsConfig::new()
                .with_server_name(server_name)
                .with_cert(Certificate::RawPublicKey(cert))
                .enable_rsa_signatures(),
            None => TlsConfig::new()
                .with_server_name(server_name)
                .enable_rsa_signatures(),
        };

        let tls_connection: TlsConnection<TcpSocket, Aes128GcmSha256> =
            TlsConnection::new(socket, record_read_buf, record_write_buf);

        TlsSocket {
            tls_connection,
            tls_config,
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
    /// let tls_socket = TlsSocket::new(tcp_socket, &mut read_buf, &mut write_buf, "example.com", None);
    /// ```
    pub fn new_buffer() -> [u8; 16_384] {
        [0; 16_384]
    }

    /// Write all data to the TLS connection.
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), embedded_tls::TlsError> {
        self.tls_connection.write_all(buf)
    }

    /// Read data from the TLS connection and converts it to a [`String`].
    pub fn read_string(&mut self) -> Result<String, embedded_tls::TlsError> {
        let mut buf = Self::new_buffer();
        let _ = self.read(&mut buf)?;

        let text = String::from_utf8_lossy(&buf);
        let text = REGEX.replace_all(&text, "");
        Ok(text.into_owned())
    }
}

impl ErrorType for TlsSocket<'_> {
    type Error = embedded_tls::TlsError;
}

impl OptionType for TlsSocket<'_> {
    type Options = TlsSocketOptions;
}

impl Open for TlsSocket<'_> {
    /// Open the TLS connection.
    fn open(&mut self, options: Self::Options) -> Result<(), embedded_tls::TlsError> {
        let mut rng = ChaCha20Rng::seed_from_u64(options.seed());
        let tls_context = TlsContext::new(&self.tls_config, &mut rng);
        self.tls_connection
            .open::<ChaCha20Rng, NoVerify>(tls_context)
    }
}

impl embedded_io::Read for TlsSocket<'_> {
    /// Read data from the TLS connection.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.tls_connection.read(buf)
    }
}

impl embedded_io::Write for TlsSocket<'_> {
    /// Write data to the TLS connection.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tls_connection.write(buf)
    }

    /// Flush the TLS connection.
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.tls_connection.flush()
    }
}
