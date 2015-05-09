extern crate nickel;
extern crate time;

use std::collections::HashMap;

use self::nickel::{
    Request,
    Response,
    MiddlewareResult,
};

pub fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
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
