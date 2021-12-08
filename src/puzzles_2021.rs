mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
// mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;

pub fn run(day: i32) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),
        4 => day_04::run(),
        5 => day_05::run(),
        6 => day_06::run(),
        7 => day_07::run(),
        8 => day_08::run(),
        // 9 => day_09::run(),
        // 10 => day_10::run(),
        // 11 => day_11::run(),
        // 12 => day_12::run(),
        // 13 => day_13::run(),
        // 14 => day_14::run(),
        // 15 => day_15::run(),
        // 16 => day_16::run(),

        // Handle the rest of cases
        _ => println!("Nothing to see here"),
    }
}
