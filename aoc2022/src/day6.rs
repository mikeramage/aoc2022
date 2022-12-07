use std::{usize, num};
use std::fs;
use itertools::Itertools;

use crate::utils;

#[derive(Debug)]
struct Instruction {
    num_crates: usize,
    old_stack: usize,
    new_stack: usize,
}

///Day 6 solution
pub fn day6() -> (usize, usize) {
    let input = fs::read_to_string("input/day6.txt").expect("Oh dear, couldn't read file!");
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    let mut chars_processed: usize = 4;

    for four_char_window in input.as_bytes().windows(4) {
        if four_char_window.into_iter().unique().count() == 4 {
            //All 4 characters unique!
            break;
        }
        chars_processed += 1;
    }

    part1 = chars_processed;
    
    chars_processed = 14;
    for fourteen_char_window in input.as_bytes().windows(14) {
        if fourteen_char_window.into_iter().unique().count() == 14 {
            //All 4 characters unique!
            break;
        }
        chars_processed += 1;
    }
    
    part2 = chars_processed;

    (part1, part2)
}

