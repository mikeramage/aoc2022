use itertools::Itertools;
use std::fs;
use std::usize;

///Day 6 solution
pub fn day6() -> (usize, usize) {
    let input = fs::read_to_string("input/day6.txt").expect("Oh dear, couldn't read file!");
    (get_message_start(&input, 4), get_message_start(&input, 14))
}

fn get_message_start(input: &String, num_unique_chars: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(num_unique_chars)
        .position(|x| x.iter().unique().count() == num_unique_chars)
        .unwrap()
        + num_unique_chars
}

