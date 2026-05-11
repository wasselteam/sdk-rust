use std::io::{self, Read as _};

use wassel_sdk::{
    bindings::wassel::foundation::http_client::{self, IncomingResponse, OutgoingRequest},
    http::{IntoResponse, Request, StatusCode, handler},
    wasi::http::types::Fields,
};

#[handler]
fn handle_request(request: Request) -> impl IntoResponse {
    let path = request.uri().path();

    let Some(id) = path.strip_prefix("/todos/") else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let req = OutgoingRequest::new(Fields::new());
    let url = format!("https://jsonplaceholder.typicode.com/todos/{id}");
    let todo_response = match http_client::send(&url, req) {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        Ok(r) => r,
    };

    let status = todo_response.status();
    let body = match read_body_to_bytes(todo_response) {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not read body: {e:?}"),
            )
                .into_response();
        }
    };

    (
        StatusCode::from_u16(status).unwrap(),
        body.unwrap_or_default(),
    )
        .into_response()
}

fn read_body_to_bytes(response: IncomingResponse) -> Result<Option<Vec<u8>>, io::Error> {
    let body = match response.consume() {
        Ok(b) => b,
        Err(()) => return Ok(None),
    };
    let mut stream = body.stream().expect("getting stream from body");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    Ok(Some(buf))
}
