use crate::puzzle_input;

pub fn run() {
    let (earliest, schedule) =
        read_bus_schedule(&puzzle_input::read_all_lines("./input/2020-d13-input1.txt"));
    println!(
        "** Part 1 Final: {:?}",
        find_earliest_bus(earliest, &schedule)
    );
    println!(
        "** Part 2 Final: {:?}",
        find_earliest_bus_sequence(&schedule)
    );
}

// Each bus has an ID number that also indicates how often the bus leaves for the airport.
// Bus schedules are defined based on a timestamp that measures the number of minutes
// since some fixed reference point in the past.

fn read_bus_schedule(input: &[String]) -> (i64, Vec<Option<i64>>) {
    // The first line is your estimate of the earliest timestamp you could depart on a bus.
    // The second line lists the bus IDs that are in service according to the shuttle company;
    // entries that show x must be out of service, so you decide to ignore them.
    let time = input[0].parse::<i64>().unwrap();
    let buses: Vec<Option<i64>> = input[1]
        .trim()
        .split(',')
        .map(|x| {
            if x == "x" {
                None
            } else {
                Some(x.parse::<i64>().unwrap())
            }
        })
        .collect();
    println!("Time: {:?}, Buses: ${:?}", time, buses);
    (time, buses)
}

fn get_buses(schedule: &[Option<i64>]) -> Vec<i64> {
    schedule.iter().flatten().copied().collect()
}

// Part 1
fn find_earliest_bus(earliest: i64, schedule: &[Option<i64>]) -> i64 {
    let mut id = 0;
    let mut wait = i64::MAX;
    let mut buses = get_buses(schedule);
    buses.sort_unstable();

    for bus in buses {
        let remainder = bus - (earliest % bus);
        if remainder < wait {
            wait = remainder;
            id = bus;
        }
    }
    wait * id
}

// Part 2
fn find_earliest_bus_sequence(schedule: &[Option<i64>]) -> i64 {
    let buses: Vec<(i64, i64)> = schedule
        .iter()
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|x| (x.0 as i64, x.1.unwrap() as i64))
        .collect();

    let mut t = 0;
    let mut lcm = 1;
    for (offset, duration) in buses {
        while (t + offset) % duration != 0 {
            t += lcm;
        }
        lcm *= duration;
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earliest_bus() {
        let input: Vec<String> = "939
        7,13,x,x,59,x,31,19"
            .split('\n')
            .map(|x| x.trim().to_string())
            .collect();

        let (earliest, schedule) = read_bus_schedule(&input);
        assert_eq!(find_earliest_bus(earliest, &schedule), 295);
        assert_eq!(find_earliest_bus_sequence(&schedule), 1068781);
    }
}
