use std::collections::HashSet;
use std::usize;

use crate::utils;

enum Instruction {
    NOOP,
    ADDX,
}

///Day 10 solution
pub fn day10() -> (usize, usize) {
    let instructions = utils::parse_input_sep_strings("input/day10.txt", ' ');
    let mut x: i32 = 1;
    let mut cycle: i32 = 0; //*During* the first cycle, cycle is 0
    let inspection_cycle: HashSet<i32> = HashSet::from([20, 60, 100, 140, 180, 220]);
    // Just one long vector of pixels. We'll restructure for output.
    let mut pixel_map: Vec<char> = vec![];
    let crt_len: i32 = 40;
    let crt_len_usize: usize = 40;
    let mut total_signal_strength: i32 = 0;
    for item in &instructions {
        let instruction = match item[0].as_str() {
            "addx" => Instruction::ADDX,
            "noop" => Instruction::NOOP,
            _ => panic!("Bad instruction"),
        };

        let value: Option<i32> = match &instruction {
            Instruction::ADDX => Some(item[1].parse::<i32>().unwrap()),
            Instruction::NOOP => None,
        };

        //Part 1 - look at inspection cycle
        if inspection_cycle.contains(&(cycle + 1)) {
            //Current value will hold during the inspection cycle - update total signal strength
            total_signal_strength += (cycle + 1) * x;
        } else if inspection_cycle.contains(&(cycle + 2)) {
            if let Instruction::ADDX = instruction {
                total_signal_strength += (cycle + 2) * x;
            }
        }

        //Part 2 - consider pixels
        match &instruction {
            Instruction::ADDX => {
                if ((cycle % crt_len) >= (x - 1)) && (cycle % crt_len) <= x + 1 {
                    pixel_map.insert(cycle.try_into().unwrap(), '#');
                } else {
                    pixel_map.insert(cycle.try_into().unwrap(), '.');
                }
                if ((cycle + 1) % crt_len) >= (x - 1) && ((cycle + 1) % crt_len) <= x + 1 {
                    pixel_map.insert((cycle + 1).try_into().unwrap(), '#');
                } else {
                    pixel_map.insert((cycle + 1).try_into().unwrap(), '.');
                }
            }
            Instruction::NOOP => {
                if ((cycle % crt_len) >= (x - 1)) && ((cycle % crt_len) <= (x + 1)) {
                    pixel_map.insert(cycle.try_into().unwrap(), '#');
                } else {
                    pixel_map.insert(cycle.try_into().unwrap(), '.');
                }
            }
        }

        execute_instruction(&instruction, &value, &mut x, &mut cycle);
    }

    let part1: usize = total_signal_strength.try_into().unwrap();

    for (index, pixel) in pixel_map.iter().enumerate() {
        if index != 0 && index % (crt_len_usize) == 0 {
            print!("\n");
        }
        print!("{}", pixel);
    }
    print!("\n");

    let part2: usize = 0;

    (part1, part2)
}

fn execute_instruction(
    instruction: &Instruction,
    value: &Option<i32>,
    x: &mut i32,
    cycle: &mut i32,
) {
    match instruction {
        Instruction::ADDX => {
            *x += value.unwrap();
            *cycle += 2
        }
        Instruction::NOOP => *cycle += 1,
    }
}
