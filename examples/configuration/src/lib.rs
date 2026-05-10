use wassel_sdk_rust::bindings::{
    export,
    exports::wassel::foundation::http_handler::{Guest, IncomingRequest, ResponseOutparam},
    wasi::http::types::{Headers, OutgoingBody, OutgoingResponse},
};

struct Plugin;

impl Guest for Plugin {
    fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
        match request.path_with_query().as_deref().unwrap_or("/") {
            "/secret" => write_response(response_out, 200, Some(get_config("secret").as_bytes())),
            "/host" => write_response(response_out, 200, Some(get_config("host").as_bytes())),
            "/hostname" => {
                write_response(response_out, 200, Some(get_config("hostname").as_bytes()))
            }
            _ => write_response(response_out, 404, None),
        }
    }
}

fn get_config(key: &str) -> String {
    wassel_sdk_rust::bindings::wasi_config::store::get(key)
        .unwrap_or_default()
        .unwrap_or_default()
}

fn write_response(out: ResponseOutparam, status: u16, body_bytes: Option<&[u8]>) {
    let res = OutgoingResponse::new(Headers::new());
    res.set_status_code(status).unwrap();

    if let Some(bytes) = body_bytes {
        let body = res.body().unwrap();
        {
            let stream = body.write().unwrap();
            stream.write(bytes).unwrap();
        }
        OutgoingBody::finish(body, None).unwrap();
    }

    ResponseOutparam::set(out, Ok(res));
}

export!(Plugin);
