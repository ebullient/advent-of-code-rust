use crate::puzzle_input;

pub fn run() {
    let input: Vec<usize> = puzzle_input::read_string("./input/2021-d06-input.txt")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // this is a matrix math problem, but I have to .. do a bunch of stuff. So this is the dumb way
    let mut calc = Calculator::new(&input);
    for _ in 0..80 {
        calc.iterate_the_long_way();
    }

    println!("** Part 1 Final: {:?}", calc.sum());
    assert_eq!(355386, calc.sum());

    for _ in 80..256 {
        calc.iterate_the_long_way();
    }
    println!("** Part 2 Final: {:?}", calc.sum());
    assert_eq!(1613415325809, calc.sum());
}

#[derive(Clone, Debug)]
struct Calculator {
    a: [i64; 9],
    b: [i64; 9],
    is_a: bool,
}
impl Calculator {
    pub fn new(input: &[usize]) -> Calculator {
        let mut init = [0; 9];

        for x in input {
            init[*x] += 1;
        }

        Calculator {
            a: init,
            b: [0; 9],
            is_a: true,
        }
    }

    fn iterate_the_long_way(&mut self) {
        let (prev, data) = if self.is_a {
            (&self.a, &mut self.b)
        } else {
            (&self.b, &mut self.a)
        };
        for i in 0..8 {
            match i {
                0 => {
                    data[i] = prev[i + 1];
                    data[6] = prev[7] + prev[0];
                    data[8] = prev[0];
                }
                6 | 8 => {
                    // no-op
                }
                _ => data[i] = prev[i + 1],
            }
        }
        self.is_a = !self.is_a;
    }

    #[allow(dead_code)]
    fn compare(&self, expected: [i64; 9]) -> bool {
        let data = if self.is_a { &self.a } else { &self.b };

        println!("{:?} == {:?}", expected, data);
        *data == expected
    }

    fn sum(&self) -> i64 {
        let data = if self.is_a { &self.a } else { &self.b };
        data.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Validate using test data
    fn count(input: &str) -> (Vec<usize>, [i64; 9]) {
        let mut tally = [0; 9];

        let data: Vec<usize> = input
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for x in data.iter() {
            tally[*x] += 1;
        }

        (data, tally)
    }

    #[test]
    fn test_the_long_way() {
        let init = String::from("3,4,3,1,2");
        let (input, tally) = count(&init);
        let mut calc = Calculator::new(&input);
        assert!(calc.compare(tally));
        assert!(calc.compare([0, 1, 1, 2, 1, 0, 0, 0, 0]));

        // calc.iterate_the_long_way();
        // assert_eq!(true, calc.compare([1,1,2,1,0,0,0,0,0]));

        // calc.iterate_the_long_way();
        // assert_eq!(true, calc.compare([1,2,1,0,0,0,1,0,1]));

        // calc.iterate_the_long_way();
        // assert_eq!(true, calc.compare([2,1,0,0,0,1,1,1,1]));

        for _ in 0..18 {
            calc.iterate_the_long_way();
        }
        let (_, tally) = count(&String::from(
            "6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8",
        ));
        assert!(calc.compare(tally));
        assert_eq!(26, calc.sum());

        for _ in 18..80 {
            calc.iterate_the_long_way();
        }
        assert_eq!(5934, calc.sum());
    }
}
