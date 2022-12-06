use std::fs::read_to_string;

use itertools::Itertools;

fn calculate_elf_calories(elf: &str) -> u64 {
    elf.lines()
        .map(|calories| calories.parse::<u64>().unwrap())
        .sum()
}

pub fn part1() -> u64 {
    read_to_string("inputs/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(calculate_elf_calories)
        .max()
        .unwrap()
}

pub fn part2() -> u64 {
    read_to_string("inputs/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(calculate_elf_calories)
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
