use std::{fs::read_to_string, ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

struct Pair<T>(RangeInclusive<T>, RangeInclusive<T>);

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
                    .map(|s| s.parse().unwrap())
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

fn logic(f: &dyn Fn(&Pair<u32>)  -> bool) -> usize {
    read_to_string("inputs/day4.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Pair<u32>>().unwrap())
        .map(|pair| f(&pair))
        .filter(|contains| *contains)
        .count()
}

pub fn part1() -> usize {
    logic(&Pair::contains)
}

pub fn part2() -> usize {
    logic(&Pair::overlaps)
}

#[cfg(test)]
mod tests {
    use crate::day4::{part1, part2};

    #[test]
    fn test_day4() {
        assert_eq!(651, part1());
        assert_eq!(956, part2());
    }
}
