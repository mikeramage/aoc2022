use std::usize;

use crate::utils;

///Day 8 solution
pub fn day8() -> (usize, usize) {
    let rows = utils::parse_input_usizes("input/day8.txt");
    let mut cols: Vec<Vec<usize>> = vec![];
    for ii in 0..rows.len() {
        for jj in 0..rows[ii].len() {
            if ii == 0 {
                cols.insert(jj, vec![rows[ii][jj]]);
            } else {
                cols[jj].insert(ii, rows[ii][jj]);
            }
        }
    }

    // println!("Rows: {:?}", rows);
    // println!("Cols: {:?}", cols);

    let mut part1: usize = 0;
    let mut part2: usize = 0;
    let mut score: usize;

    for ii in 0..rows.len() {
        for jj in 0..cols.len() {
            if is_visible(&rows, &cols, ii, jj) {
                part1 += 1;
            }

            score = scenic_score(&rows, &cols, ii, jj);
            if score > part2 {
                part2 = score;
            }
        }
    }

    (part1, part2)
}

// Utter garbage implementation. Please change.
fn is_visible(rows: &Vec<Vec<usize>>, cols: &Vec<Vec<usize>>, ii: usize, jj: usize) -> bool {
    // println!("Checking row {}, col{}: height{}", ii, jj, rows[ii][jj]);

    if ii == 0 || jj == 0 || ii == (rows.len() - 1) || jj == (cols.len() - 1) {
        return true;
    }

    let mut visible = true;
    //Visible from the left?
    for kk in 0..jj {
        // println!("  Check from left: {} >= {}?", rows[ii][kk], rows[ii][jj]);
        if rows[ii][kk] >= rows[ii][jj] {
            // println!("    No!");
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }
    visible = true;

    //from the right?
    for kk in jj + 1..cols.len() {
        // println!("  Check from right: {} >= {}?", rows[ii][kk], rows[ii][jj]);
        if rows[ii][kk] >= rows[ii][jj] {
            // println!("    No!");
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }
    visible = true;

    //from the top?
    for kk in 0..ii {
        // println!("  Check from top: {} >= {}?", cols[jj][kk], cols[jj][ii]);
        if cols[jj][kk] >= cols[jj][ii] {
            // println!("    No!");
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }
    visible = true;

    //from the bottom?
    for kk in ii + 1..rows.len() {
        // println!("  Check from bottom: {} >= {}?", cols[jj][kk], cols[jj][ii]);
        if cols[jj][kk] >= cols[jj][ii] {
            // println!("    No!");
            visible = false;
            break;
        }
    }

    visible
}

fn scenic_score(rows: &Vec<Vec<usize>>, cols: &Vec<Vec<usize>>, ii: usize, jj: usize) -> usize {
    let mut score: usize;
    let mut count: usize = 0;

    // println!("Checking row {}, col{}: height{}", ii, jj, rows[ii][jj]);

    //left score
    for kk in (0..jj).rev() {
        // println!("  Got tree to left, height: {}", rows[ii][kk]);
        count += 1;
        if rows[ii][kk] >= rows[ii][jj] {
            // println!("    Higher than this one. Stop");
            break;
        }
    }

    // println!("  count is: {}", count);

    score = count;
    count = 0;

    //right score
    for kk in jj + 1..rows.len() {
        // println!("  Got tree to right, height: {}", rows[ii][kk]);
        count += 1;
        if rows[ii][kk] >= rows[ii][jj] {
            // println!("    Higher than this one. Stop");
            break;
        }
    }

    // println!("  count is: {}", count);

    score *= count;
    count = 0;

    //top score
    for kk in (0..ii).rev() {
        // println!("    Got tree to top, height: {}", cols[jj][kk]);
        count += 1;
        if cols[jj][kk] >= cols[jj][ii] {
            // println!("    Higher than this one. Stop");
            break;
        }
    }

    // println!("  count is: {}", count);

    score *= count;
    count = 0;

    //left score
    for kk in ii + 1..cols.len() {
        // println!("    Got tree to bottom, height: {}", cols[jj][kk]);
        count += 1;
        if cols[jj][kk] >= cols[jj][ii] {
            // println!("    Higher than this one. Stop");
            break;
        }
    }

    // println!("  count is: {}", count);
    score *= count;

    // println!("score is: {}", score);

    score
}
