use std::{fs::read_to_string, str::Chars};

use itertools::Itertools;

const ALPHALEN: usize = 122 + 1;

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32 - 64) + 26
    } else {
        c as u32 - 96
    }
}

trait AlphaSum {
    fn sum_alpha(self) -> SummedAlpha;
}

type SummedAlpha = Vec<u64>;

impl AlphaSum for Chars<'_> {
    fn sum_alpha(self) -> SummedAlpha {
        let mut contains = vec![0; ALPHALEN];
        for c in self {
            contains[c as usize] += 1;
        }

        contains
    }
}

fn find_common_alpha(arrays: Vec<SummedAlpha>) -> Option<char> {
    for i in 0..ALPHALEN {
        if !arrays.iter().any(|s| s[i] == 0) {
            return char::from_u32(i as u32);
        }
    }

    None
}

pub fn part1() -> u32 {
    read_to_string("inputs/day3.txt")
        .unwrap()
        .lines()
        .filter_map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            find_common_alpha(vec![first.chars().sum_alpha(), second.chars().sum_alpha()])
        })
        .map(get_priority)
        .sum()
}

pub fn part2() -> u32 {
    read_to_string("inputs/day3.txt")
        .unwrap()
        .lines()
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
    use crate::day3::{part1, part2};

    #[test]
    fn test_day3() {
        assert_eq!(7845, part1());
        assert_eq!(2790, part2());
    }
}
