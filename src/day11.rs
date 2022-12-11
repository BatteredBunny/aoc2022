use itertools::Itertools;

struct Monkey {
    inspected_items: i64,
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,

    divisible_by: i64,
    test: Box<dyn Fn(i64) -> u16>,
}

fn sign_to_func(s: &str) -> fn(i64, i64) -> Option<i64> {
    match s {
        "*" => i64::checked_mul,
        "+" => i64::checked_add,
        "-" => i64::checked_sub,
        "/" => i64::checked_div,
        _ => unreachable!(),
    }
}

impl Monkey {
    fn from_lines(lines: [&str; 6]) -> Monkey {
        let items: Vec<i64> = lines[1]
            .trim_start_matches("  Starting items: ")
            .split(", ")
            .map(|num| num.parse().unwrap())
            .collect();

        let operation_arguments: Vec<&str> = lines[2]
            .trim_start_matches("  Operation: new = ")
            .split_whitespace()
            .collect();

        let sign = sign_to_func(operation_arguments[1]);
        let operation: Box<dyn Fn(i64) -> i64> =
            if operation_arguments[0] == "old" && operation_arguments[2] == "old" {
                Box::new(move |x: i64| {
                    // println!("{x}");
                    sign(x, x).unwrap()
                })
            } else {
                let arg = operation_arguments[2].parse().unwrap();
                Box::new(move |x: i64| {
                    // println!("{x} {arg}");
                    sign(x, arg).unwrap()
                })
            };

        let divisible_by = lines[3]
            .trim_start_matches("  Test: divisible by ")
            .parse::<i64>()
            .unwrap();

        let true_monkey = lines[4]
            .trim_start_matches("    If true: throw to monkey ")
            .parse::<u16>()
            .unwrap();

        let false_monkey = lines[5]
            .trim_start_matches("    If false: throw to monkey ")
            .parse::<u16>()
            .unwrap();

        let test: Box<dyn Fn(i64) -> u16> = Box::new(move |x: i64| {
            if x % divisible_by == 0 {
                true_monkey
            } else {
                false_monkey
            }
        });

        Monkey {
            inspected_items: 0,
            items,
            operation,
            divisible_by,
            test,
        }
    }
}

fn logic(monkeys: &mut Vec<Monkey>, rounds: i32, reducer: Box<dyn Fn(i64) -> i64>) -> i64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();

            for item in items.iter() {
                let worry_level = reducer((*monkeys[i].operation)(*item));

                let monkey_id = (*monkeys[i].test)(worry_level);
                monkeys[monkey_id as usize].items.push(worry_level)
            }

            monkeys[i].inspected_items += items.len() as i64;
            monkeys[i].items = Vec::new();
        }
    }

    monkeys
        .iter()
        .map(|monke| monke.inspected_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .batching(|it| {
            let lines = it.next_chunk::<6>().ok();
            it.next();
            lines
        })
        .map(Monkey::from_lines)
        .collect();

    logic(&mut monkeys, 20, Box::new(|x| x / 3))
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .batching(|it| {
            let lines = it.next_chunk::<6>().ok();
            it.next();
            lines
        })
        .map(Monkey::from_lines)
        .collect();

    let divider: i64 = monkeys.iter().map(|monkey| monkey.divisible_by).product();
    logic(&mut monkeys, 10000, Box::new(move |x| x % divider))
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day11::{part1, part2};

    #[test]
    fn test_day11() {
        let input = &read_to_string("input/2022/day11.txt").unwrap();

        assert_eq!(61503, part1(input));
        assert_eq!(14081365540, part2(input));
    }
}
