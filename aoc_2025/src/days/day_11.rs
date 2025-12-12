use std::{collections::HashMap, hash::Hash};

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let nodes = parse(input_file);
    let a = part_a(&nodes);
    let b = part_b(&nodes);
    (a, b)
}

fn part_a(nodes: &HashMap<[char; 3], Vec<[char; 3]>>) -> u64 {
    let mut memo = HashMap::new();
    count_paths_recursive(nodes, ['y', 'o', 'u'], ['o', 'u', 't'], &mut memo)
}

fn count_paths_recursive(
    nodes: &HashMap<[char; 3], Vec<[char; 3]>>,
    current: [char; 3],
    end: [char; 3],
    memo: &mut HashMap<[char; 3], u64>,
) -> u64 {
    if let Some(&cached) = memo.get(&current) {
        return cached;
    }
    if current == end {
        return 1;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = nodes.get(&current) {
        for neighbor in neighbors {
            total_paths += count_paths_recursive(nodes, *neighbor, end, memo);
        }
    }
    memo.insert(current, total_paths);
    total_paths
}

fn part_b(nodes: &HashMap<[char; 3], Vec<[char; 3]>>) -> u64 {
    let start = ['s', 'v', 'r'];
    let end = ['o', 'u', 't'];
    let dac = ['d', 'a', 'c'];
    let fft = ['f', 'f', 't'];

    let mut memo: HashMap<[char; 3], u64> = HashMap::new();

    let from_fft_to_out = count_paths_recursive(nodes, fft, end, &mut memo);
    let from_dac_to_out = count_paths_recursive(nodes, dac, end, &mut memo);

    let mut memo: HashMap<[char; 3], u64> = HashMap::new();
    let to_dac = count_paths_recursive(nodes, start, dac, &mut memo);
    let from_fft_to_dac = count_paths_recursive(nodes, fft, dac, &mut memo);

    let mut memo: HashMap<[char; 3], u64> = HashMap::new();
    let to_fft = count_paths_recursive(nodes, start, fft, &mut memo);
    let from_dac_to_fft = count_paths_recursive(nodes, dac, fft, &mut memo);

    to_dac * from_dac_to_fft * from_fft_to_out + to_fft * from_fft_to_dac * from_dac_to_out
}

fn parse(input_file: &str) -> HashMap<[char; 3], Vec<[char; 3]>> {
    let mut server_racks: HashMap<[char; 3], Vec<[char; 3]>> = HashMap::new();
    for line in input_file.trim().lines() {
        let (name, connections_str) = line.split_once(": ").expect("Could not split line: {line}");
        let server: Vec<char> = name.chars().collect();
        assert_eq!(server.len(), 3, "Server key must be 3 characters long");
        let server_key: [char; 3] = [server[0], server[1], server[2]];

        let mut connections: Vec<[char; 3]> = Vec::new();

        for conn in connections_str.split(' ').map(|s| s.trim()) {
            let conn_chars: Vec<char> = conn.chars().collect();
            let conn_key: [char; 3] = [conn_chars[0], conn_chars[1], conn_chars[2]];
            connections.push(conn_key);
        }

        server_racks.insert(server_key, connections);
    }
    server_racks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read_test_day_input;
    use rstest::*;

    #[rstest]
    fn test_parse() {
        let parsed = parse(&read_test_day_input("11"));
        let mut expected = HashMap::new();
        expected.insert(['a', 'a', 'a'], vec![['y', 'o', 'u'], ['h', 'h', 'h']]);
        expected.insert(['y', 'o', 'u'], vec![['b', 'b', 'b'], ['c', 'c', 'c']]);
        expected.insert(['b', 'b', 'b'], vec![['d', 'd', 'd'], ['e', 'e', 'e']]);
        expected.insert(
            ['c', 'c', 'c'],
            vec![['d', 'd', 'd'], ['e', 'e', 'e'], ['f', 'f', 'f']],
        );
        expected.insert(['d', 'd', 'd'], vec![['g', 'g', 'g']]);
        expected.insert(['e', 'e', 'e'], vec![['o', 'u', 't']]);
        expected.insert(['f', 'f', 'f'], vec![['o', 'u', 't']]);
        expected.insert(['g', 'g', 'g'], vec![['o', 'u', 't']]);
        expected.insert(
            ['h', 'h', 'h'],
            vec![['c', 'c', 'c'], ['f', 'f', 'f'], ['i', 'i', 'i']],
        );
        expected.insert(['i', 'i', 'i'], vec![['o', 'u', 't']]);

        assert_eq!(parsed, expected);
    }

    #[rstest]
    fn test_part_b() {
        let input = read_test_day_input("11b");
        let nodes = parse(&input);
        let result = part_b(&nodes);
        assert_eq!(result, 2);
    }
}
