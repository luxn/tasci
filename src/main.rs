extern crate snowflake;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{server, App, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");

    format!("Hello {}!\n{:?}", to, users)
}

fn main() {
    server::new( || {
        App::new()
            .resource("/", |r| r.f(greet))
    })
        .bind("localhost:8081")
        .expect("Can not bind to port 8080 on localhost")
        .run();
}
