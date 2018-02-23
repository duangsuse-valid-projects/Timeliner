extern crate futures;
extern crate num_cpus;
extern crate serde_json;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

#[macro_use]
extern crate serde_derive;

use futures::future;
use tokio_service::Service;
use tokio_proto::TcpServer;
use tokio_minihttp::{Http, Request, Response};
use serde_json::*;

use std::io;

struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.body("Hello, world!");
        future::ok(resp)
    }
}

fn main() {
    println!("Hello, world!");
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut srv = TcpServer::new(Http, addr);
    srv.threads(num_cpus::get());
    srv.serve(|| Ok(HelloWorld));
}
