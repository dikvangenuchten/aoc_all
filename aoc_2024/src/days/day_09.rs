use std::{collections::VecDeque, fmt::Display, str::FromStr};

pub fn solve_day(input_file: &str) -> (usize, usize) {
    let a = part_a(input_file);
    let b = part_b(input_file);
    (a, b)
}

fn part_a(input_file: &str) -> usize {
    let fs = FileSystem::from_str(input_file.trim()).unwrap();
    fs.checksum_a()
}

fn part_b(input_file: &str) -> usize {
    let fs = FileSystem::from_str(input_file.trim()).unwrap();
    fs.checksum_b()
}

struct FileSystem {
    files: Vec<File>,
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file in &self.files {
            f.write_str(&file.to_string())?;
        }
        Ok(())
    }
}

impl FileSystem {
    fn checksum_a(&self) -> usize {
        let mut files: VecDeque<File> = self.files.iter().copied().collect();
        let mut checksum = 0;
        let mut offset = 0;
        while let Some(file) = files.pop_front() {
            if file.is_empty {
                let mut rem_space = file.length;
                while rem_space > 0 {
                    if let Some(last_file) = files.back_mut() {
                        let n = last_file.pop_n(rem_space);
                        let diff = calculate_checksum(last_file.id, offset, offset + n);
                        checksum += diff;
                        offset += n;
                        rem_space -= n;
                        if last_file.length == 0 {
                            files.pop_back();
                        }
                    } else {
                        break;
                    }
                }
            } else {
                checksum += file.checksum(offset);
                offset += file.length;
            }
        }
        checksum
    }

    fn checksum_b(&self) -> usize {
        let final_files = self.reorder_b();
        let mut checksum = 0;
        for file in final_files {
            checksum += file.checksum(file.start_pos);
        }
        checksum
    }

    fn reorder_b(&self) -> Vec<File> {
        let files: Vec<&File> = self.files.iter().collect();
        let mut final_files: Vec<File> = self
            .files
            .iter()
            .filter(|f| f.length > 0)
            .copied()
            .collect();
        let mut first_empty = 0;

        for file in files.iter().rev() {
            if file.is_empty {
                continue;
            }

            let idx = final_files
                .binary_search(file)
                .expect("File should be present");

            if first_empty > idx {
                // No empty spots before this
                // Thus no need to check before
                continue;
            }

            if let Some(insert_pos) = final_files[first_empty..idx]
                .iter()
                .position(|f| f.is_empty && f.length >= file.length)
            {
                let insert_pos = first_empty + insert_pos;

                // Replace the old with a empty one
                let new_empty = File {
                    start_pos: final_files[idx].start_pos,
                    length: final_files[idx].length,
                    id: final_files[idx].id,
                    is_empty: true,
                };
                // Push to end then use swap_remove which inserts last item
                final_files.push(new_empty);
                let mut to_move = final_files.swap_remove(idx);

                // Update start pos
                assert_eq!(file, &&to_move);
                to_move.start_pos = final_files[insert_pos].start_pos;
                if to_move.length == final_files[insert_pos].length {
                    final_files.push(to_move);
                    final_files.swap_remove(insert_pos);
                } else {
                    final_files.insert(insert_pos, to_move);
                    final_files[insert_pos + 1].start_pos += to_move.length;
                    final_files[insert_pos + 1].length -= to_move.length;
                }

                first_empty += update_first_empty(&final_files, first_empty)
            }
        }
        final_files
    }
}

fn update_first_empty(final_files: &Vec<File>, first_empty: usize) -> usize {
    final_files[first_empty..]
        .iter()
        .position(|f| f.is_empty && f.length > 0)
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct File {
    start_pos: usize,
    length: usize,
    id: usize,
    is_empty: bool,
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start_pos.cmp(&other.start_pos) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.id.cmp(&other.id),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl File {
    fn pop_n(&mut self, n: usize) -> usize {
        if self.is_empty {
            self.length = 0;
            return 0;
        }
        let n = self.length.min(n);
        self.length -= n;
        n
    }

    fn checksum(&self, offset: usize) -> usize {
        if self.is_empty {
            0
        } else {
            calculate_checksum(self.id, offset, offset + self.length)
        }
    }
}

fn calculate_checksum(id: usize, start: usize, end: usize) -> usize {
    let mut sum = 0;
    for i in start..end {
        sum += i * id;
    }
    sum
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for FileSystem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut is_empty = false;
        let mut ptr = 0;
        let mut id = 0;
        Ok(Self {
            files: s
                .chars()
                .map(|char| {
                    let length = char.to_digit(10).unwrap() as usize;
                    let file = File {
                        start_pos: ptr,
                        length,
                        id,
                        is_empty,
                    };
                    // update ptr, is_empty
                    ptr += length;
                    id += is_empty as usize;
                    is_empty = !is_empty;
                    file
                })
                .collect(),
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty {
            f.write_str(&".".repeat(self.length))
        } else {
            f.write_str(&self.id.to_string().repeat(self.length))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_day_input;

    use super::*;
    use rstest::rstest;

    // Debug visualizationcode
    fn decode(dense: &str) -> String {
        let fs = FileSystem::from_str(dense).unwrap();
        fs.to_string()
    }

    #[rstest]
    #[case("12345", "0..111....22222")]
    #[case("13222", "0...11..22")]
    #[case("2333133121414131402", "00...111...2...333.44.5555.6666.777.888899")]
    fn test_decode(#[case] encoded: &str, #[case] decoded: &str) {
        assert_eq!(decode(encoded), decoded)
    }

    #[rstest]
    #[case("12345", 60)]
    #[case("13222", 13)]
    #[case("2333133121414131402", 1928)]
    fn test_checksum_a(#[case] fs: FileSystem, #[case] checksum: usize) {
        assert_eq!(fs.checksum_a(), checksum)
    }

    #[rstest]
    #[case("2333133121414131402", 2858)]
    fn test_checksum_b(#[case] fs: FileSystem, #[case] checksum: usize) {
        assert_eq!(fs.checksum_b(), checksum)
    }

    #[rstest]
    fn test_part_a() {
        assert_eq!(part_a("2333133121414131402"), 1928)
    }

    #[rstest]
    fn test_actual_a() {
        let input = read_day_input("09");
        let result = part_a(&input);
        assert_eq!(result, 6432869891895)
    }

    #[rstest]
    fn test_part_b() {
        assert_eq!(part_b("2333133121414131402"), 2858)
    }

    #[rstest]
    fn test_actual_b() {
        let input = read_day_input("09");
        let result = part_b(&input);
        assert_eq!(result, 6467290479134)
    }
}
