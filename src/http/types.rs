use alloc::string::String;
use core::fmt;

/// HTTP basic authorization type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicAuthorization {
    /// Provide ID and password
    IdPassword(String, String),
    /// Provide the already encoded string "ID:Password"
    Encoded(String),
}

/// HTTP authorization type
///
/// Defaults to [`Authorization::Basic`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Authorization {
    #[default]
    /// No authorization
    None,
    /// Basic authorization
    ///
    /// # Fields
    /// - first: ID
    /// - second: Password
    Basic(BasicAuthorization),
    /// Bearer authorization
    ///
    /// # Fields
    /// - first: Bearer token
    Bearer(String),
    /// Any other authorization, as a string
    Other(String),
}

/// Content Type of the HTTP packet's body.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ContentType {
    #[default]
    /// text/plain
    TextPlain,
    /// application/json
    ApplicationJson,
    /// application/octet-stream
    OctetStream,
    /// Any other content type, as a string
    Other(String),
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::TextPlain => write!(f, "text/plain"),
            ContentType::ApplicationJson => write!(f, "application/json"),
            ContentType::OctetStream => write!(f, "application/octet-stream"),
            ContentType::Other(s) => write!(f, "{s}"),
        }
    }
}
