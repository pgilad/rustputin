extern crate nickel;
extern crate time;
extern crate hyper;

use self::nickel::{
    Request,
    Response,
    MiddlewareResult,
    Continue
};

use self::hyper::uri::RequestUri;

pub fn handler<'a>(request: &mut Request, response: Response<'a>) -> MiddlewareResult<'a> {
    let path = match request.origin.uri {
        RequestUri::AbsolutePath(ref val) => val.to_string(),
        _ => "unknown uri".to_string()
    };
    println!("{} {}", request.origin.method, path);
    Ok(Continue(response))
}
