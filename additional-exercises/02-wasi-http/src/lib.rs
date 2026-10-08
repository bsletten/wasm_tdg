//! A `wasi:http/proxy` component, served by `wasmtime serve`.
//!
//! This is the shape that makes WebAssembly interesting on the server side.
//! There is no `main`, no socket, no listener and no HTTP server compiled in.
//! The component *exports* `wasi:http/incoming-handler` and the host supplies
//! the network; the same artifact runs under `wasmtime serve` and on edge
//! platforms that speak the same interface.
//!
//! Run `wasm-tools component wit` on the built artifact to see the contract:
//! it imports `wasi:http/outgoing-handler` and exports
//! `wasi:http/incoming-handler`.

use wstd::http::body::Body;
use wstd::http::{Error, Request, Response, StatusCode};

#[wstd::http_server]
async fn main(request: Request<Body>) -> Result<Response<Body>, Error> {
    let path = request.uri().path().to_owned();
    let query = request.uri().query().unwrap_or("").to_owned();

    match path.as_str() {
        "/" => Ok(Response::new(
            "WebAssembly: The Definitive Guide -- wasi:http example\n\
             \n\
             Try:\n\
             \t/hello\n\
             \t/how-old?now=2026&born=2000\n"
                .to_owned()
                .into(),
        )),

        "/hello" => Ok(Response::new("Hello, world!\n".to_owned().into())),

        // `how_old` has followed us since Chapter 2. Here it answers over HTTP.
        "/how-old" => {
            let param = |key: &str| -> Option<i32> {
                query
                    .split('&')
                    .filter_map(|kv| kv.split_once('='))
                    .find(|(k, _)| *k == key)
                    .and_then(|(_, v)| v.parse().ok())
            };

            match (param("now"), param("born")) {
                (Some(now), Some(born)) => {
                    Ok(Response::new(format!("You are {}\n", now - born).into()))
                }
                _ => Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("usage: /how-old?now=2026&born=2000\n".to_owned().into())
                    .unwrap()),
            }
        }

        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("not found\n".to_owned().into())
            .unwrap()),
    }
}
