use wassel_sdk_rust::bindings::{
    export,
    exports::wassel::foundation::http_handler::{Guest, IncomingRequest, ResponseOutparam},
    wasi::http::types::{Fields, Headers, OutgoingBody, OutgoingResponse},
    wasi_config,
    wassel::foundation::http_client::{self, IncomingResponse, OutgoingRequest},
};

use std::io::{self, Read as _, Write as _};

struct Plugin;

impl Guest for Plugin {
    fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
        let _base_url = wasi_config::store::get("base_url")
            .ok()
            .flatten()
            .unwrap_or_else(|| "No base url".to_owned());

        let path = request.path_with_query().expect("path");
        let Some(id) = path.strip_prefix("/todos/") else {
            write_response(response_out, 404, None);
            return;
        };

        let req = OutgoingRequest::new(Fields::new());
        let url = format!("https://jsonplaceholder.typicode.com/todos/{id}");
        let todo_response = match http_client::send(&url, req) {
            Err(e) => {
                write_response(response_out, 500, Some(e.to_string().as_bytes()));
                return;
            }
            Ok(r) => r,
        };

        let status = todo_response.status();
        let body = match read_body_to_bytes(todo_response) {
            Ok(b) => b,
            Err(e) => {
                write_response(
                    response_out,
                    500,
                    Some(format!("Could not read body: {e:?}").as_bytes()),
                );
                return;
            }
        };

        write_response(response_out, status, body.as_deref());
    }
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

fn write_response(out: ResponseOutparam, status: u16, body_bytes: Option<&[u8]>) {
    let res = OutgoingResponse::new(Headers::new());
    res.set_status_code(status).unwrap();

    if let Some(bytes) = body_bytes {
        let body = res.body().unwrap();
        {
            let mut stream = body.write().unwrap();
            stream.write_all(bytes).unwrap();
        }
        OutgoingBody::finish(body, None).unwrap();
    }

    ResponseOutparam::set(out, Ok(res));
}

export!(Plugin);
