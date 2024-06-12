pub trait OptionType {
    type Options<'b>: ?Sized;
}

/// Type implementing this trait support a Open semantics.
pub trait Open<'a>: ErrorType + OptionType {
    /// Open a resource, using options for configuration.
    ///
    /// # Errors
    /// This function can return an error if the resource could not be opened.
    fn open(self, options: &'a Self::Options<'a>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Types implementing this trait support a simplified socket use.
///
/// [`EasySocket`] types support sockets with an Open/Close, Read/Write semantics.
///
/// As usual in Rust, no `close` method is needed, as dropping an object should
/// already close the resources.
///
/// `EasySockets` methods to be used are [`open`](Open::open), [`read`](Read::read),
/// [`write`](Write::write) and [`flush`](Write::flush). Likely, a `new` method is
/// needed befor opening the socket, but this depends on the implementation.
///
/// # Notes
/// [`EasySocket`] types should implement in their [`drop`] method the steps required
/// to close the acquired resources.
pub trait EasySocket: for<'a> Open<'a> + Write + Read {}

// re-exports
pub trait Write = embedded_io::Write;
pub trait Read = embedded_io::Read;
pub trait ErrorType = embedded_io::ErrorType;
