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

pub fn parse_rawstring_to_vector3(rawstring: &String) -> [f32; 3] {
    let mut split = rawstring.split(",");
    let vec = split.collect::<Vec<&str>>();

    let px = vec[0].trim().parse::<f32>().unwrap();
    let py = vec[1].trim().parse::<f32>().unwrap();
    let pz = vec[2].trim().parse::<f32>().unwrap();
    return [px, py, pz];
}

