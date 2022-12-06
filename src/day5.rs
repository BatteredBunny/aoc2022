use std::{fs::read_to_string, str::FromStr};

use itertools::Itertools;
use regex::Regex;

lazy_static! {
    static ref MOVE_PARSE_REGEX: regex::Regex =
        Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    static ref LAST_NUM_REGEX: regex::Regex = Regex::new("([0-9]) $").unwrap();
    static ref CRATES_FIND_REGEX: regex::Regex = Regex::new(r"(?:\[([A-Z])\]|   )(?: |$)").unwrap();
}

#[derive(Clone)]
struct Row(Vec<char>);

impl Row {
    fn prepend_same_order(&mut self, letters: Vec<char>) {
        self.0 = letters
            .into_iter()
            .chain(self.0.clone().into_iter())
            .collect();
    }

    fn prepend(&mut self, letters: Vec<char>) {
        self.0 = letters
            .into_iter()
            .rev()
            .chain(self.0.clone().into_iter())
            .collect();
    }

    fn take(&mut self, amount: usize) -> Vec<char> {
        (0..amount).map(|_| self.0.remove(0)).rev().collect()
    }

    fn new() -> Self {
        Self(Vec::new())
    }
}

struct Move {
    amount: usize,
    cur_pos: usize,
    new_pos: usize,
}

impl Move {
    fn execute_same_order(&self, modify: &mut [Row]) {
        let letters = modify[self.cur_pos - 1].take(self.amount);
        modify[self.new_pos - 1].prepend_same_order(letters);
    }

    fn execute(&self, modify: &mut [Row]) {
        let letters = modify[self.cur_pos - 1].take(self.amount);
        modify[self.new_pos - 1].prepend(letters);
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = MOVE_PARSE_REGEX.captures(s).unwrap();

        Ok(Move {
            amount: captures.get(1).unwrap().as_str().parse().unwrap(),
            cur_pos: captures.get(2).unwrap().as_str().parse().unwrap(),
            new_pos: captures.get(3).unwrap().as_str().parse().unwrap(),
        })
    }
}

fn logic(execute_move: &dyn Fn(&Move, &mut [Row])) -> String {
    let (crates, moves) = read_to_string("inputs/day5.txt")
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect_tuple::<(String, String)>()
        .unwrap();

    let rows_amount: usize = LAST_NUM_REGEX
        .captures(&crates)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let mut rows: Vec<Row> = vec![Row::new(); rows_amount];

    for (i, char) in crates.lines().flat_map(|line| {
        CRATES_FIND_REGEX
            .captures_iter(line)
            .enumerate()
            .filter_map(|(i, c)| c.get(1).map(|m| (i, m.as_str().chars().next().unwrap())))
    }) {
        rows[i].0.push(char);
    }

    for mov in moves.lines().flat_map(Move::from_str) {
        execute_move(&mov, &mut rows);
    }

    rows.iter().map(|row| row.0.first().unwrap()).collect()
}

pub fn part1() -> String {
    logic(&Move::execute)
}

pub fn part2() -> String {
    logic(&Move::execute_same_order)
}

#[cfg(test)]
mod tests {
    use crate::day5::{part1, part2};
    use test::Bencher;

    use super::{logic, Move};

    #[test]
    fn day5_test() {
        assert_eq!("FCVRLMVQP", part1());
        assert_eq!("RWLWGJGFD", part2());
    }

    #[bench]
    fn day5_bench(b: &mut Bencher) {
        b.iter(|| logic(&Move::execute));
        b.iter(|| logic(&Move::execute_same_order));
    }
}
