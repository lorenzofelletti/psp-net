pub trait OptionType {
    type Options<'b>: ?Sized;
}

/// Type implementing this trait support a Open semantics.
pub trait Open<'a, 'b>: ErrorType + OptionType {
    type Return;

    /// Open a resource, using options for configuration.
    ///
    /// # Arguments
    /// - `options`: The options to use to configure the TLS connection
    ///
    /// # Errors
    /// This function can return an error if the resource could not be opened.
    ///
    /// # Notes
    /// See [`TlsSocketOptions`](crate::types::TlsSocketOptions) for more information
    /// on the options you can pass.
    fn open(self, options: &'b Self::Options<'b>) -> Result<Self::Return, Self::Error>
    where
        Self: Sized,
        'b: 'a;
}

/// Types implementing this trait support a simplified socket use.
///
/// [`EasySocket`] types support sockets with an Open/Close, Read/Write semantics.
///
/// As usual in Rust, no `close` method is needed, as dropping an object should
/// already close the resources.
///
/// `EasySockets` methods to be used are [`open`](Open::open), [`read`](embedded_io::Read::read),
/// [`write`](embedded_io::Write::write) and [`flush`](embedded_io::Write::flush). Likely, a `new` method is
/// needed befor opening the socket, but this depends on the implementation.
///
/// # Notes
/// [`EasySocket`] types should implement in their [`drop`] method the steps required
/// to close the acquired resources.
pub trait EasySocket: Write + Read {}

// re-exports
pub trait Write = embedded_io::Write;
pub trait Read = embedded_io::Read;
pub trait ErrorType = embedded_io::ErrorType;
