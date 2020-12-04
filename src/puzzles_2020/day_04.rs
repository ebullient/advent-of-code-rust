extern crate regex;
extern crate lazy_static;

use crate::puzzle_input;
use std::collections::HashMap;
use regex::Regex;

pub fn run() {
    let input = puzzle_input::read_all_lines("./input/2020-d04-input1.txt");
    let (required, valid) = validate(&input);
    println!("** Part 1 Final: {:?}", required);
    println!("** Part 1 Final: {:?}", valid);
}

#[derive(Debug,PartialEq)]
enum Validity {
    Missing,
    Present,
    Valid
}

static REQ_KEYS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
static MISSING: Option<&Validity> = Some(&Validity::Missing);

fn has_all_required<'a>(map: &HashMap<&'a str, Validity>) -> bool {
    REQ_KEYS.iter().filter(|x| !map.contains_key(*x)).count() == 0
}

fn is_valid<'a>(map: &HashMap<&'a str, Validity>) -> bool {
    REQ_KEYS.iter().filter(|x| *map.get(*x).or(MISSING).unwrap() != Validity::Valid).count() == 0
}

fn check<'a>(map: &mut HashMap<&'a str, Validity>, line: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]*):(.*)$").unwrap();
        static ref YR: Regex = Regex::new(r"^\d{4}$").unwrap();
        static ref HT: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        static ref HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        static ref ECL: Regex = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    line.split_whitespace().for_each(|x| {
        for cap in RE.captures_iter(x) {
            match &cap[1] {
                "byr" => map.insert("byr", if YR.is_match(&cap[2]) && in_year_range(&cap[2], 1920, 2002) {
                        Validity::Valid
                    } else {
                        Validity::Present
                    }),
                "iyr" => map.insert("iyr", if YR.is_match(&cap[2]) && in_year_range(&cap[2], 2010, 2020) {
                        Validity::Valid
                    } else {
                        Validity::Present
                    }),
                "eyr" => map.insert("eyr", if YR.is_match(&cap[2]) && in_year_range(&cap[2], 2020, 2030) {
                        Validity::Valid
                    } else {
                        Validity::Present
                    }),
                "hgt" => map.insert("hgt", if let Some(ht_caps) = HT.captures(&cap[2]) {
                        if in_height_range(&ht_caps[1], &ht_caps[2]) {
                            Validity::Valid
                        } else {
                            Validity::Present
                        }
                    } else {
                        Validity::Present
                    }),
                "hcl" => map.insert("hcl", if HCL.is_match(&cap[2]) {
                        Validity::Valid
                    } else {
                        Validity::Present
                    }),
                "ecl" => map.insert("ecl", if ECL.is_match(&cap[2]) {
                        Validity::Valid
                    } else {
                        Validity::Present
                    }),
                "pid" => map.insert("pid", if PID.is_match(&cap[2]) {
                        Validity::Valid
                    } else {
                        Validity::Present
                    }),
                "cid" => map.insert("cid", Validity::Present),
                _ => None
            };
        }
    });
}

fn in_year_range(y: &str, min: i32, max: i32) -> bool {
    let year = y.parse::<i32>().unwrap();
    min <= year && year <= max
}

fn in_height_range(h: &str, u: &str) -> bool {
    let height = h.parse::<i32>().unwrap();
    (u == "cm" && 150 <= height && height <= 193) ||
    (u == "in" && 59 <= height && height <= 76 )
}

fn validate(batch: &Vec<String>) -> (i32, i32) {
    let mut map: HashMap<&str, Validity> = HashMap::new();
    let mut present = 0;
    let mut valid = 0;
    for line in batch {
        let s = line.trim();
        if s.is_empty() {
            present += if has_all_required(&map) { 1 } else { 0 };
            valid += if is_valid(&map) { 1 } else { 0 };
            map.clear();
        } else {
            check(&mut map, &line);
        }
    }
    present += if has_all_required(&map) { 1 } else { 0 };
    valid += if is_valid(&map) { 1 } else { 0 };

    (present, valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_passport() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let (required, _) = validate(&input.split('\n').map(|x| x.to_string()).collect());
        assert_eq!(required, 2);
    }

    #[test]
    fn test_invalid_passports() {
        let input = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

        let (required, valid) = validate(&input.split('\n').map(|x| x.to_string()).collect());
        assert_eq!(required, 4);
        assert_eq!(valid, 0);
    }

    #[test]
    fn test_valid_passports() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let (required, valid) = validate(&input.split('\n').map(|x| x.to_string()).collect());
        assert_eq!(required, 4);
        assert_eq!(valid, 4);
    }
}
