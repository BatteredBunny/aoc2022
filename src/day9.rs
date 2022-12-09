use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Move {
    U,
    D,
    R,
    L,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn execute(&mut self, m: &Move) {
        match m {
            Move::R => self.x += 1,
            Move::L => self.x -= 1,
            Move::U => self.y -= 1,
            Move::D => self.y += 1,
        }
    }

    fn is_touching(&self, other: &Self) -> bool {
        (self.x == other.x || self.x == other.x + 1 || self.x == other.x - 1)
            && (self.y == other.y || self.y == other.y + 1 || self.y == other.y - 1)
    }

    fn follow(&mut self, other: &Self) {
        if !self.is_touching(other) {
            self.x += (other.x - self.x).clamp(-1, 1);
            self.y += (other.y - self.y).clamp(-1, 1);
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_whitespace().next().unwrap() {
            "U" => Move::U,
            "D" => Move::D,
            "R" => Move::R,
            "L" => Move::L,
            _ => return Err(String::from("Invalid")),
        })
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .flat_map(|line| {
            vec![line.parse().unwrap(); line.split_whitespace().nth(1).unwrap().parse().unwrap()]
        })
        .collect()
}

fn logic(moves: &[Move], l: usize) -> usize {
    let mut pieces = vec![Pos::new(); l];
    let mut visited: HashSet<Pos> = HashSet::from([Pos::new()]);

    for m in moves.iter() {
        pieces[0].execute(m);

        for i in 1..pieces.len() {
            let (first, second) = pieces.split_at_mut(i);
            second.first_mut().unwrap().follow(first.last().unwrap());
        }

        visited.insert(*pieces.last().unwrap());
    }

    visited.len()
}

#[aoc(day9, part1)]
pub fn part1(moves: &[Move]) -> usize {
    logic(moves, 2)
}

#[aoc(day9, part2)]
pub fn part2(moves: &[Move]) -> usize {
    logic(moves, 10)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day9::{input_generator, part1, part2};

    #[test]
    fn test_day9() {
        let input = input_generator(&read_to_string("input/2022/day9.txt").unwrap());

        assert_eq!(6256, part1(&input));
        assert_eq!(2665, part2(&input));
    }
}
