use alloc::{string::String, vec::Vec};
use bitflags::bitflags;

use crate::parse_response;

use super::HttpVersion;

pub struct Response {
    pub http_version: HttpVersion,
    pub status: Code,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

bitflags! {
    pub struct XCodes: u16 {
        const Ok = 200;
        const Created = 201;
        const Accepted = 202;
        const NonAuthoritativeInformation = 203;
        const NoContent = 204;
        const ResetContent = 205;
        const PartialContent = 206;
        const MultipleChoices = 300;
        const MovedPermanently = 301;
        const Found = 302;
        const SeeOther = 303;
        const NotModified = 304;
        const UseProxy = 305;
        const TemporaryRedirect = 307;
        const PermanentRedirect = 308;
        const BadRequest = 400;
        const Unauthorized = 401;
        const PaymentRequired = 402;
        const Forbidden = 403;
        const NotFound = 404;
        const MethodNotAllowed = 405;
        const InternalServerError = 500;
    }
}

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

fn parse_responsex(response: String) {
    let resx = response.clone();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut res = httparse::Response::new(&mut headers);

    let r = httparse::ParserConfig::default()
        .allow_multiple_spaces_in_response_status_delimiters(true)
        .parse_response(&mut res, response.as_bytes())
        .unwrap();
    if r.is_partial() {
        panic!("Partial response");
    } else {
        res.code;
    }

    let x = parse_response! {
        resx,
        max_headers 16,
    };
    if let Ok(x) = x {
        if x.is_complete() {
            res.code;
            if let Code::Ok = Code::from(res.code.unwrap()) {}
        }
    }
}
