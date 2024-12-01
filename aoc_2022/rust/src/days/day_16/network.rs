use std::collections::{HashMap, HashSet};

use super::{
    parse::parse_input,
    valve::{Name, Valve},
};

struct Network {
    valves: HashMap<Name, Valve>,
    distance_matrix: HashMap<(Name, Name), u16>,
}

impl From<Vec<Valve>> for Network {
    fn from(value: Vec<Valve>) -> Self {
        Self::new(
            value.into_iter().map(|v| (v.name, v)).collect(),
            Option::None,
        )
    }
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        parse_input(value).into()
    }
}

impl Network {
    fn new(
        valves: HashMap<Name, Valve>,
        distance_matrix: Option<HashMap<(Name, Name), u16>>,
    ) -> Self {
        let distance_matrix = match distance_matrix {
            Some(distance_matrix) => distance_matrix,
            None => calculate_distance_matrix(&valves),
        };
        Self {
            valves,
            distance_matrix,
        }
    }

    fn solve_part_1(self: Self) -> u16 {
        let mut closed_valves = self
            .valves
            .iter()
            .filter(|(_, valve)| valve.flow_rate != 0)
            .map(|(name, _)| name.clone())
            .collect::<HashSet<Name>>();
        self.recursive_solve_part_1("AA".into(), 30, &mut closed_valves)
    }

    fn recursive_solve_part_1(
        self: &Self,
        cur_location: Name,
        minutes_left: u16,
        closed_valves: &mut HashSet<Name>,
    ) -> u16 {
        let mut max_flow_rate = 0;
        for valve in closed_valves.clone() {
            let distance = *self.distance_matrix.get(&(cur_location, valve)).unwrap();
            if (distance + 1) > minutes_left {
                continue;
            }
            closed_valves.remove(&valve);

            let valve_flow_rate =
                self.valves.get(&valve).unwrap().flow_rate * (minutes_left - distance);

            let rest_flow_rate =
                self.recursive_solve_part_1(valve, minutes_left - (distance + 1), closed_valves);

            let flow_rate = valve_flow_rate + rest_flow_rate;

            if flow_rate > max_flow_rate {
                max_flow_rate = flow_rate
            }

            closed_valves.insert(valve);
        }
        max_flow_rate
    }
}

fn calculate_distance_matrix<'a>(valves: &HashMap<Name, Valve>) -> HashMap<(Name, Name), u16> {
    let non_broken_valves: Vec<Valve> = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate != 0)
        .map(|(_, valve)| valve.clone())
        .collect();
    let mut distances = HashMap::with_capacity(non_broken_valves.len() ^ 2);
    for src in &non_broken_valves {
        for target in &non_broken_valves {
            distances.insert(
                (src.name, target.name),
                find_shortest_path(src.name, target.name, &valves) + 1,
            );
        }
    }
    // Also calculate it for the start valve
    for target in &non_broken_valves {
        distances.insert(
            ("AA".into(), target.name),
            find_shortest_path("AA".into(), target.name, &valves),
        );
    }

    distances
}

fn find_shortest_path(src: Name, target: Name, graph: &HashMap<Name, Valve>) -> u16 {
    let mut length = 0;
    let mut reachable = HashSet::from([src]);
    while !reachable.contains(&target) {
        reachable = reachable
            .iter()
            .flat_map(|valve| graph.get(valve).unwrap().connects_to())
            .map(|s| *s)
            .collect();
        length += 1;
    }
    return length;
}

#[cfg(test)]
mod tests {
    use crate::days::read_day_input;

    use super::*;

    use rstest::*;

    #[fixture]
    fn example_input_str() -> &'static str {
        Box::leak(read_day_input("test_day_16").into_boxed_str())
    }

    #[fixture]
    fn example_graph(example_input_str: &'static str) -> Network {
        parse_input(example_input_str).into()
    }

    #[rstest]
    fn test_example_part_1(example_graph: Network) {
        assert_eq!(example_graph.solve_part_1(), 1651)
    }

    #[fixture]
    fn input_str() -> &'static str {
        Box::leak(read_day_input("day_16").into_boxed_str())
    }

    #[fixture]
    fn graph(input_str: &'static str) -> Network {
        parse_input(input_str).into()
    }

    #[rstest]
    fn test_part_1(graph: Network) {
        assert_eq!(graph.solve_part_1(), 1789)
    }
}
