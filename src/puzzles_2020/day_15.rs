use std::collections::HashMap;

pub fn run() {
    let input = vec![1, 12, 0, 20, 8, 16];
    // Their question for you is: what will be the 2020th number spoken?
    // In the example above, the 2020th number spoken will be 436.
    println!("** Part 1 Final: {:?}", find_number(&input, 2020));
    println!("** Part 2 Final: {:?}", find_number(&input, 30000000));
}

fn find_number(seed: &Vec<u32>, target: u32) -> u32 {
    let mut memory: HashMap<u32, u32> = HashMap::new();
    let mut t = 1;
    let mut last = 0;

    for s in seed {
        memory.insert(*s, t);
        last = *s;
        t += 1;
    }
    println!("Starting: {:?} {:?} .. target {:?}", t, last, target);
    loop {
        let when = memory.insert(last, t - 1);
        if when == None {
            last = 0;
        } else {
            last = t - 1 - when.unwrap();
        }

        if t == target {
            return last as u32;
        }
        t += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_1() {
        assert_eq!(find_number(vec![0, 3, 6], 4), 0);
        assert_eq!(find_number(vec![0, 3, 6], 7), 1);
        assert_eq!(find_number(vec![0, 3, 6], 10), 0);
        assert_eq!(find_number(vec![0, 3, 6], 2020), 436);

        assert_eq!(find_number(vec![1, 3, 2], 2020), 1);
        assert_eq!(find_number(vec![2, 1, 3], 2020), 10);
        assert_eq!(find_number(vec![1, 2, 3], 2020), 27);

        assert_eq!(find_number(vec![2, 3, 1], 2020), 78);
        assert_eq!(find_number(vec![3, 2, 1], 2020), 438);
        assert_eq!(find_number(vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_memory_2() {
        assert_eq!(find_number(vec![0, 3, 6], 30000000), 175594);
        assert_eq!(find_number(vec![1, 3, 2], 30000000), 2578);
    }
}
