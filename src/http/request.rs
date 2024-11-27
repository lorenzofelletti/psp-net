use core::fmt;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use super::{ContentType, HttpVersion};

/// HTTP request method
///
/// Defaults to [`Method::Get`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Method {
    #[default]
    Get,
    Post,
    Put,
    Delete,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
        }
    }
}

/// HTTP request
///
/// # Fields
/// - [`method`]: HTTP request method
/// - [`http_version`]: HTTP version
/// - [`path`]: HTTP path
/// - [`headers`]: HTTP headers
/// - [`body`]: HTTP body
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub http_version: HttpVersion,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub content_type: Option<ContentType>,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut headers_and_body = String::new();

        for (header, value) in &self.headers {
            headers_and_body.push_str(format!("{header}: {value}\n").as_str());
        }
        if let Some(content_type) = &self.content_type {
            headers_and_body.push_str(format!("Content-Type: {content_type}\n").as_str());
        }
        if !self.body.is_empty() {
            headers_and_body.push_str(format!("Content-Length: {}\n", self.body.len()).as_str());
            headers_and_body
                .push_str(format!("\n{}\n", String::from_utf8_lossy(&self.body)).as_str());
        }

        write!(
            f,
            "{} {} {}\n{}",
            self.method, self.path, self.http_version, headers_and_body
        )
    }
}

impl Request {
    /// Render the request as a vector of bytes
    ///
    /// # Returns
    /// A vector of bytes, representing the request
    pub fn render(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
