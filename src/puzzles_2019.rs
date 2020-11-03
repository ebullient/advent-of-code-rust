use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod compute;
mod day_01;
mod day_02;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_string<P>(filename: P) -> io::Result<String>
where P: AsRef<Path>,  {
    return fs::read_to_string(filename);
}


pub fn run(day: i32) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        // Handle the rest of cases
        _ => println!("Nothing to see here"),
    }
}