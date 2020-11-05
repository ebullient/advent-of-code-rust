pub fn run() {
    let mut n = 0;
    for input in 353096..=843212 {
        if is_valid(input) {
            n += 1;
        }
    }
    println!("** Part 1 Final: {0}", n);

    n = 0;
    for input in 353096..=843212 {
        if is_really_valid(input) {
            n += 1;
        }
    }
    println!("** Part 2 Final: {0}", n);
}

fn to_digits(input: i32) -> Vec<i32> {
    let mut x = input;
    let mut result: Vec<i32> = Vec::new();

    loop {
        result.push(x % 10);
        x /= 10;
        if x == 0 {break}
    }
    result.reverse();
    result
}

fn is_valid(password: i32) -> bool {
    let input = to_digits(password);

    let mut adjacent = false;
    let mut decrease = false;

    // 2. Two adjacent digits are the same (like 22 in 122345).
    // 3. Going from left to right, the digits never decrease; 
    //    they only ever increase or stay the same (like 111123 or 135679).
    for i in 1..6 {
        if input[i-1] > input[i] {
            decrease = true;
        } 
        if input[i-1] == input[i] {
            adjacent = true;
        } 
    }

    input.len() == 6 && adjacent && ! decrease
}

fn is_really_valid(password: i32) -> bool {
    let input = to_digits(password);

    let mut adjacent = false;
    let mut in_a_row = 1;
    let mut decrease = false;

    // 2. Two adjacent digits are the same (like 22 in 122345).
    // 4! the two adjacent matching digits are not part of a 
    //    larger group of matching digits.
    // 3. Going from left to right, the digits never decrease; 
    //    they only ever increase or stay the same (like 111123 or 135679).
    for i in 1..6 {
        if input[i-1] > input[i] {
            decrease = true;
        } 

        if input[i-1] == input[i] {
            in_a_row += 1;
        } else {
            if in_a_row == 2 {
                adjacent = true;
            }
            in_a_row = 1;
        }
    }
    if in_a_row == 2 {
        adjacent = true;
    }

    input.len() == 6 && adjacent && ! decrease
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digits() {
        assert_eq!(to_digits(223450), [2, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(111111), true);
        assert_eq!(is_valid(223450), false);
        assert_eq!(is_valid(123789), false);
    }

    #[test]
    fn test_is_really_valid() {
        assert_eq!(is_really_valid(111111), false);
        assert_eq!(is_really_valid(111122), true);
        assert_eq!(is_really_valid(112233), true);
        assert_eq!(is_really_valid(123444), false);
    }
}