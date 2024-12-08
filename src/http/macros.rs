/// Macro helping craft HTTP requests
///
/// By default, the HTTP version is set to 1.1.
///
/// # Examples
/// ## Example GET request
/// ```no_run
/// # extern crate alloc;
/// # use psp_net::request;
/// # use alloc::string::String;
/// let req = request! {
///     "www.example.com" get "/",
///     "User-Agent" => "Mozilla/5.0",
/// };
/// ```
///
/// ## Example POST request
/// ```no_run
/// # extern crate alloc;
/// # use psp_net::request;
/// # use alloc::string::String;
/// # let body = "test".to_string();
/// let req = request! {
///     "www.example.com" post "/users/create" ContentType::ApplicationJson,
///     body body
/// };
/// ```
///
/// ## Example With HTTP 1.0
/// ```no_run
/// # extern crate alloc;
/// # use psp_net::request;
/// # use alloc::string::String;
/// # use psp_net::http::HttpVersion;
/// let req = request! {
///     "www.example.com" get "/"; HttpVersion::V1,
/// };
/// ```
///
/// ## Example With Formatted Header
/// ```no_run
/// # extern crate alloc;
/// # use psp_net::request;
/// # use alloc::string::String;
/// let req = request! {
///     "www.example.com" get "/",
///     /// enclose the header value in parentheses if it is not
///     /// a string, or more specifically a single token tree (tt).
///     "User-Agent" => (format!("Mozilla/5.0 ({})", "test")),
/// };
/// ```
#[macro_export]
macro_rules! request {
    (
        $host:tt get $path:tt $(; $http_version:expr)?,
        $($header:expr => $value:expr,)*
    ) => {
        {
            use alloc::string::{String, ToString};
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            use psp_net::some_or_none;
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::_request! {
                http_version http_ver,
                host $host,
                path $path,
                method $crate::http::Method::Get,
                auth $crate::http::types::Authorization::None,
                $($header => $value,)*
            }
        }
    };

    (
        $host:tt get $path:tt $(; $http_version:expr)?,
        $(authorization $auth:expr,)?
        $($header:expr => $value:expr,)*
    ) => {
        {
            use alloc::string::{String, ToString};
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            use psp_net::some_or_none;
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::types::Authorization::None);
            $crate::http::Request {
                method: $crate::http::Method::Get,
                path: $path.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                authorization: auth,
                content_type: None,
                body: Vec::new(),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };

    (
        $host:tt post $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        $(authorization $auth:expr,)?
        $($header:tt => $value:tt),*
        $(body $body:expr)?
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            use psp_net::some_or_none;
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::types::Authorization::None);
            $crate::http::Request {
                method: $crate::http::Method::Post,
                path: $path.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                authorization: auth,
                content_type: $crate::some_or_none!($($content_type)?),
                body: $crate::some_or_none!($($body)?).unwrap_or(Vec::new()),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };

    (
        $host:tt put $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        $(authorization $auth:expr,)?
        $($header:tt => $value:tt),*
        $(body $body:expr)?
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            use psp_net::some_or_none;
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::types::Authorization::None);
            $crate::http::Request {
                method: $crate::http::Method::Put,
                path: $path.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                authorization: auth,
                content_type: $crate::some_or_none!($($content_type)?),
                body: $crate::some_or_none!($($body)?).unwrap_or(Vec::new()),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };

    (
        $host:tt delete $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        $(authorization $auth:expr,)?
        $($header:tt => $value:tt),*
        $(body $body:expr)?
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            use psp_net::some_or_none;
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::types::Authorization::None);
            $crate::http::Request {
                method: $crate::http::Method::Delete,
                path: $path.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                authorization: auth,
                content_type: $crate::some_or_none!($($content_type)?),
                body: $crate::some_or_none!($($body)?).unwrap_or(Vec::new()),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };
}

#[macro_export]
macro_rules! parse_response {
    (
        $response:expr,
        $(max_headers $max_headers:tt,)?
    ) => {{
        use alloc::vec;
        let me = $crate::some_or_none!($($max_headers)?).unwrap_or(16);
        let mut headers = vec![httparse::EMPTY_HEADER; me];

        let mut res = httparse::Response::new(&mut headers);

        let parsed =
            httparse::ParserConfig::default().parse_response(&mut res, $response.as_bytes());
        parsed
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _request {
    // no body, no content type
    (
        http_version $http_version:expr,
        host $host:tt,
        path $path:tt,
        method $method:expr,
        auth $auth:expr,
        $($header:expr => $value:expr,)*
    ) => {
        $crate::http::Request {
            method: $crate::http::Method::Get,
            path: $path.to_string(),
            headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
            authorization: $auth,
            content_type: None,
            body: Vec::new(),
            http_version: $http_version,
        }
    };
    // body, no content type
    (
        http_version $http_version:expr,
        host $host:tt,
        path $path:tt,
        method $method:expr,
        auth $auth:expr,
        body $body:expr,
        $($header:expr => $value:expr,)*
    ) => {
        $crate::http::Request {
            method: $method,
            path: $path.to_string(),
            headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
            authorization: $auth,
            content_type: None,
            body: $body,
            http_version: $http_version,
        }
    };
    // body and content type
    (
        http_version $http_version:expr,
        host $host:tt,
        path $path:tt,
        method $method:expr,
        auth $auth:expr,
        content_type $content_type:expr,
        body $body:expr,
        $($header:expr => $value:expr,)*
    ) => {
        $crate::http::Request {
            method: $method,
            path: $path.to_string(),
            headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
            authorization: $auth,
            content_type: $content_type,
            body: $body,
            http_version: $http_version,
        }
    };
}
