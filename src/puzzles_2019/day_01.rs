use crate::puzzle_input;

pub fn run() {
    part_1();
    part_2();
}

// At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper.
// They haven't determined the amount of fuel required yet.
// The Fuel Counter-Upper needs to know the total fuel requirement.
// To find it, individually calculate the fuel needed for the mass of each module
// (your puzzle input), then add together all the fuel values.
fn part_1() {
    let mut sum = 0;
    let lines = puzzle_input::read_all_lines("./input/2019-d01-input1.txt");
    for line in lines {
        let v = line.parse::<i32>().unwrap();
        let fuel = calculate_fuel(v);
        sum += fuel;
    }
    println!("** Part 1 Final: {0}", sum);
}

fn part_2() {
    let mut sum = 0;
    let lines = puzzle_input::read_all_lines("./input/2019-d01-input1.txt");
    for line in lines {
        let v = line.parse::<i32>().unwrap();
        let fuel = calculate_fuel(v);
        sum += add_additional_fuel(fuel);
    }
    println!("** Part 2 Final: {0}", sum);
}

// Fuel required to launch a given module is based on its mass.
fn calculate_fuel(mass: i32) -> i32 {
    // Specifically, to find the fuel required for a module, take its mass,
    // divide by three, round down, and subtract 2.
    (mass / 3) - 2
}

fn add_additional_fuel(fuel: i32) -> i32 {
    let mut sum = 0;
    let mut result = Some(fuel);

    while let Some(i) = result {
        if i > 0 {
            sum += i;
            result = Some(calculate_fuel(i));
        } else {
            // We're done
            result = None;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fuel_works() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn add_additional_fuel_works() {
        assert_eq!(add_additional_fuel(2), 2);
        assert_eq!(add_additional_fuel(654), 966);
        assert_eq!(add_additional_fuel(33583), 50346);
    }
}
