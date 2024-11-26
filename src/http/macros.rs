/// Macro helping craft HTTP requests
///
/// # Example
/// Example GET request
/// ```no_run
/// request! {
///     "www.example.com" get "/",
///     "User-Agent" => "Mozilla/5.0",
/// }
/// ```
///
/// Example POST request
/// ```no_run
/// request! {
///     "www.example.com" post "/users/create" ContentType::ApplicationJson,
///     body body,
/// ```
#[macro_export]
macro_rules! request {
    (
        $host:tt get $uri:tt $(; $http_version:expr)?,
        $($header:expr => $value:expr,)*
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            $crate::http::Request {
                method: $crate::http::Method::Get,
                uri: $uri.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                content_type: None,
                body: Vec::new(),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };

    (
        $host:tt post $uri:tt $($content_type:expr)? $(; $http_version:expr)?,
        $($header:tt => $value:tt),*
        $(body $body:expr)?
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            $crate::http::Request {
                method: $crate::http::Method::Post,
                uri: $uri.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                content_type: $crate::some_or_none!($($content_type)?),
                body: $crate::some_or_none!($($body)?).unwrap_or(Vec::new()),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };

    (
        $host:tt put $uri:tt $($content_type:expr)? $(; $http_version:expr)?,
        $($header:tt => $value:tt),*
        $(body $body:expr)?
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            $crate::http::Request {
                method: $crate::http::Method::Put,
                uri: $uri.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                content_type: $crate::some_or_none!($($content_type)?),
                body: $crate::some_or_none!($($body)?).unwrap_or(Vec::new()),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };

    (
        $host:tt delete $uri:tt $($content_type:expr)? $(; $http_version:expr)?,
        $($header:tt => $value:tt),*
        $(body $body:expr)?
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            $crate::http::Request {
                method: $crate::http::Method::Delete,
                uri: $uri.to_string(),
                headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
                content_type: $crate::some_or_none!($($content_type)?),
                body: $crate::some_or_none!($($body)?).unwrap_or(Vec::new()),
                http_version: $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1),
            }
        }
    };
}
