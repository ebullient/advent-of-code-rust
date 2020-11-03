fn opcode_1(i: usize, codes: &mut Vec<i32>) -> usize {
    // Add value from ix + value from iy, place in iz
    let ix: usize = codes[i+1] as usize;
    let iy: usize = codes[i+2] as usize;
    let iz: usize = codes[i+3] as usize;

    codes[iz] = codes[ix] + codes[iy];

    i+4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_2(i: usize, codes: &mut Vec<i32>) -> usize {
    // Multiply value from ix * value from iy, place in iz
    let ix: usize = codes[i+1] as usize;
    let iy: usize = codes[i+2] as usize;
    let iz: usize = codes[i+3] as usize;
 
    codes[iz] = codes[ix] * codes[iy];

    i+4 // advance 4: 1 opcode + 3 parameters
}

pub fn run(codes: &mut Vec<i32>) {
    let mut i: usize = 0;
    loop {
        match codes[i] {
            1 => { i = opcode_1(i, codes) },
            2 => { i = opcode_2(i, codes) },
            99 => { 
                break; 
            },
            _ => {
                println!("ERROR: {0} Unknown at index {1}", codes[i], i);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn intcode_program(input_ref: &str) -> String {
        let mut codes: Vec<i32> = input_ref.split(',')
                                           .map(|x| x.parse::<i32>().unwrap())
                                           .collect();
        run(&mut codes);
        codes.iter().join(",")
    }

    #[test]
    fn test_intcode_program() {
        assert_eq!(intcode_program("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(intcode_program("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(intcode_program("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(intcode_program("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
        assert_eq!(intcode_program("1,9,10,3,2,3,11,0,99,30,40,50"), "3500,9,10,70,2,3,11,0,99,30,40,50");
    }
}