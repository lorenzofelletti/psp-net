use core::fmt::Debug;

/// Trait describing the state of a socket
pub trait SocketState: Debug {}

/// Socket is in an unbound state
#[derive(Debug)]
pub struct Unbound;
impl SocketState for Unbound {}

/// Socket is in a bound state
#[derive(Debug)]
pub struct Bound;
impl SocketState for Bound {}

/// Socket is in a connected state
#[derive(Debug)]
pub struct Connected;
impl SocketState for Connected {}

/// Socket is not ready to send or receive data
#[derive(Debug)]
pub struct NotReady;
impl SocketState for NotReady {}

/// Socket is ready to send or receive data
#[derive(Debug)]
pub struct Ready;
impl SocketState for Ready {}
