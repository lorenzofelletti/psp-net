use embedded_io::{ErrorType, Read, Write};

pub trait OptionType {
    type Options: ?Sized;
}

/// Type implementing this trait support a Open semantics.
pub trait Open: ErrorType + OptionType {
    /// Open a resource, using options for configuration.
    fn open(&mut self, options: Self::Options) -> Result<(), Self::Error>
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
/// EasySockets methods to be used are [`open`](Open::open), [`read`](Read::read),
/// [`write`](Write::write) and [`flush`](Write::flush). Likely, a `new` method is
/// needed befor opening the socket, but this depends on the implementation.
///
/// # Notes
/// [`EasySocket`] types should implement in their [`drop`] method the steps required
/// to close the acquired resources.
pub trait EasySocket: Open + Write + Read {}
