use iron::prelude::*;
use iron::status;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, world!")))
}

fn main() {
    Iron::new(handler).http("0.0.0.0:8080").expect("Server failed on startup.");
}