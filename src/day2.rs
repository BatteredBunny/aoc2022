use std::str::FromStr;

const WINNING_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;

#[derive(PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => return Err(String::new()),
        })
    }
}

#[aoc_generator(day2, part1)]
fn input_generator_part1(input: &str) -> Vec<(Move, Move)> {
    input
        .lines()
        .filter_map(|raw| {
            let mut moves = raw.split_whitespace();
            Some((moves.next()?.parse().ok()?, moves.next()?.parse().ok()?))
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Move, Move)]) -> i32 {
    input
        .iter()
        .map(|(opponent_move, my_move)| {
            my_move.score()
                + match opponent_move {
                    Move::Rock => match my_move {
                        Move::Rock => DRAW_SCORE,
                        Move::Paper => WINNING_SCORE,
                        _ => 0,
                    },
                    Move::Paper => match my_move {
                        Move::Paper => DRAW_SCORE,
                        Move::Scissors => WINNING_SCORE,
                        _ => 0,
                    },
                    Move::Scissors => match my_move {
                        Move::Rock => WINNING_SCORE,
                        Move::Scissors => DRAW_SCORE,
                        _ => 0,
                    },
                }
        })
        .sum()
}

pub enum WantedOutCome {
    Loss, // X
    Draw, // Y
    Win,  // Z
}

impl FromStr for WantedOutCome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => WantedOutCome::Loss,
            "Y" => WantedOutCome::Draw,
            "Z" => WantedOutCome::Win,
            _ => return Err(String::new()),
        })
    }
}

#[aoc_generator(day2, part2)]
fn input_generator_part2(input: &str) -> Vec<(Move, WantedOutCome)> {
    input
        .lines()
        .filter_map(|raw| {
            let mut moves = raw.split_whitespace();
            Some((moves.next()?.parse().ok()?, moves.next()?.parse().ok()?))
        })
        .collect()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Move, WantedOutCome)]) -> i32 {
    input
        .iter()
        .map(|(opponent_move, wanted_outcome)| match opponent_move {
            Move::Rock => match wanted_outcome {
                WantedOutCome::Loss => Move::Scissors.score(),
                WantedOutCome::Draw => Move::Rock.score() + DRAW_SCORE,
                WantedOutCome::Win => Move::Paper.score() + WINNING_SCORE,
            },
            Move::Paper => match wanted_outcome {
                WantedOutCome::Loss => Move::Rock.score(),
                WantedOutCome::Draw => Move::Paper.score() + DRAW_SCORE,
                WantedOutCome::Win => Move::Scissors.score() + WINNING_SCORE,
            },
            Move::Scissors => match wanted_outcome {
                WantedOutCome::Loss => Move::Paper.score(),
                WantedOutCome::Draw => Move::Scissors.score() + DRAW_SCORE,
                WantedOutCome::Win => Move::Rock.score() + WINNING_SCORE,
            },
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day2::{input_generator_part1, input_generator_part2, part1, part2};

    #[test]
    fn test_day2() {
        let raw = read_to_string("input/2022/day2.txt").unwrap();

        assert_eq!(14163, part1(&input_generator_part1(&raw)));
        assert_eq!(12091, part2(&input_generator_part2(&raw)));
    }
}
