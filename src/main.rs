extern crate futures;
extern crate num_cpus;
extern crate serde;
extern crate serde_json;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use futures::future;
use tokio_service::Service;
use tokio_proto::TcpServer;
use tokio_minihttp::{Http, Request, Response};

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use std::time::SystemTime;
use std::collections::HashMap;

/// Default listening address
const DEFAULT_LISTEN: &str = "0.0.0.0:80";
/// Default password
const DEFAULT_PASSWORD: &str = "dolphins";
/// Version string
const VERSION: &str = "0.1.0";

lazy_static! {
    /// Password
    static ref PASS: String = get_password();
}

/// Timeline Service
struct Timeline {
    posts: Vec<Post>,
}

impl Service for Timeline {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, request: Request) -> Self::Future {
        let mut resp = Response::new();
        if check_spam(&request) {
            match request.path() {
                "/" => {
                    resp.header("Content-Type", "text/plain")
                        .body(&self.posts.len().to_string());
                }
                "/version" => {
                    resp.header("Content-Type", "text/plain").body(VERSION);
                }
                "/pop" => {
                    if check_auth(request) {
                        &mut self.posts.pop();
                        println!("Poping.... New length: {}", &self.posts.len());
                    } else {
                        resp.status_code(401, "Key Required");
                    }
                }
                _ => {
                    resp.status_code(404, "QAQ");
                }
            }
        } else {
            resp.body("Rate limited");
        }
        future::ok(resp)
    }
}
/// Comment model
#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    author: String,
    text: String,
    date: SystemTime,
}

/// Post model
#[derive(Serialize, Deserialize, Debug)]
struct Post {
    author: String,
    text: String,
    date: SystemTime,
    comments: Vec<Comment>,
}

/// Get password from cmdline or default
fn get_password() -> String {
    args().nth(2).unwrap_or_else(|| {
        eprintln!("Warning: using default password!!!");
        DEFAULT_PASSWORD.to_string()
    })
}

/// Spam check
fn check_spam(req: &Request) -> bool {
    false
}

/// Checks auth header in request headers
/// returns true if password is valid
fn check_auth(req: Request) -> bool {
    false
}

/// Dump current posts to storage
fn dump_posts(p: Vec<Post>) {
    let filename = "posts.json";
    let or_exit = |e| {
        panic!("Failed to create {}: {}", filename, e);
    };
    let or_create = |_| {
        eprintln!("Failed to open storage, Creating {}...", filename);
        File::create(filename).unwrap_or_else(or_exit)
    };
    let mut file = File::open(filename).unwrap_or_else(or_create);
    let buffer = serde_json::to_string(&p).unwrap();
    file.write(buffer.as_bytes()).unwrap_or_else(|e| {
        panic!("Failed to write file!: {}", e);
    });
}

fn main() {
    let addr = args()
        .nth(1)
        .unwrap_or(DEFAULT_LISTEN.to_string())
        .parse()
        .unwrap();
    println!("Hello, timeline!");
    eprintln!("Listing on {}, password: {}", addr, *PASS);
    let mut srv = TcpServer::new(Http, addr);
    srv.threads(num_cpus::get());
    srv.serve(|| {
        Ok(Timeline {
            /// All Posts
            /// New a blank vector or load from storage
            posts: {
                let file = File::open("posts.json");
                let mut posts: Vec<Post> = Vec::<Post>::new();
                if file.is_err() {
                    posts
                } else {
                    let mut buffer = String::new();
                    file.unwrap()
                        .read_to_string(&mut buffer)
                        .unwrap_or_else(|_| {
                            panic!("Failed to read file");
                        });
                    posts = serde_json::from_str(&buffer).unwrap_or_else(|_| {
                        panic!("Failed to load storage");
                    });
                    posts
                }
            },
        })
    });
}
