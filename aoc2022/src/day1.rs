use crate::utils;

///Day 1 solution
pub fn day1() -> (usize, usize) {
    let elven_calories: Vec<String> = utils::parse_input_by_blank_lines("input/day1.txt");
    (max_calories(&elven_calories, 1), max_calories(&elven_calories, 3))
    
}

/// Figures out the number of calories carried by the most calorie-laden elf.    
fn max_calories(elven_calories: &[String], num_elements: usize) -> usize {
    let mut calorie_sums_per_elf: Vec<usize> = elven_calories
        .iter()
        .map(|x| x.split('\n').map(|y| y.parse::<usize>().unwrap()).sum()).collect();
    calorie_sums_per_elf.sort();
    calorie_sums_per_elf.reverse();
    calorie_sums_per_elf[0..num_elements].iter().sum::<usize>()
}