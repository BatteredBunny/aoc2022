use pathfinding::prelude::astar;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tile {
    Start,
    End,
    Tile(u32),
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            'S' => Tile::Start,
            'E' => Tile::End,
            'a'..='z' => Tile::Tile(c as u32 - 97),
            _ => unreachable!(),
        }
    }

    fn to_char(self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Tile(i) => char::from_u32(i + 97).unwrap(),
        }
    }

    fn get_height(&self) -> u32 {
        match self {
            Tile::Start => 0,
            Tile::End => 25,
            Tile::Tile(i) => *i,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: u32,
    y: u32,
    tile: Tile,
}

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn move_choices(&self, map: &Map) -> Vec<(Pos, u32)> {
        let mut moves: Vec<Pos> = Vec::new();

        if self.y + 1 < map.0.len() as u32 && self.can_move_to(map.get(self.y + 1, self.x)) {
            moves.push(map.get(self.y + 1, self.x).clone())
        }

        if self.x + 1 < map.0[self.y as usize].len() as u32
            && self.can_move_to(map.get(self.y, self.x + 1))
        {
            moves.push(map.get(self.y, self.x + 1).clone())
        }

        if self.y > 0 && self.can_move_to(map.get(self.y - 1, self.x)) {
            moves.push(map.get(self.y - 1, self.x).clone())
        }

        if self.x > 0 && self.can_move_to(map.get(self.y, self.x - 1)) {
            moves.push(map.get(self.y, self.x - 1).clone())
        }

        moves.into_iter().map(|p| (p, 1)).collect()
    }

    fn can_move_to(&self, other: &Pos) -> bool {
        self.tile.get_height() >= other.tile.get_height()
            || (other.tile.get_height() > 0
                && self.tile.get_height() == other.tile.get_height() - 1)
    }
}

#[derive(Debug)]
pub struct Input {
    map: Map,
    start: Pos,
    end: Pos,
}

#[derive(Debug)]
struct Map(Vec<Vec<Pos>>);

impl Map {
    fn get(&self, y: u32, x: u32) -> &Pos {
        &self.0[y as usize][x as usize]
    }

    fn print(&self, current: &Pos, highlight: &[Pos]) {
        for row in &self.0 {
            for pos in row {
                if pos.x == current.x && pos.y == current.y {
                    print!("\x1b[93mX\x1b[0m")
                } else if highlight
                    .iter()
                    .any(|high| pos.x == high.x && pos.y == high.y)
                {
                    print!("\x1b[94m{}\x1b[0m", pos.tile.to_char());
                } else {
                    print!("{}", pos.tile.to_char());
                }
            }

            println!()
        }

        println!()
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Input {
    let mut start = Pos {
        x: 0,
        y: 0,
        tile: Tile::Start,
    };

    let mut end = Pos {
        x: 0,
        y: 0,
        tile: Tile::End,
    };

    let map = Map(input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pos = Pos {
                        x: x as u32,
                        y: y as u32,
                        tile: Tile::from_char(c),
                    };
                    match pos.tile {
                        Tile::End => {
                            end.x = pos.x;
                            end.y = pos.y;
                        }
                        Tile::Start => {
                            start.x = pos.x;
                            start.y = pos.y;
                        }
                        _ => {}
                    }

                    pos
                })
                .collect()
        })
        .collect());

    Input { map, start, end }
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> u32 {
    astar(
        &input.start,
        |p| p.move_choices(&input.map),
        |p| p.distance(&input.end),
        |p| p.tile == Tile::End,
    )
    .expect("Cant find a path")
    .1
}

#[aoc(day12, part2)]
pub fn part2(input: &Input) -> u32 {
    input
        .map
        .0
        .iter()
        .flatten()
        .filter(|pos| pos.tile.get_height() == 0)
        .filter_map(|pos| {
            Some(
                astar(
                    pos,
                    |p| p.move_choices(&input.map),
                    |p| p.distance(&input.end),
                    |p| p.tile == Tile::End,
                )?
                .1,
            )
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day12::{input_generator, part1, part2};

    #[test]
    fn test_day12() {
        let input = input_generator(&read_to_string("input/2022/day12.txt").unwrap());

        assert_eq!(352, part1(&input));
        assert_eq!(345, part2(&input));
    }
}
