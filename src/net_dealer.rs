use std::io::Read;
use crate::errors;
use errors::NetError;
use crate::util::write_beijing_xml;

pub fn do_get(url: &str) -> Result<String, NetError> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}

#[test]
fn get_test() {
    let url="http://map.bjsubway.com/subwaymap/beijing.xml";
    let body=do_get(url).unwrap();
    write_beijing_xml(&body[3..]);
}