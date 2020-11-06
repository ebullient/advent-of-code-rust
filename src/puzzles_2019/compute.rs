use std::io;

// A tuple struct
struct Modes(i32, i32, i32);

fn get_parameters(input: i32) -> (Modes, i32) {
    let mut x = input;
    let op = x % 100;
    x /= 100;
    let c = x % 10;
    x /= 10;
    let b = x % 10;
    x /= 10;
    let a = x;

    (Modes(a, b, c), op)
}

fn opcode_1(modes: Modes, i: usize, codes: &mut Vec<i32>) -> usize {
    // Add value from ix + value from iy, place in iz
    let x = 
        if modes.2 == 0 { // position mode
            let ix: usize = codes[i+1] as usize;
            codes[ix]
        } else { // immediate mode
            codes[i+1]
        };
    let y =
        if modes.1 == 0 { // position mode
            let iy: usize = codes[i+2] as usize;
            codes[iy]
        } else { // immediate mode
            codes[i+2]
        };

    let iz: usize = codes[i+3] as usize; // always position mode

    codes[iz] = x + y;

    i+4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_2(modes: Modes, i: usize, codes: &mut Vec<i32>) -> usize {
    // Multiply value from ix * value from iy, place in iz
    // Add value from ix + value from iy, place in iz
    let x = 
        if modes.2 == 0 { // position mode
            let ix: usize = codes[i+1] as usize;
            codes[ix]
        } else { // immediate mode
            codes[i+1]
        };
    let y =
        if modes.1 == 0 { // position mode
            let iy: usize = codes[i+2] as usize;
            codes[iy]
        } else { // immediate mode
            codes[i+2]
        };

    let iz: usize = codes[i+3] as usize; // always position mode

    codes[iz] = x * y;

    i+4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_3(_modes: Modes, i: usize, codes: &mut Vec<i32>) -> usize {
    // Opcode 3 takes a single integer as input and saves it to the 
    // position given by its only parameter. 
    let mut input = String::new();
    println!(">>");
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("read {} bytes: {}", n, input);
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    let ix: usize = codes[i+1] as usize;
    codes[ix] = input.trim().parse::<i32>().unwrap();

    i+2 // advance 2: 1 opcode + 1 parameter
}

fn opcode_4(modes: Modes, i: usize, codes: &mut Vec<i32>) -> usize {
    // Opcode 4 outputs the value of its only parameter. 
    // For example, the instruction 4,50 would output the value at address 50.
    if modes.2 == 0 { // position mode
        let ix: usize = codes[i+1] as usize;
        println!("{}", codes[ix]);
    } else { // immediate mode
        println!("{}", codes[i+1]);
    };

    i+2 // advance 2: 1 opcode + 1 parameter
}

pub fn run(codes: &mut Vec<i32>) {
    let mut i: usize = 0;
    //let mut 
    loop {
        let (modes, op) = get_parameters(codes[i]);
        //println!("{},{},{} {:?}", modes.0, modes.1, modes.2, op);
        match op {
            1 => { i = opcode_1(modes, i, codes) },
            2 => { i = opcode_2(modes, i, codes) },
            3 => { i = opcode_3(modes, i, codes) },
            4 => { i = opcode_4(modes, i, codes) },
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

    #[test]
    fn test_parameter_mode() {
        let (modes, op) = get_parameters(12345);
        assert_eq!(modes.0, 1);
        assert_eq!(modes.1, 2);
        assert_eq!(modes.2, 3);
        assert_eq!(op, 45);
    }

    #[test]
    fn test_parameter_mode_2() {
        let (modes, op) = get_parameters(1002);
        assert_eq!(modes.0, 0);
        assert_eq!(modes.1, 1);
        assert_eq!(modes.2, 0);
        assert_eq!(op, 02);
    }
}