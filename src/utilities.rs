use std::fs::File;
use std::io::{BufReader, Read};

pub(crate) fn load_input(path: String) -> String {
    let file = File::open(path).unwrap();
    let mut buffer = BufReader::new(file);
    let mut i = String::new();
    _ = buffer.read_to_string(&mut i);

    i
}