use regex::Regex;
use std::{
    borrow::{Borrow, BorrowMut},
    str::FromStr,
};

use lazy_static::lazy_static;

pub fn solve(input_str: &str) -> (u32, u32) {
    let fs = parse_input(input_str);
    let part_1 = solve_part_1(&fs);
    let part_2 = solve_part_2(&fs);
    (part_1, part_2)
}

fn solve_part_1(fs: &FS) -> u32 {
    fs.calculate_total_size_of_directories_of_at_most_100000()
}

fn solve_part_2(fs: &FS) -> u32 {
    let total_size = 70000000;
    let space_free = total_size - fs.get_size();
    let min_size = 30000000 - space_free;
    fs.find_smallest_directory_larger_then(min_size)
}

#[derive(Debug, PartialEq, Eq)]
enum FS {
    File(String, u32),
    Dir(String, Vec<FS>, u32),
}

impl FS {
    fn add_item(&mut self, path: &str, item: FS) {
        let item_size = match item {
            FS::File(_, size) => size,
            FS::Dir(_, _, size) => size,
        };

        if let FS::Dir(name, sub_items, size) = self {
            *size += item_size;
            if name == path {
                sub_items.push(item);
                return;
            }
            let path = path
                .strip_prefix(name.as_str())
                .unwrap()
                .strip_prefix('/')
                .unwrap();
            for sub_item in sub_items {
                match sub_item.borrow_mut() {
                    FS::Dir(name, _, _) if path.starts_with(name.as_str()) => {
                        sub_item.add_item(path, item);
                        break;
                    }
                    _ => continue,
                };
            }
        } else {
            unreachable!()
        }
    }

    fn calculate_total_size_of_directories_of_at_most_100000(&self) -> u32 {
        match self {
            FS::Dir(_, sub_items, size) => {
                let mut sum = 0;
                if size < &100000 {
                    sum += size;
                }
                for item in sub_items {
                    sum += item.calculate_total_size_of_directories_of_at_most_100000()
                }
                sum
            }
            FS::File(_, _) => 0,
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            FS::File(_, size) => *size,
            FS::Dir(_, _, size) => *size,
        }
    }

    fn find_smallest_directory_larger_then(&self, min_size: u32) -> u32 {
        self._find_smallest_directory_larger_then(min_size, self.get_size())
    }

    fn _find_smallest_directory_larger_then(&self, min_size: u32, mut current_best: u32) -> u32 {
        if let FS::Dir(_, sub_items, _) = self {
            for sub_item in sub_items {
                if let FS::File(_, _) = sub_item.borrow() {
                    continue;
                }
                if sub_item.get_size() > min_size && sub_item.get_size() < current_best {
                    current_best = sub_item.get_size()
                }
                current_best = sub_item._find_smallest_directory_larger_then(min_size, current_best)
            }
        }
        current_best
    }
}

impl FromStr for FS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"dir (?P<name>.*)").unwrap();
            }
            let name = RE.captures(s).unwrap().name("name").unwrap().as_str();
            return Ok(FS::Dir(name.to_string(), vec![], 0));
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<size>[0-9]*) (?P<name>.*)").unwrap();
        }
        let capture = RE.captures(s).unwrap();
        return Ok(FS::File(
            capture.name("name").unwrap().as_str().to_string(),
            capture.name("size").unwrap().as_str().parse().unwrap(),
        ));
    }
}

fn parse_input(input_str: &str) -> FS {
    let mut cur_dir = Vec::new();
    let mut ls_active = false;
    lazy_static! {
        static ref CD_REGEX: Regex = Regex::new(r"^\$ cd ([^\n]*)").unwrap();
    }
    let mut root = FS::Dir("/".to_string(), vec![], 0);
    for line in input_str.trim().split('\n') {
        if line.starts_with('$') {
            ls_active = false;
        }
        if ls_active {
            let path = &cur_dir.join("/");
            let item = FS::from_str(line).unwrap();

            root.add_item(path, item);
        } else if let Some(capture) = CD_REGEX.captures(line) {
            let dir_name = capture.get(1).unwrap().as_str();
            if dir_name == ".." {
                cur_dir.pop();
            } else {
                cur_dir.push(dir_name);
            }
        } else if line == "$ ls" {
            ls_active = true;
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::days::read_day_input;

    use super::*;
    use rstest::*;

    // #[rstest]
    // #[case("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k", HashMap::from([
    //     ("/a/e".to_string(), 584),
    //     ("/a".to_string(), 94853),
    //     ("/d".to_string(), 24933642),
    //     ("/".to_string(), 48381165),
    //     ]
    // ))]
    // fn test_parse_input(#[case] input_str: &str, #[case] expected_hashmap: HashMap<String, u32>) {
    //     assert_eq!(parse_input_to_map(input_str), expected_hashmap)
    // }

    #[fixture]
    fn example_input() -> &'static str {
        "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k"
    }

    #[rstest]
    fn test_parse_directory(example_input: &str) {
        assert_eq!(
            parse_input(example_input),
            FS::Dir(
                "/".to_string(),
                vec![
                    (FS::Dir(
                        "a".to_string(),
                        vec![
                            (FS::Dir("e".to_string(), vec![(FS::File("i".to_string(), 584))], 584)),
                            (FS::File("f".to_string(), 29116)),
                            (FS::File("g".to_string(), 2557)),
                            (FS::File("h.lst".to_string(), 62596)),
                        ],
                        584 + 29116 + 2557 + 62596
                    )),
                    (FS::File("b.txt".to_string(), 14848514)),
                    (FS::File("c.dat".to_string(), 8504156)),
                    (FS::Dir(
                        "d".to_string(),
                        vec![
                            (FS::File("j".to_string(), 4060174)),
                            (FS::File("d.log".to_string(), 8033020)),
                            (FS::File("d.ext".to_string(), 5626152)),
                            (FS::File("k".to_string(), 7214296)),
                        ],
                        4060174 + 8033020 + 5626152 + 7214296
                    ))
                ],
                48381165
            )
        )
    }

    #[rstest]
    fn test_example_input_p1(example_input: &str) {
        let fs = &parse_input(example_input);
        assert_eq!(solve_part_1(fs), 95437)
    }

    #[rstest]
    fn test_example_input_p2(example_input: &str) {
        let fs = &parse_input(example_input);
        assert_eq!(solve_part_2(fs), 24933642)
    }

    #[rstest]
    fn test_parse_day_input() {
        let input = read_day_input("day_07");
        parse_input(input.as_str());
    }
}
