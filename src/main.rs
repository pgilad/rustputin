#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate regex;
extern crate time;

use nickel::*;
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    server.utilize(middleware! { |request|
        println!("logging request: {:?}", request.origin.uri);
    });

    fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
        let mut data = HashMap::<&str, String>::new();
        let cur_time = time::now();
        data.insert("time", cur_time.rfc822().to_string());
        res.render("templates/index.tpl", &data)
    }

    router.get("/", handler);
    server.utilize(StaticFilesHandler::new("public/"));
    server.utilize(router);
    server.listen("127.0.0.1:6767");
}
