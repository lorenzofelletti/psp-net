//! This module contains SCE specific types wrappers.

use core::ops::Deref;

use alloc::rc::Rc;
use psp::sys;

/// Raw socket file descriptor
///
/// This is a wrapper around a raw socket file descriptor, which
/// takes care of closing it when no other references to it exist.
///
/// # Notes
/// The drop implementation of this type calls the close syscall.
/// Closing via drop is best-effort as of now (errors are ignored).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RawSocketFileDescriptor(pub(crate) i32);

impl Drop for RawSocketFileDescriptor {
    fn drop(&mut self) {
        unsafe {
            sys::sceNetInetClose(self.0);
        };
    }
}

impl Deref for RawSocketFileDescriptor {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Socket file descriptor
///
/// This is a wrapper around a raw socket file descriptor, which
/// takes care of closing it when no other references to it exist.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketFileDescriptor(pub(crate) Rc<RawSocketFileDescriptor>);

impl SocketFileDescriptor {
    /// Create a new socket file descriptor.
    ///
    /// # Arguments
    /// - `fd`: socket's file descriptor to wrap
    ///
    /// # Safety
    /// - `fd` must be a valid socket file descriptor
    pub(crate) fn new(fd: i32) -> Self {
        Self(Rc::new(RawSocketFileDescriptor(fd)))
    }
}

impl Deref for SocketFileDescriptor {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
