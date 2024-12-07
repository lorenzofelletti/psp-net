/// Macro helping craft HTTP requests
///
/// By default, the HTTP version is set to 1.1.
///
/// # Examples
/// ## Example GET request
/// ```no_run
/// request! {
///     "www.example.com" get "/",
///     "User-Agent" => "Mozilla/5.0",
/// }
/// ```
///
/// ## Example POST request
/// ```no_run
/// request! {
///     "www.example.com" post "/users/create" ContentType::ApplicationJson,
///     body body,
/// ```
///
/// ## Example With HTTP 1.0
/// ```no_run
/// request! {
///     "www.example.com" get "/"; HttpVersion::V1,
/// }
/// ```
///
/// ## Example With Formatted Header
/// ```no_run
/// request! {
///     "www.example.com" get "/"; HttpVersion::V1,
///     /// enclose the header value in parentheses if it is not
///     /// a string, or more specifically a single token tree (tt).
///     "User-Agent" => (format!("Mozilla/5.0 ({})", "test")),
/// }
/// ```
#[macro_export]
macro_rules! request {
    (
        $host:tt get $path:tt $(; $http_version:expr)?,
        $(authorization $auth:expr,)?
        $($header:expr => $value:expr,)*
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::request::Authorization::None);
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
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::request::Authorization::None);
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
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::request::Authorization::None);
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
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::request::Authorization::None);
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

macro_rules! new_response {
    () => {};
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
