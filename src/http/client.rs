use std::borrow::Cow;
use std::io;
use std::io::Read as _;

use http::{HeaderMap, HeaderValue, StatusCode, header::IntoHeaderName};
use wasip2::http::types as wasi;

use crate::{
    bindings::wassel::http_client::http_client,
    http::{Body, Request, Response, headers_from_wasi, headers_to_wasi, method_to_wasi},
};

type Result<T, E = RequestError> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("WASI error: {0}")]
    ErrorCode(#[from] http_client::ErrorCode),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid status code")]
    InvalidStatusCode,

    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
}

impl From<http::status::InvalidStatusCode> for RequestError {
    fn from(_: http::status::InvalidStatusCode) -> Self {
        Self::InvalidStatusCode
    }
}

pub trait RequestExt {
    fn send(self) -> Result<Response, RequestError>;
}

impl RequestExt for Request {
    fn send(mut self) -> Result<Response, RequestError> {
        let out_request = http_client::OutgoingRequest::new(headers_to_wasi(self.headers()));

        out_request
            .set_method(&method_to_wasi(self.method()))
            .expect("HTTP method should be valid WASI method");

        let out_body = out_request
            .body()
            .expect("Newly created request should have a body");

        {
            let mut stream = out_body
                .write()
                .expect("Newly created request should have a stream");
            io::copy(self.body_mut(), &mut stream)?;
        }

        wasi::OutgoingBody::finish(out_body, None)?;

        let url = self.uri().to_string();
        let in_response = http_client::send(&url, out_request)?;

        let headers = headers_from_wasi(&in_response.headers());
        let status = StatusCode::from_u16(in_response.status())?;

        let body = if let Ok(in_body) = in_response.consume() {
            let mut bytes = Vec::new();
            {
                let mut stream = in_body
                    .stream()
                    .expect("Stream should be present in response");
                stream.read_to_end(&mut bytes)?;
            }
            wasi::IncomingBody::finish(in_body);
            Body::Full(bytes)
        } else {
            Body::Empty
        };

        let mut response = Response::new(body);
        *response.status_mut() = status;
        *response.headers_mut() = headers;

        Ok(response)
    }
}

#[derive(Default)]
pub struct RequestBuilder(RequestBuilderInner);

enum RequestBuilderInner {
    Ok {
        method: http::Method,
        uri: http::Uri,
        headers: HeaderMap,
        body: Body,
    },
    Err(RequestError),
}

impl Default for RequestBuilderInner {
    fn default() -> Self {
        Self::Ok {
            method: Default::default(),
            uri: Default::default(),
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

impl RequestBuilderInner {
    fn set_body(&mut self, body: impl Into<Body>) {
        if let RequestBuilderInner::Ok { body: body_ref, .. } = self {
            *body_ref = body.into();
        }
    }

    fn set_header(&mut self, key: impl IntoHeaderName, value: impl Into<HeaderValue>) {
        if let RequestBuilderInner::Ok { headers, .. } = self {
            headers.insert(key, value.into());
        }
    }
}

impl RequestBuilder {
    pub fn new(method: http::Method, uri: http::Uri) -> Self {
        Self(RequestBuilderInner::Ok {
            method,
            uri,
            headers: HeaderMap::new(),
            body: Body::Empty,
        })
    }

    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.0.set_body(body);
        self
    }

    pub fn header(mut self, key: impl IntoHeaderName, value: impl Into<HeaderValue>) -> Self {
        self.0.set_header(key, value);
        self
    }

    pub fn build(self) -> Result<Request, RequestError> {
        match self.0 {
            RequestBuilderInner::Ok {
                method,
                uri,
                headers,
                body,
            } => {
                let mut req = Request::new(body);
                *req.uri_mut() = uri;
                *req.method_mut() = method;
                *req.headers_mut() = headers;
                Ok(req)
            }
            RequestBuilderInner::Err(request_error) => Err(request_error),
        }
    }

    pub fn send(self) -> Result<Response, RequestError> {
        self.build()?.send()
    }
}

pub trait IntoUri {
    fn into_uri(self) -> Result<http::Uri>;
}

impl IntoUri for http::Uri {
    fn into_uri(self) -> Result<http::Uri> {
        Ok(self)
    }
}

impl IntoUri for Cow<'_, str> {
    fn into_uri(self) -> Result<http::Uri> {
        let uri = self.parse()?;
        Ok(uri)
    }
}

impl IntoUri for &String {
    fn into_uri(self) -> Result<http::Uri> {
        Cow::<'_, str>::Borrowed(self).into_uri()
    }
}

impl IntoUri for &str {
    fn into_uri(self) -> Result<http::Uri> {
        Cow::<'_, str>::Borrowed(self).into_uri()
    }
}

impl IntoUri for String {
    fn into_uri(self) -> Result<http::Uri> {
        Cow::<'_, str>::Owned(self).into_uri()
    }
}

pub fn request(method: http::Method, uri: impl IntoUri) -> RequestBuilder {
    match uri.into_uri() {
        Ok(uri) => RequestBuilder::new(method, uri),
        Err(e) => RequestBuilder(RequestBuilderInner::Err(e)),
    }
}

pub fn options(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::OPTIONS, uri)
}

pub fn get(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::GET, uri)
}

pub fn post(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::POST, uri)
}

pub fn put(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::PUT, uri)
}

pub fn delete(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::DELETE, uri)
}

pub fn head(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::HEAD, uri)
}

pub fn trace(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::TRACE, uri)
}

pub fn connect(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::CONNECT, uri)
}

pub fn patch(uri: impl IntoUri) -> RequestBuilder {
    request(http::Method::PATCH, uri)
}
