use std::fs;
use std::io::Write;
use const_format::formatcp;

const DIR: &'static str = "./beijing_subway_files/";
// const BEIJING_XML: &'static str = formatcp!("{:?}{:?}", DIR, "beijing.xml");
const BEIJING_XML: &'static str="./beijing_subway_files/beijing.xml";
pub fn read_beijing_xml() -> String {
    fs::read_to_string(BEIJING_XML).unwrap()
}

pub fn write_beijing_xml(content: &str) {
    fs::create_dir_all(DIR).expect("create dir error");//create if not exists
    let mut file = fs::File::create(BEIJING_XML).expect("create file error");
    file.write(content.as_bytes());
}