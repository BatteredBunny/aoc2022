use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

pub struct Pair<T>(RangeInclusive<T>, RangeInclusive<T>);

impl Pair<u32> {
    fn contains(&self) -> bool {
        self.0.contains_range(&self.1) || self.1.contains_range(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

impl FromStr for Pair<u32> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: (RangeInclusive<u32>, RangeInclusive<u32>) = s
            .split(',')
            .map(|r| {
                let (start, end) = r
                    .split('-')
                    .flat_map(u32::from_str)
                    .collect_tuple()
                    .unwrap();

                start..=end
            })
            .collect_tuple()
            .unwrap();

        Ok(Pair(ranges.0, ranges.1))
    }
}

trait RangeUtils<T> {
    fn contains_range(&self, other: &RangeInclusive<T>) -> bool;
    fn overlaps(&self, other: &RangeInclusive<T>) -> bool;
}

impl RangeUtils<u32> for RangeInclusive<u32> {
    fn contains_range(&self, other: &RangeInclusive<u32>) -> bool {
        self.end() >= other.end() && self.start() <= other.start()
    }

    fn overlaps(&self, other: &RangeInclusive<u32>) -> bool {
        other.contains(self.end()) || other.contains(self.start())
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Pair<u32>> {
    input.lines().flat_map(Pair::from_str).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Pair<u32>]) -> usize {
    input
        .iter()
        .map(Pair::contains)
        .filter(|contains| *contains)
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Pair<u32>]) -> usize {
    input
        .iter()
        .map(Pair::overlaps)
        .filter(|contains| *contains)
        .count()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day4::{input_generator, part1, part2};

    #[test]
    fn test_day4() {
        let input = input_generator(&read_to_string("input/2022/day4.txt").unwrap());

        assert_eq!(651, part1(&input));
        assert_eq!(956, part2(&input));
    }
}
