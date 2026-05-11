mod body;
mod request;
mod response;

use std::str::FromStr;

pub use body::Body;
pub use http::StatusCode;
pub use request::Request;
pub use response::{IntoResponse, Response};
pub use wassel_sdk_macros::handler;

pub fn handle_request_with_handler<I: IntoResponse>(
    in_request: wasip2::http::types::IncomingRequest,
    out_response: wasip2::http::types::ResponseOutparam,
    handler: impl Fn(Request) -> I,
) {
    let (request, in_body) = convert_request(in_request);
    let response = handler(request).into_response();
    wasip2::http::types::IncomingBody::finish(in_body);
    response.write_to_response_outparam(out_response);
}

fn convert_request(
    in_request: wasip2::http::types::IncomingRequest,
) -> (Request, wasip2::http::types::IncomingBody) {
    let method = convert_method(in_request.method());
    let uri = convert_uri(&in_request);
    let headers = convert_headers(in_request.headers());

    let in_body = in_request.consume().expect("Request should have body");
    let stream = in_body.stream().expect("Body should have stream");
    let body = Body::Stream(Box::new(stream));

    let mut request = Request::new(body);
    *request.method_mut() = method;
    *request.uri_mut() = uri;
    *request.headers_mut() = headers;

    (request, in_body)
}

fn convert_method(method: wasip2::http::types::Method) -> http::Method {
    match method {
        wasip2::http::types::Method::Get => http::Method::GET,
        wasip2::http::types::Method::Head => http::Method::HEAD,
        wasip2::http::types::Method::Post => http::Method::POST,
        wasip2::http::types::Method::Put => http::Method::PUT,
        wasip2::http::types::Method::Delete => http::Method::DELETE,
        wasip2::http::types::Method::Connect => http::Method::CONNECT,
        wasip2::http::types::Method::Options => http::Method::OPTIONS,
        wasip2::http::types::Method::Trace => http::Method::TRACE,
        wasip2::http::types::Method::Patch => http::Method::PATCH,
        wasip2::http::types::Method::Other(s) => {
            http::Method::from_bytes(s.as_bytes()).expect("WASI Method should be valid HTTP method")
        }
    }
}

fn convert_uri(in_request: &wasip2::http::types::IncomingRequest) -> http::Uri {
    use wasip2::http::types::Scheme;

    let mut builder = http::Uri::builder();

    match in_request.scheme() {
        Some(Scheme::Http) => builder = builder.scheme("http"),
        Some(Scheme::Https) => builder = builder.scheme("https"),
        Some(Scheme::Other(s)) => builder = builder.scheme(s.as_str()),
        None => {}
    }

    if let Some(path) = in_request.path_with_query() {
        builder = builder.path_and_query(path);
    }

    if let Some(authority) = in_request.authority() {
        builder = builder.authority(authority);
    }

    builder.build().expect("WASI URI should be valid HTTP URI")
}

fn convert_headers(headers: wasip2::http::types::Headers) -> http::HeaderMap {
    let entries: Vec<(http::HeaderName, http::HeaderValue)> = headers
        .entries()
        .into_iter()
        .map(|(name, value)| {
            let name = http::HeaderName::from_str(&name)
                .expect("WASI header name should be valid HTTP header name");
            let value = http::HeaderValue::from_bytes(&value)
                .expect("WASI header value should be valid HTTP header value");
            (name, value)
        })
        .collect();

    http::HeaderMap::from_iter(entries)
}
