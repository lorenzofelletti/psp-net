use alloc::{format, string::String};
use base64::Engine;
use core::fmt;

/// HTTP basic authorization type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicAuthorization {
    /// Provide ID and password. Calling [`to_string`](Self::to_string) will return the encoded string,
    /// or any other method relying on the `Display` trait
    IdPassword(String, String),
    /// Provide the already encoded string "ID:Password"
    Encoded(String),
}

impl BasicAuthorization {
    /// Create a new basic authorization using ID and password.
    ///
    /// In particular, it returns a [`BasicAuthorization::IdPassword`] variant
    /// of the [`BasicAuthorization`] enum.
    #[must_use]
    pub fn new<S>(id: S, password: S) -> Self
    where
        S: Into<String>,
    {
        BasicAuthorization::IdPassword(id.into(), password.into())
    }

    /// Create a new basic authorization using the already encoded string
    ///
    /// In particular, it returns a [`BasicAuthorization::Encoded`] variant
    /// of the [`BasicAuthorization`] enum.
    #[must_use]
    pub fn new_encoded<S>(encoded: S) -> Self
    where
        S: Into<String>,
    {
        BasicAuthorization::Encoded(encoded.into())
    }
}

impl<S: Into<String>> From<S> for BasicAuthorization {
    fn from(id: S) -> Self {
        BasicAuthorization::new_encoded(id)
    }
}

impl fmt::Display for BasicAuthorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasicAuthorization::IdPassword(id, password) => {
                let engine = base64::engine::general_purpose::STANDARD;
                write!(f, "{}", engine.encode(format!("{id}:{password}")))
            }
            BasicAuthorization::Encoded(encoded) => write!(f, "{encoded}"),
        }
    }
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

impl fmt::Display for Authorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Authorization::None => write!(f, ""),
            Authorization::Basic(basic_authorization) => write!(f, "Basic {basic_authorization}"),
            Authorization::Bearer(token) => write!(f, "Bearer {token}"),
            Authorization::Other(s) => write!(f, "{s}"),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString};

    macro_rules! table_tests {
        ($func: expr, $($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                $func($value);
            }
        )*
        }
    }

    fn test_authorization(value: (Authorization, String)) {
        let (authorization, expected) = value;
        let actual = authorization.to_string();
        assert_eq!(actual, expected);
    }
    macro_rules! authorization_tests {
        ($($name:ident: $value:expr,)*) => {
            table_tests!{test_authorization, $($name: $value,)*}
        }
    }

    authorization_tests! {
        none: (Authorization::None, String::new()),
        basic: (Authorization::Basic(BasicAuthorization::new("user", "password")), "Basic dXNlcjpwYXNzd29yZA==".to_string()),
        bearer: (Authorization::Bearer("token".to_string()), "Bearer token".to_string()),
        other: (Authorization::Other("other".to_string()), "other".to_string()),
    }
}
