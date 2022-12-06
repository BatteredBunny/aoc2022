use std::fs::read_to_string;

const ALPHALEN: usize = 123;

fn logic(n: usize) -> usize {
    let code = read_to_string("inputs/day6.txt").unwrap();

    for skip in 0.. {
        let mut contains = vec![0; ALPHALEN];

        for (counter, (absolute_position, char)) in code.chars().enumerate().skip(skip).enumerate()
        {
            if counter == n {
                match contains.iter().find(|c| **c > 1) {
                    Some(_) => break,
                    None => return absolute_position,
                }
            }

            contains[char as usize] += 1;
        }
    }

    0
}

pub fn part1() -> usize {
    logic(4)
}

pub fn part2() -> usize {
    logic(14)
}

#[cfg(test)]
mod tests {
    use crate::day6::{part1, part2};

    #[test]
    fn test_day1() {
        assert_eq!(1640, part1());
        assert_eq!(3613, part2());
    }
}
