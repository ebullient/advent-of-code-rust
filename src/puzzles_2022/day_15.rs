use crate::puzzle_input;
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash;
use std::ops::Range;
use std::time::Instant;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d15-input.txt");
    let field = Field::new(&input);

    let mut start = Instant::now();
    println!("** Part 1 Final: {:?}", occupied_in_row(&field, 2000000));
    let mut elapsed_time = start.elapsed();
    println!("{}", elapsed_time.as_secs_f64());
    start = Instant::now();
    println!("** Part 2 Final: {:?}", tuning_freq(&field));
    elapsed_time = start.elapsed();
    println!("{}", elapsed_time.as_secs_f64());
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BlipType {
    Sensor,
    Beacon,
}

#[derive(Clone, Copy, Debug)]
struct Blip {
    x: i64,
    y: i64,
    t: BlipType,
    md: i64,
}
impl Blip {
    fn coords(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn contains(&self, px: i64, py: i64) -> bool {
        (self.x - px).abs() + (self.y - py).abs() <= self.md
    }
}
impl hash::Hash for Blip {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl cmp::PartialEq for Blip {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl cmp::Eq for Blip {}

#[derive(Clone, Debug)]
struct Bounds {
    min: i64,
    max: i64,
}
impl Default for Bounds {
    fn default() -> Bounds {
        Bounds {
            min: std::i64::MAX,
            max: std::i64::MIN,
        }
    }
}
impl Bounds {
    fn grow(&mut self, v: i64) {
        self.min = cmp::min(self.min, v);
        self.max = cmp::max(self.max, v);
    }

    fn contains(&self, i: i64) -> bool {
        self.min <= i && i <= self.max
    }

