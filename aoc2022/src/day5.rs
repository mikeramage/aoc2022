use regex::Regex;
use std::usize;

use crate::utils;

#[derive(Debug)]
struct Instruction {
    num_crates: usize,
    old_stack: usize,
    new_stack: usize,
}

///Day 5 solution
pub fn day5() -> (usize, usize) {
    let lines: Vec<String> = utils::parse_input("input/day5.txt");
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    let re_stack_labels = Regex::new(r"^[\s\d]+$").unwrap();
    let re_stack_element = Regex::new(r"^\s*(\[\w\]\s*)*$").unwrap();
    let re_instruction = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut number_of_stacks: usize = 0;
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    //Initial pass to find the stack labels. All we need to do is get the count of stacks, which is the label of last stack in the list
    for line in &lines {
        if re_stack_labels.is_match(&line) {
            for ch in line.split_whitespace() {
                number_of_stacks = ch.parse::<usize>().unwrap();
            }
            break;
        }
    }

    for ii in 0..number_of_stacks {
        stacks.push(vec![]);
    }

    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }

        if re_stack_element.is_match(&line) {
            for ii in 0..number_of_stacks {
                //Mystical numerology.  Crate labels occur at indices 1, 5, 9, ... so steps of 4 starting at 1, with the nth at index 4*n - 3 or 4*n + 1 for zero indexing
                let ch: char = line.chars().nth(4 * ii + 1).unwrap();
                if !ch.is_whitespace() {
                    stacks[ii].push(ch);
                }
            }
        } else if re_instruction.is_match(&line) {
            let captures = re_instruction.captures(line).unwrap();
            let instruction = Instruction {
                num_crates: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                old_stack: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                new_stack: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            };
            instructions.push(instruction);
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    let mut stacks_copy = stacks.clone();

    //Finally - it's all parsed! This bit's easy
    for instruction in &instructions {
        let mut substack: Vec<char> = vec![];
        for ii in 0..instruction.num_crates {
            // pop the crates off the old stack and onto the new
            let krate: char = stacks[instruction.old_stack - 1].pop().unwrap();
            stacks[instruction.new_stack - 1].push(krate);

            let krate2: char = stacks_copy[instruction.old_stack - 1].pop().unwrap();
            substack.push(krate2);
        }
        substack.reverse();
        stacks_copy[instruction.new_stack - 1].append(&mut substack);
    }

    // println!("Stacks: {:?}", stacks);
    // println!("Instructions: {:?}", instructions);

    let mut part1_string: String = "".to_string();
    let mut part2_string: String = "".to_string();

    for ii in 0..stacks.len() {
        part1_string.push(stacks[ii].pop().unwrap());
        part2_string.push(stacks_copy[ii].pop().unwrap());
    }

    println!("Part 1: {}", part1_string);
    println!("Part 2: {}", part2_string);

    (part1, part2)
}
