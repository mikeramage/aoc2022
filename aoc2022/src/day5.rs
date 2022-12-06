use std::{usize, num};
use regex::Regex;

use crate::utils;

///Day 5 solution
pub fn day5() -> (usize, usize) {
    let lines: Vec<String> = utils::parse_input("input/day5_test.txt");
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    let re_stack_labels = Regex::new(r"^[\s\d]+$").unwrap();
    let re_stack_element = Regex::new(r"^[\[\s\w\]]+$").unwrap();
    let re_instruction = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut number_of_stacks: usize;

    //Initial pass to find the stack labels. All we need to do is get the count of stacks.
    for line in lines {
        if re_stack_labels.is_match(line) {
            for ch in line.split_whitespace() {
                number_of_stacks = ch.parse::<usize>().unwrap();
            }
        }
    }

    println!("Nstacks: {:?}", number_of_stacks);



    (part1, part2)
}

