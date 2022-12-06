use std::usize;

use crate::utils;

///Day 1 solution
pub fn day3() -> (usize, usize) {
    let rucksacks: Vec<Vec<char>> = utils::parse_input_chars("input/day3.txt");
    let mut part1_priorities: usize = 0;
    let mut part2_priorities: usize = 0;
    for rucksack in &rucksacks {
        'outer: for ch in &rucksack[0..rucksack.len() / 2] {
            for ch2 in &rucksack[rucksack.len() / 2..] {
                if ch == ch2 {
                    part1_priorities += char_to_priority(ch);
                    break 'outer;
                }
            }
        }
    }

    for ii in (0..rucksacks.len()).step_by(3) {
        'outer: for ch in &rucksacks[ii] {
            for ch2 in &rucksacks[ii + 1] {
                if ch == ch2 {
                    for ch3 in &rucksacks[ii + 2] {
                        if ch == ch3 {
                            part2_priorities += char_to_priority(ch3);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    (part1_priorities, part2_priorities)
}

/// Convert a character to a numberic value indicating the priority [a-z] -> [1-26], [A-Z] -> [27-52]
fn char_to_priority(ch: &char) -> usize {
    assert!(ch.is_ascii_alphabetic());

    if ch.is_ascii_lowercase() {
        (*ch as usize) - ('a' as usize) + 1
    } else {
        (*ch as usize) - ('A' as usize) + 27
    }
}
