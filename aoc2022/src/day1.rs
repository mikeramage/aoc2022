use crate::utils;

///Day 1 solution
pub fn day1() -> (usize, usize) {
    let elven_calories: Vec<String> = utils::parse_input_by_blank_lines("input/day1.txt");
    max_calories(&elven_calories)
}

/// Figures out the number of calories carried by the most calorie-laden elf.    
fn max_calories(elven_calories: &[String]) -> (usize, usize) {
    let mut calorie_sums_per_elf: Vec<usize> = elven_calories
        .iter()
        .map(|x| x.split('\n').map(|y| y.parse::<usize>().unwrap()).sum()).collect();
    calorie_sums_per_elf.sort();
    calorie_sums_per_elf.reverse();
    let day_1: usize = calorie_sums_per_elf[0];
    let day_2: usize = calorie_sums_per_elf[0..3].iter().sum::<usize>();
    (day_1, day_2)
}