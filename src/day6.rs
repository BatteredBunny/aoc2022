const ALPHALEN: usize = 26 * 2;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<(usize, char)> {
    input.chars().enumerate().collect()
}

fn char_to_pos(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32 - 65) + 26
    } else {
        c as u32 - 97
    }
}

fn logic(input: &[(usize, char)], n: usize) -> usize {
    for skip in 0.. {
        let mut contains = vec![0; ALPHALEN];

        for (counter, (absolute_position, char)) in input.iter().skip(skip).enumerate() {
            if counter == n {
                match contains.into_iter().find(|c| *c > 1) {
                    Some(_) => break,
                    None => return *absolute_position,
                }
            }

            contains[char_to_pos(*char) as usize] += 1;
        }
    }

    0
}

#[aoc(day6, part1)]
pub fn part1(input: &[(usize, char)]) -> usize {
    logic(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &[(usize, char)]) -> usize {
    logic(input, 14)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day6::{input_generator, part1, part2};

    #[test]
    fn test_day6() {
        let input = input_generator(&read_to_string("input/2022/day6.txt").unwrap());

        assert_eq!(1640, part1(&input));
        assert_eq!(3613, part2(&input));
    }
}
