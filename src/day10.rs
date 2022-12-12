use std::str::FromStr;

pub enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else if s.starts_with("addx ") {
            Ok(Self::Addx(s.trim_start_matches("addx ").parse().unwrap()))
        } else {
            Err(String::from("Invalid"))
        }
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(Instruction::from_str).collect()
}

fn execute_cycle(
    cycle_amount: &i64,
    x: &i64,
    executed_already: &mut bool,
    signal_strenght: &mut i64,
) {
    if !*executed_already
        && (*cycle_amount == 20 || ((*cycle_amount as f64 - 20.0) / 40.0).fract() == 0.0)
    {
        let total = *cycle_amount * x;
        *signal_strenght += total;
        *executed_already = true;
    }
}

#[aoc(day10, part1)]
pub fn part1(instructions: &[Instruction]) -> i64 {
    let mut signal_strenght = 0;
    let mut x: i64 = 1;

    let mut add_next_cycle: i64 = 0;
    let mut cycle_amount = 0;
    for instruction in instructions {
        let mut executed_already = false;
        execute_cycle(
            &cycle_amount,
            &x,
            &mut executed_already,
            &mut signal_strenght,
        );

        x += add_next_cycle;
        add_next_cycle = 0;

        if let Instruction::Addx(i) = instruction {
            add_next_cycle = *i;
            cycle_amount += 1;
        }

        execute_cycle(
            &cycle_amount,
            &x,
            &mut executed_already,
            &mut signal_strenght,
        );

        cycle_amount += 1;
    }

    signal_strenght
}

// fn sprite_pos_preview(pos: &i64) -> String {
//     (1..=40)
//         .map(|i| {
//             if i == *pos || i == pos - 1 || i == pos + 1 {
//                 '#'
//             } else {
//                 '.'
//             }
//         })
//         .collect()
// }

// fn display_row(row: &[bool]) -> String {
//     row.iter()
//         .map(|pixel| if *pixel { '#' } else { '.' })
//         .collect()
// }

// fn execute_cycle2(crt_cycle: &mut i64, x: &i64, crt: &mut [Vec<bool>]) {
//     let row = match crt_cycle {
//         0..=40 => 0,
//         41..=80 => 1,
//         81..=120 => 2,
//         121..=160 => 3,
//         161..=200 => 4,
//         201..=240 => 5,
//         _ => 6,
//     };

//     if row == 0 {
//         let pos = *crt_cycle - 1;

//         if pos == x - 1 || pos == *x || pos == x + 1 {
//             crt[row][pos as usize] = true
//         }

//         println!(
//             "During cycle   {}: CRT draws pixel in position {}",
//             crt_cycle, pos
//         );

//         println!("Current CRT row: {}", display_row(&crt[row]));
//     }

//     *crt_cycle += 1;
// }

// #[aoc(day10, part2)]
// pub fn part2(instructions: &[Instruction]) -> u64 {
//     let mut crt = vec![vec![false; 39]; 6];

//     let mut x: i64 = 1;
//     let mut add_next_cycle: i64 = 0;

//     let mut crt_cycle = 1;
//     let mut cycle_amount = 0;
//     for instruction in instructions {
//         println!("Sprite position: {}", sprite_pos_preview(&x));
//         println!();

//         execute_cycle2(&mut crt_cycle, &x, &mut crt);

//         x += add_next_cycle;
//         add_next_cycle = 0;

//         if let Instruction::Addx(i) = instruction {
//             println!("Start cycle   {}: being executing addx {}", crt_cycle, i);
//             add_next_cycle = *i;
//             cycle_amount += 1;
//         }

//         // execute_cycle2(&mut crt_cycle, &x, &mut crt);

//         cycle_amount += 1;
//     }

//     for row in crt {
//         println!("{}", display_row(&row));
//     }
//     println!();

//     todo!()
// }

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day10::{input_generator, part1};

    #[test]
    fn test_day10() {
        let input = input_generator(&read_to_string("input/2022/day10.txt").unwrap());
        assert_eq!(14340, part1(&input));
    }
}
