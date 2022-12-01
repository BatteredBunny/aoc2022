use std::fs::read_to_string;

use itertools::Itertools;

pub fn part1() -> u64 {
    read_to_string("inputs/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part2() -> u64 {
    read_to_string("inputs/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn test_day1() {
        assert_eq!(67658, part1());
        assert_eq!(200158, part2());
    }
}
