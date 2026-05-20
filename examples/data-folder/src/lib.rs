use std::{
    fs,
    io::{self, BufReader},
};
use wassel_sdk::http::{Body, IntoResponse, Request, Response, StatusCode, handler};

#[handler]
fn handle_request(request: Request) -> Result<Response, Error> {
    let path = fs::canonicalize(request.uri().path())?;
    let length = fs::metadata(&path)?.len();
    let stream = BufReader::new(fs::File::open(&path)?);
    let body = Body::stream(stream);

    let mut response = Response::new(body);
    response
        .headers_mut()
        .insert("Content-Length", length.into());
    response
        .headers_mut()
        .insert("Content-Type", "application/octet-stream".parse().unwrap());

    Ok(response)
}

enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> wassel_sdk::http::Response {
        match self {
            Error::Io(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
        }
    }
}
