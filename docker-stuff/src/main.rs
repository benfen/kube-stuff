use iron::prelude::*;
use iron::status;

static mut NUM: i32 = 0;

fn handler(_: &mut Request) -> IronResult<Response> {
    unsafe {
        let resp = format!("Hello: {}", NUM);
        NUM += 1;
        Ok(Response::with((status::Ok, resp)))
    }
}

fn main() {
    Iron::new(handler).http("0.0.0.0:8080").expect("Server failed on startup.");
}