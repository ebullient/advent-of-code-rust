mod puzzles_2019;
mod puzzles_2020;
mod puzzle_input;

extern crate getopts;
#[macro_use] extern crate lazy_static;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -y 2019 -d 1", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("y", "year", "select a puzzle year", "2019");
    opts.optopt("d", "day", "select a puzzle day", "01");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let year = match matches.opt_str("y") {
        Some(s) => s.parse::<i32>().unwrap(),
        None => 2020,
    };
    let day = match matches.opt_str("d") {
        Some(s) => s.parse::<i32>().unwrap(),
        None => 1,
    };

    println!("{}", format!("Running {0}:day_{1:02}", year, day));
    match year {
        2019 => puzzles_2019::run(day),
        2020 => puzzles_2020::run(day),
        // Handle the rest of cases
        _ => println!("Nothing to see here"),
    }
}
