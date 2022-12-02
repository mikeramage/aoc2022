use crate::utils;

///Day 1 solution
pub fn day2() -> (usize, usize) {
    let input: Vec<Vec<String>> = utils::parse_input_space_sep_strings("input/day2.txt");
    calculate_score(&input)
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Strategy {
    Simple,
    WinDrawLose,
}

/// Figures out the number of calories carried by the most calorie-laden elf.    
fn calculate_score(input: &Vec<Vec<String>>) -> (usize, usize) {
    let mut score_day1: usize = 0;
    let mut score_day2: usize = 0;
    // println!("Strategy: {:?}", strategy);
    for pair in input {
        let opponent_shape: Shape = match pair[0].as_str() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Bad shape character {}", pair[0]),
        };

        let my_shape_simple: Shape = get_my_shape(&opponent_shape, Strategy::Simple, &pair[1]);
        let my_shape_windrawlose: Shape =
            get_my_shape(&opponent_shape, Strategy::WinDrawLose, &pair[1]);

        score_day1 += combine_scores(
            shape_score(&my_shape_simple),
            round_score(&opponent_shape, &my_shape_simple),
        );
        score_day2 += combine_scores(
            shape_score(&my_shape_windrawlose),
            round_score(&opponent_shape, &my_shape_windrawlose),
        );
    }

    (score_day1, score_day2)
}

fn get_my_shape(opponent_shape: &Shape, strategy: Strategy, input: &String) -> Shape {
    match strategy {
        Strategy::Simple => match input.as_str() {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Bad shape character {}", input),
        },
        Strategy::WinDrawLose => match input.as_str() {
            "X" => match opponent_shape {
                //lose
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            "Y" => match opponent_shape {
                //draw
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissors => Shape::Scissors,
            },
            "Z" => match opponent_shape {
                //win
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            _ => panic!("Bad shape character {}", input),
        },
    }
}

fn shape_score(shape: &Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn round_score(opponent_shape: &Shape, my_shape: &Shape) -> usize {
    match opponent_shape {
        Shape::Rock => match my_shape {
            Shape::Rock => 3,
            Shape::Paper => 6,
            Shape::Scissors => 0,
        },
        Shape::Paper => match my_shape {
            Shape::Rock => 0,
            Shape::Paper => 3,
            Shape::Scissors => 6,
        },
        Shape::Scissors => match my_shape {
            Shape::Rock => 6,
            Shape::Paper => 0,
            Shape::Scissors => 3,
        },
    }
}

fn combine_scores(shape_score: usize, round_score: usize) -> usize {
    shape_score + round_score
}
