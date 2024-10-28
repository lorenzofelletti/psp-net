///
/// # Example
/// Example GET request
/// ```no_run
/// request! {
///     host "www.example.com",
///     GET > "/",
///     "User-Agent" => "Mozilla/5.0",
/// }
/// ```
///
/// Example POST request
/// ```no_run
/// request! {
///     host "www.example.com",
///     POST > "/users/create",
///     content_type "application/json",
///     body body,
#[macro_export]
macro_rules! request {
    (
        host $host:expr,
        GET  $uri:expr,
        $($header:expr => $value:expr,)*
    ) => {
        request! {
            host $host,
            $crate::http::Method::Get => $uri,
            $($header => $value,)*
        }
    };

    (
        host $host:expr,
        POST $uri:expr,
        $(content_type $content_type:expr,)?
        $($header:expr => $value:expr,)*
        $(body $body:expr)?
    ) => {
        request! {
            host $host,
            $crate::http::Method::Post => $uri,
            $(content_type $content_type,)?
            $($header => $value,)*
            $(body $body)?
        }
    };

    (
        host $host:expr,
        PUT  $uri:expr,
        $(content_type $content_type:expr,)?
        $($header:expr => $value:expr,)*
        $(body $body:expr)?
    ) => {
        request! {
            host $host,
            $crate::http::Method::Put => $uri,
            $(content_type $content_type,)?
            $($header => $value,)*
            $(body $body)?
        }
    };

    (
        host $host:expr,
        DELETE  $uri:expr,
        $(content_type $content_type:expr,)?
        $($header:expr => $value:expr,)*
    ) => {
        request! {
            host $host,
            $crate::http::Method::Delete => $uri,
            $(content_type $content_type,)?
            $($header => $value,)*
            $(body $body)?
        }
    };

    (
        host $host:expr,
        $method:expr => $uri:expr,
        $(content_type $content_type:expr,)?
        $($header:expr => $value:expr,)*
        $(body $body:expr)?
    ) => {{
        use alloc::string::ToString;
        use alloc::vec as a_vec;
        $crate::http::Request {
            method: $method,
            uri: $uri.to_string(),
            headers: a_vec![$(($header.to_string(), $value.to_string()),)*],
            content_type: $crate::some_or_none!($($content_type)?),
            body: $crate::some_or_none!($($body)?).unwrap_or(Vec::new()),
        }
    }};
}
