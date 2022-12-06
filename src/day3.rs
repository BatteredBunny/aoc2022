use std::str::Chars;

use itertools::Itertools;

const ALPHALEN: usize = 26 * 2;

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32 - 64) + 26
    } else {
        c as u32 - 96
    }
}

fn pos_to_char(i: u32) -> Option<char> {
    if i >= 26 {
        char::from_u32((i - 26) + 65)
    } else {
        char::from_u32(i + 97)
    }
}

trait AlphaSum {
    fn sum_alpha(self) -> SummedAlpha;
}

type SummedAlpha = Vec<u8>;

impl AlphaSum for Chars<'_> {
    fn sum_alpha(self) -> SummedAlpha {
        let mut contains = vec![0; ALPHALEN];
        for c in self {
            contains[get_priority(c) as usize - 1] += 1;
        }

        contains
    }
}

fn find_common_alpha(arrays: Vec<SummedAlpha>) -> Option<char> {
    for i in 0..ALPHALEN {
        if !arrays.iter().any(|s| s[i] == 0) {
            return pos_to_char(i as u32);
        }
    }

    None
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_alphabetic()).collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .filter_map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            find_common_alpha(vec![first.chars().sum_alpha(), second.chars().sum_alpha()])
        })
        .map(get_priority)
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .batching(|it| {
            find_common_alpha(vec![
                it.next()?.chars().sum_alpha(),
                it.next()?.chars().sum_alpha(),
                it.next()?.chars().sum_alpha(),
            ])
        })
        .map(get_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day3::{input_generator, part1, part2};

    #[test]
    fn test_day3() {
        let input = input_generator(&read_to_string("input/2022/day3.txt").unwrap());

        assert_eq!(7845, part1(&input));
        assert_eq!(2790, part2(&input));
    }
}
