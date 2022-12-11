use itertools::Itertools;

fn calculate_elf_calories(elf: &str) -> u64 {
    elf.lines()
        .map(|calories| calories.parse::<u64>().unwrap())
        .sum()
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u64> {
    input.split("\n\n").map(calculate_elf_calories).collect()
}

#[aoc(day1, part1)]
pub fn part1(calories: &[u64]) -> u64 {
    *calories.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(calories: &[u64]) -> u64 {
    calories.iter().sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day1::{input_generator, part1, part2};

    #[test]
    fn test_day1() {
        let input = input_generator(&read_to_string("input/2022/day1.txt").unwrap());

        assert_eq!(67658, part1(&input));
        assert_eq!(200158, part2(&input));
    }
}
