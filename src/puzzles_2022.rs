mod day_01;
mod day_02;
mod day_03;

pub fn run(day: i32) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),

        // Handle the rest of cases
        _ => println!("Nothing to see here"),
    }
}
