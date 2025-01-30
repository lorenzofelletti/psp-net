/// Macro helping craft HTTP requests
///
/// By default, the HTTP version is set to 1.1.
///
/// # Examples
/// ## Example GET request
/// ```
/// # extern crate alloc;
/// # use psp_net::request;
/// # use alloc::string::String;
/// let req = request! {
///     "www.example.com" get "/",
///     "User-Agent" => "Mozilla/5.0";
/// };
/// ```
///
/// ## Example POST request
/// ```
/// # extern crate alloc;
/// # use psp_net::{request, http::types::ContentType};
/// # use alloc::string::String;
/// # let body = "test".as_bytes().to_vec();
/// let req = request! {
///     "www.example.com" post "/users/create" ContentType::ApplicationJson,
///     body body,
///     "User-Agent" => (format!("Mozilla/5.0 ({})", "test"));
/// };
/// ```
///
/// ## Example With HTTP 1.0
/// ```
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
/// ```
/// # extern crate alloc;
/// # use psp_net::request;
/// # use alloc::string::String;
/// let req = request! {
///     "www.example.com" get "/",
///     /// enclose the header value in parentheses if it is not
///     /// a string, or more specifically a single token tree (tt).
///     "User-Agent" => (format!("Mozilla/5.0 ({})", "test"));
/// };
/// ```
#[macro_export]
macro_rules! request {
    // get, no authorization
    (
        $host:tt get $path:tt $(; $http_version:expr)?,
        $($header:expr => $value:expr;)*
    ) => {
        {
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::_request! {
                http_version http_ver,
                host $host,
                path $path,
                method $crate::http::Method::Get,
                auth $crate::http::types::Authorization::None,
                body Vec::new(),
                $($header => $value;)*
            }
        }
    };

    // get, with authorization
    (
        $host:tt get $path:tt $(; $http_version:expr)?,
        authorization $auth:expr,
        $($header:expr => $value:expr;)*
    ) => {
        {
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::_request! {
                http_version http_ver,
                host $host,
                path $path,
                method $crate::http::Method::Get,
                auth $auth,
                body Vec::new(),
                $($header => $value;)*
            }
        }
    };

    // post, no authorization
    (
        $host:tt post $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        body $body:expr,
        $($header:expr => $value:expr;)*
    ) => {
        {
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::_request! {
                http_version http_ver,
                host $host,
                path $path,
                method $crate::http::Method::Post,
                auth $crate::http::types::Authorization::None,
                content_type $crate::some_or_none!($($content_type)?),
                body $body,
                $($header => $value;)*
            }
        }
    };

    // post, with authorization
    (
        $host:tt post $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        authorization $auth:expr,
        body $body:expr,
        $($header:expr => $value:expr;)*
    ) => {
        {
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::_request! {
                http_version http_ver,
                host $host,
                path $path,
                method $crate::http::Method::Post,
                auth $auth,
                content_type $crate::some_or_none!($($content_type)?),
                body $body,
                $($header => $value;)*
            }
        }
    };

    // put, no authorization
    (
        $host:tt put $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        $($header:expr => $value:expr;)*
        $(body $body:expr)?
    ) => {
        {
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::types::Authorization::None);
            let body = $crate::some_or_none!($($body)?).unwrap_or(Vec::new());
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::http::Request {
                http_version: http_ver,
                host: $host,
                path: $path,
                method: $crate::http::Method::Put,
                authorization: $crate::http::types::Authorization::None,
                content_type: $crate::some_or_none!($($content_type)?),
                body: $body,
                $($header => $value;)*
            }
        }
    };

    // put, with authorization
    (
        $host:tt put $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        authorization $auth:expr,
        $($header:expr => $value:expr;)*
        body $body:expr,
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            use psp_net::some_or_none;
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::types::Authorization::None);
            let body = $crate::some_or_none!($($body)?).unwrap_or(Vec::new());
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::http::Request {
                http_version: http_ver,
                host: $host,
                path: $path,
                method: $crate::http::Method::Put,
                authorization: $auth,
                content_type: $crate::some_or_none!($($content_type)?),
                body: $body,
                $($header => $value;)*
            }
        }
    };

    // delete, no authorization
    (
        $host:tt delete $path:tt $($content_type:expr)? $(; $http_version:expr)?,
        $($header:expr => $value:expr;)*
        $(body $body:expr)?
    ) => {
        {
            use alloc::string::ToString;
            use alloc::vec::Vec;
            use alloc::vec as a_vec;
            use psp_net::some_or_none;
            let auth = some_or_none!($($auth)?).unwrap_or($crate::http::types::Authorization::None);
            let http_ver = $crate::some_or_none!($($http_version)?).unwrap_or($crate::http::HttpVersion::V1_1);
            $crate::http::Request {
                http_version: http_ver,
                host: $host,
                path: $path,
                method: $crate::http::Method::Delete,
                authorization: $auth,
                content_type: $crate::some_or_none!($($content_type)?),
                body: body,
                $($header => $value;)*
            }
        }
    };

    // delete, with authorization
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

