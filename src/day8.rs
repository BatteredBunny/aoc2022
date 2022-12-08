#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn edge_check(cord: usize, max: usize) -> bool {
    cord == 0 || cord == max
}

#[aoc(day8, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, height)| {
                edge_check(y, max_y)
                    || edge_check(x, max_x)
                    || !((0..=x - 1).rev().any(|i| input[y][i] >= *height)
                        && (x + 1..=max_x).any(|i| input[y][i] >= *height)
                        && (0..=y - 1).rev().any(|i| input[i][x] >= *height)
                        && (y + 1..=max_y).any(|i| input[i][x] >= *height))
            })
        })
        .filter(|visible| *visible)
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, height)| {
                let mut count_wall = true;

                let up = (0..=y)
                    .rev()
                    .skip(1)
                    .take_while(|i| {
                        if count_wall && *height > input[*i][x] {
                            true
                        } else if count_wall {
                            count_wall = false;
                            true
                        } else {
                            false
                        }
                    })
                    .count();
                count_wall = true;

                let down = (y..=max_y)
                    .skip(1)
                    .take_while(|i| {
                        if count_wall && *height > input[*i][x] {
                            true
                        } else if count_wall {
                            count_wall = false;
                            true
                        } else {
                            false
                        }
                    })
                    .count();
                count_wall = true;

                let left = (0..=x)
                    .rev()
                    .skip(1)
                    .take_while(|i| {
                        if count_wall && *height > input[y][*i] {
                            true
                        } else if count_wall {
                            count_wall = false;
                            true
                        } else {
                            false
                        }
                    })
                    .count();
                count_wall = true;

                let right = (x..=max_x)
                    .skip(1)
                    .take_while(|i| {
                        if count_wall && *height > input[y][*i] {
                            true
                        } else if count_wall {
                            count_wall = false;
                            true
                        } else {
                            false
                        }
                    })
                    .count();

                up * left * down * right
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day8::{input_generator, part1, part2};

    #[test]
    fn test_day8() {
        let input = input_generator(&read_to_string("input/2022/day8.txt").unwrap());

        assert_eq!(1703, part1(&input));
        assert_eq!(496650, part2(&input));
    }
}
