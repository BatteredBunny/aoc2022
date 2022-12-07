use std::{cell::RefCell, iter, rc::Rc, str::FromStr};

use itertools::Itertools;

const MAX: u64 = 100_000;

const TOTAL_DISK_SPACE: u64 = 70_000_000;
const MIN_UNUSED_SPACE: u64 = 30_000_000;

type Wrapper<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub enum FS {
    File {
        name: String,
        size: u64,
        parent: Option<Wrapper<Self>>,
    },

    Directory {
        name: String,
        files: Vec<Wrapper<Self>>,
        parent: Option<Wrapper<Self>>,
    },
}

impl FS {
    fn size(&self) -> u64 {
        match self {
            FS::File { size, .. } => *size,
            FS::Directory { files, .. } => files.iter().map(|file| file.borrow().size()).sum(),
        }
    }

    fn name(&self) -> String {
        match self {
            FS::File { name, .. } | FS::Directory { name, .. } => name.clone(),
        }
    }
    fn parent(&self) -> Option<Wrapper<Self>> {
        match self {
            FS::File { parent, .. } | FS::Directory { parent, .. } => {
                parent.as_ref().map(Rc::clone)
            }
        }
    }

    fn add_file(&mut self, file: Self) {
        if let FS::Directory { files, .. } = self {
            files.push(Rc::new(RefCell::new(file)));
        }
    }
}

pub enum Command {
    Dir(String),
    File(u64, String),
    CdUp,
    CdRoot,
    Cd(String),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "$ ls" {
            Err(())
        } else if s.starts_with("dir ") {
            Ok(Self::Dir(s.trim_start_matches("dir ").to_string()))
        } else if s.starts_with("$ cd ") {
            Ok(match s.trim_start_matches("$ cd ") {
                ".." => Self::CdUp,
                "/" => Self::CdRoot,
                dir => Self::Cd(dir.to_string()),
            })
        } else {
            let (byte, name) = s.split(' ').next_tuple().unwrap();
            Ok(Self::File(byte.parse().unwrap(), name.to_string()))
        }
    }
}
#[aoc_generator(day7)]
fn input_generator(input: &str) -> Wrapper<FS> {
    let root = Rc::new(RefCell::new(FS::Directory {
        name: String::from("/"),
        files: Vec::new(),
        parent: None,
    }));

    let mut current_directory = Rc::clone(&root);

    for command in input.lines().flat_map(Command::from_str) {
        match command {
            Command::Dir(name) => {
                current_directory.borrow_mut().add_file(FS::Directory {
                    name: name.to_string(),
                    parent: Some(Rc::clone(&current_directory)),
                    files: Vec::new(),
                });
            }
            Command::File(bytes, name) => {
                current_directory.borrow_mut().add_file(FS::File {
                    name: name.to_string(),
                    parent: Some(Rc::clone(&current_directory)),
                    size: bytes,
                });
            }
            Command::CdUp => {
                let parent = current_directory.borrow().parent().unwrap();
                current_directory = parent;
            }
            Command::CdRoot => current_directory = Rc::clone(&root),
            Command::Cd(new_dir_name) => {
                let c = Rc::clone(&current_directory);

                if let FS::Directory { files, .. } = &*c.borrow() {
                    current_directory = Rc::clone(
                        files
                            .iter()
                            .find(|file| *(file.borrow()).name() == new_dir_name)
                            .unwrap(),
                    );
                };
            }
        }
    }

    root
}

#[aoc(day7, part1)]
pub fn part1(root: &Wrapper<FS>) -> u64 {
    dir_collect(root).iter().filter(|b| **b <= MAX).sum()
}

#[aoc(day7, part2)]
pub fn part2(root: &Wrapper<FS>) -> u64 {
    let unused_space = TOTAL_DISK_SPACE - root.borrow().size();

    if unused_space >= MIN_UNUSED_SPACE {
        0
    } else {
        let needed_space = MIN_UNUSED_SPACE - unused_space;

        dir_collect(root)
            .into_iter()
            .sorted()
            .find(|bytes| *bytes >= needed_space)
            .unwrap()
    }
}

fn dir_collect(dir: &Wrapper<FS>) -> Vec<u64> {
    if let FS::Directory { files, .. } = &*dir.borrow() {
        files
            .iter()
            .filter(|file| matches!(*file.borrow(), FS::Directory { .. }))
            .flat_map(|file| {
                dir_collect(file)
                    .into_iter()
                    .chain(iter::once(file.borrow().size()))
                    .collect::<Vec<u64>>()
            })
            .collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day7::{input_generator, part1, part2};

    #[test]
    fn test_day7() {
        let input = input_generator(&read_to_string("input/2022/day7.txt").unwrap());

        assert_eq!(1297683, part1(&input));
        assert_eq!(5756764, part2(&input));
    }
}