/// Macro creating an empty `Vec<u8>` vector
#[macro_export]
macro_rules! empty_body {
    () => {
        Vec::<u8>::new()
    };
}

/// Internal macro to create a [`crate::http::Request`].
/// It is not intended to be used directly, but serves as a support to [`request!`] macro.
#[macro_export]
#[doc(hidden)]
macro_rules! _request {
    // no content type
    (
        http_version $http_version:expr,
        host $host:tt,
        path $path:tt,
        method $method:expr,
        auth $auth:expr,
        body $body:expr,
        $($header:expr => $value:expr;)*
    ) => {{
        use alloc::string::ToString;
        use alloc::vec as a_vec;
        $crate::http::Request {
            method: $method,
            path: $path.to_string(),
            headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
            authorization: $auth,
            content_type: None,
            body: $body,
            http_version: $http_version,
        }
    }};
    // content type
    (
        http_version $http_version:expr,
        host $host:tt,
        path $path:tt,
        method $method:expr,
        auth $auth:expr,
        content_type $content_type:expr,
        body $body:expr,
        $($header:expr => $value:expr;)*
    ) => {{
        use alloc::string::ToString;
        use alloc::vec as a_vec;
        $crate::http::Request {
            method: $method,
            path: $path.to_string(),
            headers: a_vec![("Host".to_string(), $host.to_string()), $(($header.to_string(), $value.to_string()),)*],
            authorization: $auth,
            content_type: $content_type,
            body: $body,
            http_version: $http_version,
        }
    }};
}

#[cfg(test)]
mod test {
    use crate::http::{
        types::{Authorization, BasicAuthorization, ContentType},
        HttpVersion, Method,
    };

    const HOST: &str = "www.example.com";
    static ENCODED_AUTH: &str = "dXNlcjpwYXNzd29yZA==";
    lazy_static::lazy_static! {
        static ref BASIC_AUTH: Authorization = Authorization::Basic(BasicAuthorization::new_encoded(ENCODED_AUTH));
    }
    const MOZILLA_UA: (&str, &str) = ("User-Agent", "Mozilla/5.0");
    const MOZILLA_UA_STR: &str = "User-Agent: Mozilla/5.0";

    #[test]
    fn test_get_request_no_authorization() {
        let req = request! {
            HOST get "/",
            MOZILLA_UA.0 => MOZILLA_UA.1;
        };
        assert_eq!(
            req.to_string(),
            format!("GET / HTTP/1.1\nHost: {HOST}\n{MOZILLA_UA_STR}\n\n")
        );
    }

    #[test]
    fn test_get_request_with_authorization() {
        let auth = BASIC_AUTH.clone();
        let req = request! {
            HOST get "/",
            authorization auth,
            MOZILLA_UA.0 => MOZILLA_UA.1;
        };
        assert_eq!(
            req.to_string(),
            format!(
                "GET / HTTP/1.1\nAuthorization: {}\nHost: {HOST}\n{MOZILLA_UA_STR}\n\n",
                BASIC_AUTH.to_string()
            )
        );
    }

    #[test]
    fn test_post_request_no_authorization() {
        let body = Vec::new();
        let req = request! {
        HOST post "/test",
        body body,
        MOZILLA_UA.0 => MOZILLA_UA.1;
        };
        assert_eq!(
            req.to_string(),
            format!("POST /test HTTP/1.1\nHost: {HOST}\n{MOZILLA_UA_STR}\n\n")
        );
    }

    #[test]
    fn test_post_request_with_authorization() {
        let body = Vec::new();
        let auth = BASIC_AUTH.clone();
        let req = request! {
        HOST post "/test",
        authorization auth,
        body body,
        MOZILLA_UA.0 => MOZILLA_UA.1;
        };
        assert_eq!(
            req.to_string(),
            format!(
                "POST /test HTTP/1.1\nAuthorization: {}\nHost: {HOST}\n{MOZILLA_UA_STR}\n\n",
                BASIC_AUTH.to_string()
            )
        );
    }

    /// Test the macro [`_request!`] that is used internally as a support to [`request!`]
    /// macro to create a [`Request`](crate::http::Request).
    #[test]
    fn test_internal_request() {
        // no content-type
        let req = _request! {
            http_version HttpVersion::V1_1,
            host HOST,
            path "/",
            method Method::Get,
            auth Authorization::None,
            body Vec::new(),
        };

        assert_eq!(
            req.to_string(),
            format!("GET / HTTP/1.1\nHost: {HOST}\n\n",)
        );

        // content-type
        let req = _request! {
            http_version HttpVersion::V1_1,
            host HOST,
            path "/",
            method Method::Get,
            auth Authorization::None,
            content_type Some(ContentType::ApplicationJson),
            body Vec::new(),
        };

        assert_eq!(
            req.to_string(),
            format!("GET / HTTP/1.1\nHost: {HOST}\nContent-Type: application/json\n\n",)
        );
    }

    #[test]
    fn test_empty_body() {
        let empty_vec: Vec<u8> = Vec::new();
        let empty_body = empty_body!();

        assert_eq!(empty_vec, empty_body);
    }
}
