use iron::prelude::*;
use iron::status;
use std::{thread, time};

static mut NUM: i32 = 0;

fn handler(_: &mut Request) -> IronResult<Response> {
    unsafe {
        let resp = format!("Hello: {}", NUM);
        NUM += 1;

        if NUM == 5 {
            thread::spawn(|| {
                thread::sleep(time::Duration::from_millis(100));
                ::std::process::exit(0);
            });
            Ok(Response::with((status::Ok, "Exiting")))
        } else {
            Ok(Response::with((status::Ok, resp)))
        }
    }
}

fn main() {
    Iron::new(handler).http("0.0.0.0:8080").expect("Server failed on startup.");
}