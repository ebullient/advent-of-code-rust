use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_all_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let mut result: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(item) = line {
                result.push(item);
            }
        }
    }
    result
}

pub fn read_string<P>(filename: P) -> io::Result<String>
where P: AsRef<Path>,  {
    return fs::read_to_string(filename);
}