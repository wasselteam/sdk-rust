use wasip2::http::types as wasi;

pub fn method_to_wasi(method: &http::Method) -> wasi::Method {
    match *method {
        http::Method::GET => wasi::Method::Get,
        http::Method::HEAD => wasi::Method::Head,
        http::Method::POST => wasi::Method::Post,
        http::Method::PUT => wasi::Method::Put,
        http::Method::DELETE => wasi::Method::Delete,
        http::Method::CONNECT => wasi::Method::Connect,
        http::Method::OPTIONS => wasi::Method::Options,
        http::Method::TRACE => wasi::Method::Trace,
        http::Method::PATCH => wasi::Method::Patch,
        _ => wasi::Method::Other(method.to_string()),
    }
}

pub fn method_from_wasi(method: wasi::Method) -> http::Method {
    match method {
        wasi::Method::Get => http::Method::GET,
        wasi::Method::Head => http::Method::HEAD,
        wasi::Method::Post => http::Method::POST,
        wasi::Method::Put => http::Method::PUT,
        wasi::Method::Delete => http::Method::DELETE,
        wasi::Method::Connect => http::Method::CONNECT,
        wasi::Method::Options => http::Method::OPTIONS,
        wasi::Method::Trace => http::Method::TRACE,
        wasi::Method::Patch => http::Method::PATCH,
        wasi::Method::Other(s) => {
            http::Method::from_bytes(s.as_bytes()).expect("WASI Method should be valid HTTP method")
        }
    }
}