    fn range(&self) -> Range<i64> {
        Range {
            start: self.min,
            end: self.max + 1,
        }
    }
}

fn to_int(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

fn grow(x: &mut Bounds, y: &mut Bounds, b: &Blip) {
    x.grow(b.x);
    y.grow(b.y);
}

#[derive(Clone, Debug)]
struct Field {
    data: HashSet<Blip>,
    sensor_w: Bounds,
    sensor_h: Bounds,
}
impl Field {
    fn new(input: &[String]) -> Field {
        lazy_static! {
            static ref POSITION: Regex = Regex::new(
                r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)"
            )
            .unwrap();
        }
        let mut data = HashSet::new();
        let mut sensor_w = Bounds { min: 0, max: 0 };
        let mut sensor_h = Bounds { min: 0, max: 0 };

        for line in input.iter() {
            if line.is_empty() {
                continue;
            } else if POSITION.is_match(line) {
                let caps = POSITION.captures(line).unwrap();
                let beacon = Blip {
                    x: to_int(&caps[3]),
                    y: to_int(&caps[4]),
                    t: BlipType::Beacon,
                    md: 0,
                };
                data.insert(beacon);

                let x = to_int(&caps[1]);
                let y = to_int(&caps[2]);
                let md = (x - &beacon.x).abs() + (y - &beacon.y).abs();

                let sensor = Blip {
                    x,
                    y,
                    t: BlipType::Sensor,
                    md,
                };
                grow(&mut sensor_w, &mut sensor_h, &sensor);
                data.insert(sensor);
            } else {
                panic!("What is this? {:?}", line);
            }
        }

        Field {
            data,
            sensor_w,
            sensor_h,
        }
    }

    fn get_bounds(&self, min: i64, max: i64) -> (Bounds, Bounds) {
        let sensor_w = Bounds {
            min: cmp::max(self.sensor_w.min, min),
            max: cmp::min(self.sensor_w.max, max),
        };
        let sensor_h = Bounds {
            min: cmp::max(self.sensor_h.min, min),
            max: cmp::min(self.sensor_h.max, max),
        };
        (sensor_w, sensor_h)
    }
}

fn occupied_in_row(field: &Field, row: i64) -> i64 {
    let mut occupied: HashSet<(i64, i64)> = HashSet::new();
    let mut beacons = 0;

    field
        .data
        .iter()
        // Find sensors and beacons within manhattan distance of target row
        .filter(|b| match b.t {
            BlipType::Beacon => b.y == row,
            BlipType::Sensor => (b.y - row).abs() <= b.md,
        })
        // Find intersection of manhattan distance with target row
        .for_each(|b| match b.t {
            BlipType::Beacon => {
                occupied.insert(b.coords());
                beacons += 1;
            }
            BlipType::Sensor => {
                let x_delta = b.md - (b.y - row).abs();
                for i in 0..=x_delta {
                    occupied.insert((b.x + i, row));
                    occupied.insert((b.x - i, row));
                }
            }
        });
    occupied.len() as i64 - beacons
}

fn in_the_gap(sensors: &Vec<&Blip>, w: &Bounds, h: &Bounds, x: i64, y: i64) -> bool {
    if w.contains(x) && h.contains(y) {
        let mut found = false;
        for s in sensors {
            found |= s.contains(x, y);
            if found {
                break;
            }
        }
        if !found {
            return true;
        }
    }
    false
}

fn tuning_freq(field: &Field) -> i64 {
    let (sensor_w, sensor_h) = field.get_bounds(0, 4000000);
    let mut all_sensors: Vec<&Blip> = vec![];
    let sensors: Vec<&Blip> = field
        .data
        .iter()
        // Find sensors within range
        .filter(|b| b.t == BlipType::Sensor && sensor_h.contains(b.y) && sensor_w.contains(b.x))
        // Collect those sensors
        .inspect(|x| all_sensors.push(&x))
        // Compare then against each other...
        .combinations(2)
        // Find those combinations that abutt each other, but with a gap remaining
        .filter(|v| ((v[0].x - v[1].x).abs() + (v[0].y - v[1].y).abs()) - (v[0].md + v[1].md) == 2)
        .flat_map(|c| c)
        .unique_by(|k| (k.x, k.y))
        .collect();

    for b in &sensors {
        let target = b.md + 1;
        for y_delta in 0..=target {
            let x_delta = target - y_delta;
            if in_the_gap(
                &all_sensors,
                &sensor_w,
                &sensor_h,
                b.x + x_delta,
                b.y + y_delta,
            ) {
                return ((b.x + x_delta) * 4000000) + b.y + y_delta;
            }
            if in_the_gap(
                &all_sensors,
                &sensor_w,
                &sensor_h,
                b.x - x_delta,
                b.y + y_delta,
            ) {
                return ((b.x - x_delta) * 4000000) + b.y + y_delta;
            }
            if in_the_gap(
                &all_sensors,
                &sensor_w,
                &sensor_h,
                b.x + x_delta,
                b.y - y_delta,
            ) {
                return ((b.x + x_delta) * 4000000) + b.y - y_delta;
            }
            if in_the_gap(
                &all_sensors,
                &sensor_w,
                &sensor_h,
                b.x - x_delta,
                b.y - y_delta,
            ) {
                return ((b.x - x_delta) * 4000000) + b.y - y_delta;
            }
        }
    }
    0
}

fn insert(map: &mut HashMap<(i64, i64), i64>, coord: (i64, i64)) {
    map.entry(coord).and_modify(|e| *e += 1).or_insert(1);
}

#[allow(dead_code)]
fn bad_brute_force(field: &Field) -> i64 {
    // The Brutiest Brute Force:
    let (sensor_w, sensor_h) = field.get_bounds(0, 4000000);

    let mut candidate: HashMap<(i64, i64), i64> = HashMap::new();
    field
        .data
        .iter()
        // Find sensors and beacons within range
        .filter(|b| sensor_h.contains(b.y) && sensor_w.contains(b.x))
        .for_each(|b| match b.t {
            BlipType::Beacon => {
                candidate.insert(b.coords(), i64::MIN);
            }
            BlipType::Sensor => {
                for y_delta in (-1 * b.md)..=b.md {
                    let y = b.y + y_delta;
                    let x_delta = b.md - y_delta.abs();
                    insert(&mut candidate, (b.x, y));
                    for i in 1..=x_delta {
                        insert(&mut candidate, (b.x + i, y));
                        insert(&mut candidate, (b.x - i, y));
                    }
                }
            }
        });
    for x in sensor_w.range() {
        for y in sensor_h.range() {
            if !candidate.contains_key(&(x, y)) {
                return (x * 4000000) + y;
            }
        }
    }
    println!("{:?}", candidate);
    panic!("WTH did I do... ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = puzzle_input::split_string(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        );

        let field = Field::new(&input);
        assert_eq!(occupied_in_row(&field, 10), 26);
        assert_eq!(tuning_freq(&field), 56000011); // works! But SLLOOOWWW
                                                   //assert_eq!(tuning_freq(&field), 56000011);
    }
}
