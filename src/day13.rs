use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum Row {
    Array(Vec<Row>),
    Num(i32),
}

impl FromStr for Row {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl PartialOrd<Self> for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Row::Num(num) = self && let Row::Num(num2) = other {
            num.cmp(num2)
        } else if let Row::Array(nums) = self && let Row::Array(nums2) = other {
            nums.iter().zip_longest(nums2.iter()).map(|ord| match ord {
                    itertools::EitherOrBoth::Both(left, right) => left.cmp(right),
                    itertools::EitherOrBoth::Left(_) => Ordering::Greater,
                    itertools::EitherOrBoth::Right(_) => Ordering::Less,
                })
            .find(|ord| ord != &Ordering::Equal)
            .unwrap_or(Ordering::Equal)
        } else if let Row::Num(_) = self && let Row::Array(_) = other {
            Row::Array(vec![self.clone()]).cmp(other)
        } else {
            self.cmp(&Row::Array(vec![other.clone()]))
        }
    }
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<Row> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(Row::from_str)
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(rows: &[Row]) -> usize {
    rows.iter()
        .batching(|it| it.next_tuple::<(&Row, &Row)>())
        .enumerate()
        .filter_map(|(i, (left, right))| (left < right).then_some(i + 1))
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(rows: &[Row]) -> usize {
    let first = Row::Array(vec![Row::Array(vec![Row::Num(2)])]);
    let second = Row::Array(vec![Row::Array(vec![Row::Num(6)])]);

    rows.iter()
        .chain(vec![&first, &second].into_iter())
        .sorted()
        .enumerate()
        .filter(|(_, row)| **row == first || **row == second)
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day13::{input_generator, part1, part2};

    #[test]
    fn test_day13() {
        let input = input_generator(&read_to_string("input/2022/day13.txt").unwrap());

        assert_eq!(6101, part1(&input));
        assert_eq!(21909, part2(&input));
    }
}
