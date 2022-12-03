use std::fs::read_to_string;

use itertools::Itertools;

const ALPHALEN: usize = 122 + 1;

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32 - 64) + 26
    } else {
        c as u32 - 96
    }
}

fn sum_chars(chars: std::str::Chars<'_>) -> Vec<u64> {
    let mut contains: Vec<u64> = vec![0; ALPHALEN];
    for c in chars {
        contains[c as usize] += 1;
    }

    contains
}

fn find_common_alpha(arrays: Vec<Vec<u64>>) -> Option<char> {
    for i in 0..ALPHALEN {
        match arrays.iter().find(|array| array[i] == 0) {
            Some(_) => continue,
            None => return char::from_u32(i as u32),
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

            find_common_alpha(vec![sum_chars(first.chars()), sum_chars(second.chars())])
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
                sum_chars(it.next()?.chars()),
                sum_chars(it.next()?.chars()),
                sum_chars(it.next()?.chars()),
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
