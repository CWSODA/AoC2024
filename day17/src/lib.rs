pub fn solve(input: &str) -> u64 {
    let (reg, instructions) = parse(input);

    // P1
    let mut output_string = String::new();
    for num in run(reg, &instructions) {
        output_string += &format!("{num},");
    }
    output_string.pop();
    println!("{output_string}");

    // P2
    let mut possibles: Vec<u64> = vec![0];
    let mut count = 1;
    loop {
        let mut current_vec = vec![];
        let pattern = &instructions[(instructions.len() - count)..];

        for possible in possibles {
            for plus in 0b000u64..=0b111u64 {
                let new_reg = to_reg(possible, plus);
                // println!("New: {:?}", new_reg);
                if matches(new_reg, pattern, &instructions) {
                    current_vec.push(new_reg[0]);
                }
            }
        }
        possibles = current_vec;

        // println!("Possibles at {count}:");
        // _print_binary(&possibles);

        count += 1;
        if count > instructions.len() {
            // smallest number is index 0 of possibles since range from 0b000 to 0b111
            return possibles[0];
        }
    }
}

fn run(mut reg: [u64; 3], instructions: &[u8]) -> Vec<u64> {
    let mut pointer = 0;
    let mut output = vec![];

    while let (Some(&opcode), Some(&operand)) =
        (instructions.get(pointer), instructions.get(pointer + 1))
    {
        match opcode {
            0 => reg[0] = reg[0] / 2u64.pow(combo(operand, &reg) as u32),
            1 => reg[1] = reg[1] ^ operand as u64,
            2 => reg[1] = combo(operand, &reg) % 8,
            3 => {
                if reg[0] != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => reg[1] = reg[1] ^ reg[2],
            5 => output.push(combo(operand, &reg) % 8),
            6 => reg[1] = reg[0] / 2u64.pow(combo(operand, &reg) as u32),
            7 => reg[2] = reg[0] / 2u64.pow(combo(operand, &reg) as u32),
            _ => {
                panic!("Invalid opcode!")
            }
        }
        pointer += 2;
    }

    output
}

fn matches(mut reg: [u64; 3], pattern: &[u8], instructions: &[u8]) -> bool {
    let mut pointer = 0;
    let mut count = 0;

    while let (Some(&opcode), Some(&operand)) =
        (instructions.get(pointer), instructions.get(pointer + 1))
    {
        match opcode {
            0 => reg[0] = reg[0] / 2u64.pow(combo(operand, &reg) as u32),
            1 => reg[1] = reg[1] ^ operand as u64,
            2 => reg[1] = combo(operand, &reg) % 8,
            3 => {
                if reg[0] != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => reg[1] = reg[1] ^ reg[2],
            5 => {
                let out = combo(operand, &reg) % 8;
                // must be equal lengths, avoids index out of bounds
                if count >= pattern.len() {
                    return false;
                }
                if out != pattern[count] as u64 {
                    return false;
                }
                count += 1;
            }
            6 => reg[1] = reg[0] / 2u64.pow(combo(operand, &reg) as u32),
            7 => reg[2] = reg[0] / 2u64.pow(combo(operand, &reg) as u32),
            _ => {
                panic!("Invalid opcode!")
            }
        }
        pointer += 2;
    }

    true
}

// note that B and C are 0
fn to_reg(possible: u64, plus: u64) -> [u64; 3] {
    [(possible << 3) + plus, 0, 0]
}

// returns registers, instructions
fn parse(input: &str) -> ([u64; 3], Vec<u8>) {
    let mut reg = [0; 3];

    let mut iter = input.lines();

    reg[0] = iter
        .next()
        .expect("No first line!")
        .split(|ch: char| !ch.is_numeric())
        .filter(|slice| !slice.is_empty())
        .map(|slice| slice.parse::<u64>().unwrap())
        .next()
        .unwrap();

    reg[1] = iter
        .next()
        .expect("No Second line!")
        .split(|ch: char| !ch.is_numeric())
        .filter(|slice| !slice.is_empty())
        .map(|slice| slice.parse::<u64>().unwrap())
        .next()
        .unwrap();

    reg[2] = iter
        .next()
        .expect("No Third line!")
        .split(|ch: char| !ch.is_numeric())
        .filter(|slice| !slice.is_empty())
        .map(|slice| slice.parse::<u64>().unwrap())
        .next()
        .unwrap();

    let nums: Vec<_> = iter
        .skip(1)
        .next()
        .expect("No instructions!")
        .split(|ch: char| !ch.is_numeric())
        .filter(|slice| !slice.is_empty())
        .map(|slice| slice.parse::<u8>().unwrap())
        .collect();

    (reg, nums)
}

fn combo(operand: u8, reg: &[u64; 3]) -> u64 {
    match operand {
        x if (x <= 3) => x as u64,
        4 => reg[0],
        5 => reg[1],
        6 => reg[1],
        _ => panic!("Invalid operand"),
    }
}

// for debug
fn _print_output(out: &[u64]) {
    for num in out {
        print!("{num},");
    }
    println!();
}

// for debug
fn _print_binary(out: &[u64]) {
    for num in out {
        println!("{num:b},");
    }
}

// combo:
// 0 -> 3 literals
// 4: regA
// 5: regB
// 6: regC
// 7: reserved

/*
opcodes:
0: division of regA by 2^combo, as int, written to regA
1: bitwise XOR of regB, written to regB
2: combo modulo 8, written to regB
3: jumps to literal if regA != 0
4: bitwise XOR of regB and regC, written to regB
5: como modulo 8, output
6: division of regA by 2^combo, as int, written to regB
7: division of regA by 2^combo, as int, written to regC
*/
