use std::{fs, io};
use serde::de::DeserializeOwned;
use serde_json::Result;

pub fn parse_json_file<T: DeserializeOwned>(file_path : &String) -> Option<T> {
    let file_result = read_file(file_path);

    if file_result.is_ok() {
        let p: T = serde_json::from_str(&file_result.unwrap()).ok()?;
        return Some(p);
    }

    None
}

pub fn read_file(file_path : &String) -> io::Result<String> {
    return fs::read_to_string(file_path);
        // .expect("Something went wrong reading the file");
}


