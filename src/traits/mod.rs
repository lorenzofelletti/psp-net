//! Traits implemented by the crate types.

use alloc::vec::Vec;
use core::fmt::Debug;

pub mod dns;
pub mod io;

/// A trait for a buffer that can be used with a socket.
///
/// It can be used by either a read or write buffer of a socket.
pub trait SocketBuffer: Clone + Debug + Default {
    /// Create a new buffer
    fn new() -> Self
    where
        Self: Sized;

    /// Append a buffer to the end.
    ///
    /// # Arguments
    /// - `buf`: buffer containing the data to be appended
    fn append_buffer(&mut self, buf: &[u8]);

    /// Shift the buffer to the left by amount
    ///
    /// This is used to remove data from the buffer.
    fn shift_left_buffer(&mut self, amount: usize);

    /// Clear the buffer
    fn clear(&mut self) {
        self.shift_left_buffer(self.len());
    }

    /// Check if the buffer is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the buffer as a slice
    ///
    /// # Returns
    /// - The buffer as a slice of bytes
    fn as_slice(&self) -> &[u8];

    /// Get the length of the buffer
    fn len(&self) -> usize;
}

impl SocketBuffer for Vec<u8> {
    #[inline]
    fn new() -> Self {
        Vec::new()
    }

    #[inline]
    fn append_buffer(&mut self, buf: &[u8]) {
        self.append(&mut buf.to_vec());
    }

    fn shift_left_buffer(&mut self, amount: usize) {
        // shift the buffer to the left by amount
        if self.len() <= amount {
            self.clear();
        } else {
            self.drain(..amount);
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.as_slice()
    }
}
