mod body;
pub mod client;
mod headers;
mod method;
mod request;
mod response;

pub use body::Body;
pub use headers::{headers_from_wasi, headers_to_wasi};
pub use http::StatusCode;
pub use method::{method_from_wasi, method_to_wasi};
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
    let method = method_from_wasi(in_request.method());
    let uri = convert_uri(&in_request);
    let headers = headers_from_wasi(&in_request.headers());

    let in_body = in_request.consume().expect("Request should have body");
    let stream = in_body.stream().expect("Body should have stream");
    let body = Body::Stream(Box::new(stream));

    let mut request = Request::new(body);
    *request.method_mut() = method;
    *request.uri_mut() = uri;
    *request.headers_mut() = headers;

    (request, in_body)
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
