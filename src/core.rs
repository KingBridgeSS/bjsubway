use crate::errors::NetError;
use crate::net_dealer;
use crate::util;
use roxmltree::*;
use urlencoding::encode;
use serde_json::{Value};
use serde_json::Value::String;

pub fn update() {
    let content = net_dealer::do_get("http://map.bjsubway.com/subwaymap/beijing.xml").unwrap();
    util::write_beijing_xml(&content[3..]);
    println!("updated");
}

pub fn show_lines() {
    let content = util::read_beijing_xml();
    let doc = Document::parse(&content).unwrap();
    let lines: Vec<Node> = doc
        .descendants()
        .filter(|n| n.has_tag_name(("l")))
        .collect();
    for line in &lines {
        println!("lid = {:?},\tlcode = {:?}", &line.attribute("lid").unwrap(), &line.attribute("lcode").unwrap());
    }
}

pub fn print_detail(lcode: &str) {
    let xml_str = util::read_beijing_xml();
    let doc = Document::parse(&xml_str).unwrap();
    let line: Vec<Node> = doc.
        descendants()
        .find((|n| n.has_tag_name("l")
            && n.attribute("lcode") == Some(lcode)))
        .unwrap()
        .children()
        .filter(|n| n.is_element())
        .collect();
    for station in line {
        println!("{}", station.attribute("lb").unwrap());
    }
}

pub fn find_lines(from: &str, to: &str) {
    let _from = encode(&from);
    let _to = encode(&to);
    let url = format!("http://map.bjsubway.com/searchstartend?start={_from}&end={_to}&mintype=1");
    let json_str = net_dealer::do_get(&url).unwrap();
    let json_data: Value = serde_json::from_str(&*json_str).expect("Parsing json error");
    let line_paths = json_data["fangan"][0]["p"].as_array().unwrap();

    let xml_str = util::read_beijing_xml();
    let doc = Document::parse(&xml_str).unwrap();
    for line_path in line_paths {
        for station in line_path.as_array().unwrap() {
            let lcode = &station[0];
            let station_name = &station[1];
            let lids: Vec<Node> = doc.
                descendants()
                .filter((|n| n.has_tag_name("l")
                    && n.attribute("lcode") == Some(lcode.as_str().unwrap())))
                .collect();
            let lid = lids[0].attribute("lid").unwrap();
            println!("station: {station_name},\tlid: {lid}");
        }
    }
}

#[test]
fn show_lines_test() {
    show_lines();
}

#[test]
fn find_lines_test() {
    find_lines("沙河", "天安门东");
}