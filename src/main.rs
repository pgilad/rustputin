#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::{
    StaticFilesHandler,
    HttpRouter,
    Nickel
};

mod root_handler;
mod logger;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    server.utilize(logger::handler);

    router.get("/", root_handler::handler);

    // Mount public directory
    server.utilize(StaticFilesHandler::new("public/"));
    server.utilize(router);

    let address = "127.0.0.1:6767";
    println!("Launching server at :{}", address);
    server.listen(address);
}
