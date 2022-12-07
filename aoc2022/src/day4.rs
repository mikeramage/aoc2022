use std::usize;

use crate::utils;

///Day 4 solution
pub fn day4() -> (usize, usize) {
    let sections: Vec<Vec<String>> = utils::parse_input_sep_strings("input/day4.txt", ',');
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for pair in sections {
        let mut ranges: Vec<usize> = vec![];
        for elf in pair {
            for range_bound in elf.split('-') {
                ranges.push(range_bound.parse::<usize>().unwrap());
            }
        }

        if check_for_containment(&ranges) {
            part1 += 1;
        }

        if check_for_overlap(&ranges) {
            part2 += 1;
        }
    }

    (part1, part2)
}

/// Takes a vector (lower_1, upper_1, lower_2, upper_2). Returns true if one of the ranges lower_i -> upper_i fully contains the other.  
fn check_for_containment(ranges: &[usize]) -> bool {
    (ranges[0] <= ranges[2] && ranges[1] >= ranges[3])
        || (ranges[2] <= ranges[0] && ranges[3] >= ranges[1])
}

/// Takes a vector (lower_1, upper_1, lower_2, upper_2). Returns true if one of the ranges lower_i -> upper_i overlaps in some way with the other.  
fn check_for_overlap(ranges: &[usize]) -> bool {
    (ranges[0] >= ranges[2] && ranges[0] <= ranges[3])
        || (ranges[1] >= ranges[2] && ranges[1] <= ranges[3])
        || check_for_containment(ranges)
}
