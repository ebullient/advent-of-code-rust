fn opcode_1(i: usize, codes: &mut Vec<i32>) -> usize {
    let ix: usize = codes[i+1] as usize;
    let iy: usize = codes[i+2] as usize;
    let iz: usize = codes[i+3] as usize;
    let x: i32 = codes[ix];
    let y: i32 = codes[iy];
    let z: i32 = x + y;
//    println!("[ {0}] ADD value from {1}({4}) by value from {2}({5}), place in {3} ({6})", codes[i], ix, iy, iz, x, y, z);
    codes[iz] = z;

    i+4 // advance 4: 1 opcode + 3 parameters
}

fn opcode_2(i: usize, codes: &mut Vec<i32>) -> usize {
    let ix: usize = codes[i+1] as usize;
    let iy: usize = codes[i+2] as usize;
    let iz: usize = codes[i+3] as usize;
    let x: i32 = codes[ix];
    let y: i32 = codes[iy];
    let z: i32 = x * y;

//    println!("[ {0}] MULTIPLY value from {1}({4}) by value from {2}({5}), place in {3} ({6})", codes[i], ix, iy, iz, x, y, z);
    codes[iz] = z;

    i+4 // advance 4: 1 opcode + 3 parameters
}

pub fn run(codes: &mut Vec<i32>) {
    let mut i: usize = 0;
//    println!("BEGIN:\n{:?}", codes);
    loop {
        match codes[i] {
            1 => { i = opcode_1(i, codes) },
            2 => { i = opcode_2(i, codes) },
            99 => { 
//                println!("[99] Finished"); 
                break; 
            },
            _ => {
                println!("ERROR: {0} Unknown at index {1}", codes[i], i);
                break;
            }
        }
    }
//    println!("END: \n{:?}", codes);
}