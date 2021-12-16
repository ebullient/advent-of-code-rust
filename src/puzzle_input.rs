use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_all_lines<'a, P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut result: Vec<String> = Vec::new();
    let mut reader = Box::new(io::empty()) as Box<dyn BufRead>;

    let file = File::open(filename);
    if file.is_ok() {
        reader = Box::new(BufReader::new(file.unwrap()))
    }

    for line in reader.lines() {
        if let Ok(item) = line {
            result.push(item);
        }
    }
    result
}

pub fn read_string<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    if let Ok(result) = fs::read_to_string(filename) {
        result
    } else {
        String::new()
    }
}

#[allow(dead_code)]
pub fn split_string(input: &str) -> Vec<String> {
    input.split('\n').map(|x| x.trim().to_string()).collect()
}
