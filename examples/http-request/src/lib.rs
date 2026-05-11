use wassel_sdk::http::{
    IntoResponse, Request, Response, StatusCode,
    client::{self, RequestError},
    handler,
};

#[handler]
fn handle_request(request: Request) -> Result<Response, Error> {
    let path = request.uri().path();
    let id = path.strip_prefix("/todos/").ok_or(Error::NotFound)?;
    let url = format!("https://jsonplaceholder.typicode.com/todos/{id}");
    let response = client::get(url).send().map_err(Error::Request)?;
    Ok(response)
}

enum Error {
    NotFound,
    Request(RequestError),
}

impl IntoResponse for Error {
    fn into_response(self) -> wassel_sdk::http::Response {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            Error::Request(request_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, request_error.to_string()).into_response()
            }
        }
    }
}
