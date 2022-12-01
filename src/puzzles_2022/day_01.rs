use crate::puzzle_input;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d01-input.txt");
    let (m1, m2, m3) = count_calories(&input);

    println!("** Part 1 Final: {:?}", m1);
    println!("** Part 2 Final: {:?}", m1 + m2 + m3);
}

fn count_calories(input: &[String]) -> (i32, i32, i32) {
    let mut m1 = 0;
    let mut m2 = 0;
    let mut m3 = 0;

    let mut current = 0;
    let mut capacity: Vec<i32> = Vec::new();

    for line in input {
        if line.is_empty() {
            capacity.push(current);
            if current > m1 {
                m3 = m2;
                m2 = m1;
                m1 = current;
            } else if current > m2 {
                m3 = m2;
                m2 = current;
            } else if current > m3 {
                m3 = current;
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap()
        }
    }

    (m1, m2, m3)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
        let input: Vec<String> = "1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "   .split('\n')
                .map(|x| x.trim().to_string())
                .collect();

        let (m1, m2, m3) = count_calories(&input);
        assert_eq!(m1, 24000);
        assert_eq!(m2, 11000);
        assert_eq!(m3, 10000);
        assert_eq!(m1 + m2 + m3, 45000);
    }
}
