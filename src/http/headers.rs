use std::str::FromStr as _;

use wasip2::http::types as wasi;

pub fn headers_from_wasi(headers: &wasi::Fields) -> http::HeaderMap {
    let entries: Vec<(http::HeaderName, http::HeaderValue)> = headers
        .entries()
        .into_iter()
        .map(|(name, value)| {
            let name = http::HeaderName::from_str(&name)
                .expect("WASI header name should be valid HTTP header name");
            let value = http::HeaderValue::from_bytes(&value)
                .expect("WASI header value should be valid HTTP header value");
            (name, value)
        })
        .collect();

    http::HeaderMap::from_iter(entries)
}

pub fn headers_to_wasi(headers: &http::HeaderMap) -> wasi::Fields {
    let fields = wasi::Fields::new();
    headers.iter().for_each(|(name, value)| {
        if let Err(e) = fields.append(name.as_str(), value.as_bytes()) {
            // TODO: Maybe we should do tracing::error here
            #[cfg(debug_assertions)]
            panic!("failed to append validated header `{name}`: {e}");
            #[cfg(not(debug_assertions))]
            eprintln!("failed to append validated header `{name}`: {e}");
        }
    });
    fields
}
