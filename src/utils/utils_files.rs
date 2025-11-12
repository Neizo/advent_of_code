use std::fs;

pub fn parse_file(_file_path:String) -> String {
    fs::read_to_string(_file_path).expect("Unable to read file")
}