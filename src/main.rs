extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::prelude::*;
use iron::Iron; 
use iron::status;
use staticfile::Static;
use mount::Mount;

fn main() {

    let mut mount = Mount::new();

    let routes = &["/", "/about", "/contact"];

    for route in routes {
        mount.mount(route, Static::new(Path::new("static/index.html")));
    }

    mount.mount("/index.js", Static::new(Path::new("static/index.js")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
