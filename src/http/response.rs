use http::{HeaderMap, StatusCode, response::Parts};
use std::{
    borrow::Cow,
    io::{self, Write as _},
};
use wasip2::http::types::{self as wasi};

use crate::http::{Body, headers_to_wasi};

pub type Response = http::Response<Body>;

pub trait IntoResponse {
    fn into_response(self) -> Response;

    fn write_to_response_outparam(self, out: wasi::ResponseOutparam)
    where
        Self: Sized,
    {
        let (
            Parts {
                status, headers, ..
            },
            body,
        ) = self.into_response().into_parts();

        let fields = headers_to_wasi(&headers);
        let outgoing_response = wasi::OutgoingResponse::new(fields);

        outgoing_response
            .set_status_code(status.as_u16())
            .expect("http::StatusCode should be valid wasi::StatusCode");

        let result = write_body(&outgoing_response, body).map(|_| outgoing_response);

        wasi::ResponseOutparam::set(out, result);
    }
}

fn write_body(res: &wasi::OutgoingResponse, body: Body) -> Result<(), wasi::ErrorCode> {
    let out_body = res.body().expect("Newly created response should have body");
    let mut stream = out_body
        .write()
        .expect("Newly created body should have a stream");
    let map_err = |e: io::Error| wasi::ErrorCode::InternalError(Some(e.to_string()));

    match body {
        Body::Empty => {}
        Body::Full(bytes) => stream.write_all(&bytes).map_err(map_err)?,
        Body::Stream(mut read) => {
            let mut buf = Vec::new();
            read.read_to_end(&mut buf).map_err(map_err)?;
            stream.write_all(&buf).map_err(map_err)?;
        }
    };

    drop(stream);

    wasi::OutgoingBody::finish(out_body, None)
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        let mut res = Response::new(Body::Empty);
        *res.status_mut() = self;
        res
    }
}

impl IntoResponse for HeaderMap {
    fn into_response(self) -> Response {
        let mut res = Response::new(Body::Empty);
        *res.headers_mut() = self;
        res
    }
}

impl IntoResponse for Body {
    fn into_response(self) -> Response {
        Response::new(self)
    }
}

impl IntoResponse for Cow<'_, str> {
    fn into_response(self) -> Response {
        Body::from(self).into_response()
    }
}

impl IntoResponse for &str {
    fn into_response(self) -> Response {
        Body::from(self).into_response()
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Body::from(self).into_response()
    }
}

impl IntoResponse for Cow<'_, [u8]> {
    fn into_response(self) -> Response {
        Body::from(self).into_response()
    }
}

impl IntoResponse for &[u8] {
    fn into_response(self) -> Response {
        Body::from(self).into_response()
    }
}

impl IntoResponse for Vec<u8> {
    fn into_response(self) -> Response {
        Body::from(self).into_response()
    }
}

impl<R: IntoResponse> IntoResponse for (StatusCode, HeaderMap, R) {
    fn into_response(self) -> Response {
        let (status, headers, res) = self;
        let mut response = res.into_response();
        *response.status_mut() = status;
        *response.headers_mut() = headers;
        response
    }
}

impl<R: IntoResponse> IntoResponse for (StatusCode, R) {
    fn into_response(self) -> Response {
        let (status, res) = self;
        let mut response = res.into_response();
        *response.status_mut() = status;
        response
    }
}

impl<R: IntoResponse> IntoResponse for (HeaderMap, R) {
    fn into_response(self) -> Response {
        let (headers, res) = self;
        let mut response = res.into_response();
        *response.headers_mut() = headers;
        response
    }
}

impl<T: IntoResponse, E: IntoResponse> IntoResponse for Result<T, E> {
    fn into_response(self) -> Response {
        match self {
            Ok(ok) => ok.into_response(),
            Err(err) => err.into_response(),
        }
    }
}
