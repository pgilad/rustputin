#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate regex;
extern crate time;
extern crate hyper;

use nickel::{
    Request,
    Response,
    StaticFilesHandler,
    MiddlewareResult,
    HttpRouter,
    Nickel
};

use std::collections::HashMap;
use hyper::uri::RequestUri;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    server.utilize(middleware! { |request|
        let path = match request.origin.uri {
            RequestUri::AbsolutePath(ref val) => val.to_string(),
            _ => "unknown uri".to_string()
        };
        println!("{} {}", request.origin.method, path);
    });

    fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
        let moscow_offset = time::Duration::hours(3);
        let cur_time = time::now();
        let cur_utc_time = cur_time.to_utc().rfc822().to_string();
        let rus_time = cur_time + moscow_offset;
        let formatted_rus_time = rus_time.rfc822().to_string();

        let mut data = HashMap::<&str, String>::new();
        data.insert("time", cur_utc_time);
        data.insert("russian_time", formatted_rus_time);

        res.render("templates/index.tpl", &data)
    }

    router.get("/", handler);

    // Mount public directory
    server.utilize(StaticFilesHandler::new("public/"));
    server.utilize(router);

    let address = "127.0.0.1:6767";
    println!("Launching server at :{}", address);
    server.listen(address);
}
