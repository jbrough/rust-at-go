#![warn(rust_2018_idioms)]

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::info;
use serde_json::{json, to_string};
use std::net::SocketAddr;
use tokio::net::TcpListener;

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 1337).into();

    let srv = async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        loop {
            // got a new connection
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(|req| api_handler(req)))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    };

    info!("rs bench server listening on {:?}", addr);

    srv.await;

    Ok(())
}

async fn api_handler(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let json_response = json!({
              "status": "green",
            });
            let body = to_string(&json_response).expect("JSON serialization error");
            let res = Response::new(full(Bytes::from(body)).boxed());
            Ok(res)
        }
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
