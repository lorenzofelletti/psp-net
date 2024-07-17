use bitflags::bitflags;

bitflags! {
    /// Socket flags to use in send calls
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
    pub struct SocketSendFlags: u32 {
        /// No flags passed. Equivalent to `0x0`.
        const NONE = 0x0;
        /// Send out-of-band data
        const MSG_OOB = 0x1;
        /// End of record
        const MSG_EOR = 0x8;
    }
}

impl SocketSendFlags {
    /// Convert a [`SocketSendFlags`] into a [`i32`]
    #[must_use]
    pub fn as_i32(self) -> i32 {
        self.bits() as i32
    }
}

bitflags! {
    /// Socket flags to use in recv calls
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
    pub struct SocketRecvFlags: u32 {
        /// No flags passed. Equivalent to `0x0`.
        const NONE = 0x0;
        /// Process out-of-band data
        const MSG_OOB = 0x1;
        /// Peek at the incoming message
        const MSG_PEEK = 0x2;
        /// Wait for full message
        const MSG_WAITALL = 0x40;
    }
}

impl SocketRecvFlags {
    /// Convert a [`SocketRecvFlags`] into a [`i32`]
    #[must_use]
    pub fn as_i32(self) -> i32 {
        self.bits() as i32
    }
}
