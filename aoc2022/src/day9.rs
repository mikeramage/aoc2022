use std::collections::HashSet;
use std::usize;

use crate::utils;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

///Day 9 solution
pub fn day9() -> (usize, usize) {
    let instructions = utils::parse_input_sep_strings("input/day9.txt", ' ');
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited2: HashSet<(i32, i32)> = HashSet::new();
    let mut knot_pos: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    let mut knot_pos2: Vec<(i32, i32)> = vec![];
    for ii in 0..10 {
        knot_pos2.insert(ii, (0, 0));
    }
    visited.insert(knot_pos[knot_pos.len() - 1]);
    visited2.insert(knot_pos2[knot_pos2.len() - 1]);

    for instruction in &instructions {
        let direction = match instruction[0].as_str() {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("Unrecognized direction!"),
        };
        let num_steps = instruction[1].parse::<usize>().unwrap();

        update_head_and_knots(&mut knot_pos, &direction, num_steps, &mut visited);
        update_head_and_knots(&mut knot_pos2, &direction, num_steps, &mut visited2);
    }

    let part1: usize = visited.len();
    let part2: usize = visited2.len();

    (part1, part2)
}

fn update_head_and_knots(
    knot_pos: &mut Vec<(i32, i32)>,
    direction: &Direction,
    num_steps: usize,
    visited: &mut HashSet<(i32, i32)>,
) {
    for _step in 0..num_steps {
        //Update the head.
        match direction {
            &Direction::UP => knot_pos[0].1 += 1,
            &Direction::DOWN => knot_pos[0].1 -= 1,
            &Direction::LEFT => knot_pos[0].0 -= 1,
            &Direction::RIGHT => knot_pos[0].0 += 1,
        }

        // Update the rest of the knots
        for ii in 1..knot_pos.len() {
            knot_pos[ii] = update_knot(&knot_pos[ii - 1], &knot_pos[ii]);
        }

        visited.insert(knot_pos[knot_pos.len() - 1]);
    }
}

fn update_knot(prev_knot_pos: &(i32, i32), knot_pos: &(i32, i32)) -> (i32, i32) {
    let horizontal_diff = prev_knot_pos.0 - knot_pos.0;
    let vertical_diff = prev_knot_pos.1 - knot_pos.1;
    let mut new_knot_pos: (i32, i32) = (knot_pos.0, knot_pos.1);

    // 8 cases to consider here (this can probably be simplified, but it's clear to read like this):
    // 1. head directly above tail by more than 1 step. Tail moves up by one step
    if vertical_diff > 1 && horizontal_diff == 0 {
        new_knot_pos.1 += 1;
    }
    // 2. head directly below tail by more than 1 step. Tail moves down by one step
    else if vertical_diff < -1 && horizontal_diff == 0 {
        new_knot_pos.1 -= 1;
    }
    // 3. head directly to left of tail by more than 1 step. Tail moves left by one step
    else if vertical_diff == 0 && horizontal_diff < -1 {
        new_knot_pos.0 -= 1;
    }
    // 4. head directly to right of tail by more than 1 step. Tail moves right by one step
    else if vertical_diff == 0 && horizontal_diff > 1 {
        new_knot_pos.0 += 1;
    }
    // 5. head directly above tail by more than 1 step and left by 1 step or more OR above tail by 1 step or more and left by more than 1 step.
    // Tail moves up and left by one step
    else if (vertical_diff > 1 && horizontal_diff <= -1)
        || (vertical_diff >= 1 && horizontal_diff < -1)
    {
        new_knot_pos.0 -= 1;
        new_knot_pos.1 += 1;
    }
    // 6. head directly below tail by more than 1 step and left by 1 step or more OR below tail by 1 step or more and left by more than 1 step.
    // Tail moves down and left by one step
    else if (vertical_diff < -1 && horizontal_diff <= -1)
        || (vertical_diff <= -1 && horizontal_diff < -1)
    {
        new_knot_pos.0 -= 1;
        new_knot_pos.1 -= 1;
    }
    // 7. head directly above tail by more than 1 step and right by 1 step or more OR above tail by 1 step or more and right by more than 1 step.
    // Tail moves up and right by one step
    else if (vertical_diff > 1 && horizontal_diff >= 1)
        || (vertical_diff >= 1 && horizontal_diff > 1)
    {
        new_knot_pos.0 += 1;
        new_knot_pos.1 += 1;
    }
    // 8. head directly below tail by more than 1 step and right by 1 step or more OR below tail by 1 step or more and right by more than 1 step.
    // Tail moves down and right by one step
    else if (vertical_diff < -1 && horizontal_diff >= 1)
        || (vertical_diff <= -1 && horizontal_diff > 1)
    {
        new_knot_pos.0 += 1;
        new_knot_pos.1 -= 1;
    } else {
        assert!(vertical_diff.abs() <= 1 && horizontal_diff.abs() <= 1);
    }

    new_knot_pos
}
