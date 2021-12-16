use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_all_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut result: Vec<String> = Vec::new();

    if let Ok(file) = File::open(filename) {
        let reader = Box::new(BufReader::new(file));

        for line in reader.lines().flatten() {
            result.push(line);
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
