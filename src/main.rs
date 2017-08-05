extern crate iron;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("www")));

    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}