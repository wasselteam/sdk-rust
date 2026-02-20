use std::fs;

use wassel_sdk_rust::bindings::{
    export,
    exports::wassel::foundation::http_handler::{Guest, IncomingRequest, ResponseOutparam},
    wasi::http::types::{Headers, OutgoingBody, OutgoingResponse},
};

struct Plugin;

impl Guest for Plugin {
    fn handle_request(_request: IncomingRequest, response_out: ResponseOutparam) {
        match fs::read("file.txt") {
            Ok(data) => write_response(response_out, 200, Some(&data)),
            Err(e) => write_response(response_out, 500, Some(e.to_string().as_bytes())),
        }
    }
}

fn write_response(out: ResponseOutparam, status: u16, body_bytes: Option<&[u8]>) {
    let res = OutgoingResponse::new(Headers::new());
    res.set_status_code(status).unwrap();

    if let Some(bytes) = body_bytes {
        let body = res.body().unwrap();
        {
            let stream = body.write().unwrap();
            stream.write(bytes.into()).unwrap();
        }
        OutgoingBody::finish(body, None).unwrap();
    }

    ResponseOutparam::set(out, Ok(res));
}

export!(Plugin);
