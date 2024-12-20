use alloc::{string::String, vec::Vec};

use super::HttpVersion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub http_version: HttpVersion,
    pub status: Code,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Code {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
    Other(u16),
    Unparsable,
}

impl From<Option<u16>> for Code {
    fn from(value: Option<u16>) -> Self {
        match value {
            Some(x) => x.into(),
            None => Code::Unparsable,
        }
    }
}

impl From<Code> for u16 {
    fn from(value: Code) -> Self {
        match value {
            Code::Other(x) => x,
            _ => u16::from(value),
        }
    }
}

impl From<u16> for Code {
    fn from(value: u16) -> Self {
        match value {
            200 => Code::Ok,
            201 => Code::Created,
            202 => Code::Accepted,
            203 => Code::NonAuthoritativeInformation,
            204 => Code::NoContent,
            205 => Code::ResetContent,
            206 => Code::PartialContent,
            300 => Code::MultipleChoices,
            301 => Code::MovedPermanently,
            302 => Code::Found,
            303 => Code::SeeOther,
            304 => Code::NotModified,
            305 => Code::UseProxy,
            307 => Code::TemporaryRedirect,
            308 => Code::PermanentRedirect,
            400 => Code::BadRequest,
            401 => Code::Unauthorized,
            402 => Code::PaymentRequired,
            403 => Code::Forbidden,
            404 => Code::NotFound,
            405 => Code::MethodNotAllowed,
            500 => Code::InternalServerError,
            x => Code::Other(x),
        }
    }
}
