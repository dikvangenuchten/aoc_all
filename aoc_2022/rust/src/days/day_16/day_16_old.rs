use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve(input: &str) -> (u32, u32) {
    let valves = parse_input(input);
    let part_1 = solve_part_1(&valves, 30);
    let part_2 = solve_part_2(&valves, 25);
    (part_1, part_2)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    connects_to: Vec<&'a str>,
}

pub fn parse_input(input: &str) -> Vec<Valve> {
    input.trim_end().split("\n").map(Valve::from_str).collect()
}

impl Valve<'_> {
    pub fn new<'a>(name: &'a str, flow_rate: u32, connects_to: Vec<&'a str>) -> Valve<'a> {
        Valve {
            name,
            flow_rate,
            connects_to,
        }
    }

    fn from_str<'a>(s: &str) -> Valve {
        lazy_static! {
            static ref RE_VALVE: Regex = Regex::new(r"(-?[A-Z]{2})").unwrap();
            static ref RE_FR: Regex = Regex::new(r"(-?\d+)").unwrap();
        }
        let captures: Vec<&str> = RE_VALVE.find_iter(s).map(|s| s.as_str()).collect();
        let (name, others) = captures.split_at(1);

        let flow_rate = RE_FR.find(s).unwrap().as_str().parse::<u32>().unwrap();

        Valve::new(name[0], flow_rate, others.into())
    }
}

pub fn solve_part_1<'a>(valves: &'a Vec<Valve<'a>>, minutes_remaining: u32) -> u32 {
    let distances = calculate_distance_matrix(valves);
    let mut closed_valves = valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| (v.name, &v.flow_rate))
        .collect();

    recursively_open_valves("AA", &mut closed_valves, &distances, minutes_remaining, 0)
}

fn recursively_open_valves(
    cur_valve: &str,
    closed_valves: &mut HashSet<(&str, &u32)>,
    distances: &HashMap<(&str, &str), u32>,
    minutes_remaining: u32,
    total_pressure_released: u32,
) -> u32 {
    let mut max = total_pressure_released;
    for (closed_valve, flow_rate) in closed_valves.clone().iter() {
        let distance = distances.get(&(cur_valve, closed_valve)).unwrap();
        if distance < &minutes_remaining {
            closed_valves.remove(&(closed_valve, flow_rate));
            let remaining = minutes_remaining - distance;
            let released = recursively_open_valves(
                &closed_valve,
                closed_valves,
                distances,
                remaining,
                total_pressure_released + (*flow_rate * (remaining - 1)),
            );
            closed_valves.insert((closed_valve, flow_rate));
            if released > max {
                max = released;
            }
        }
    }
    max
}

fn calculate_distance_matrix<'a>(valves: &'a Vec<Valve<'a>>) -> HashMap<(&'a str, &'a str), u32> {
    let graph = valves
        .iter()
        .map(|v| (v.name, &v.connects_to))
        .collect::<HashMap<&str, &Vec<&str>>>();
    let non_broken_valves: Vec<&'a Valve<'a>> =
        valves.iter().filter(|valve| valve.flow_rate != 0).collect();
    let mut distances = HashMap::with_capacity(non_broken_valves.len() ^ 2);
    for src in &non_broken_valves {
        for target in &non_broken_valves {
            distances.insert(
                (src.name, target.name),
                find_shortest_path(src.name, target.name, &graph) + 1,
            );
        }
    }
    // Also calculate it for the start valve
    for target in &non_broken_valves {
        distances.insert(
            ("AA", target.name),
            find_shortest_path("AA", target.name, &graph),
        );
    }

    distances
}

fn find_shortest_path(src: &str, target: &str, graph: &HashMap<&str, &Vec<&str>>) -> u32 {
    let mut length = 0;
    let mut reachable = HashSet::from([src]);
    while !reachable.contains(target) {
        reachable = reachable
            .into_iter()
            .flat_map(|valve| *graph.get(valve).unwrap())
            .map(|s| *s)
            .collect();
        length += 1;
    }
    return length;
}

pub fn solve_part_2<'a>(valves: &'a Vec<Valve<'a>>, minutes_remaining: u32) -> u32 {
    let distances = calculate_distance_matrix(valves);
    let closed_valves = valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| (v.name, &v.flow_rate))
        .collect();

    let mut initial_state = State::new(
        "AA",
        0,
        "AA",
        0,
        0,
        minutes_remaining,
        &distances,
        closed_valves,
    );

    let sol = recursively_open_valves_part_2(&mut initial_state, 0);
    sol.max(23) - 23
}

#[derive(Debug, Clone)]
struct State<'a> {
    hum_goal: &'a str,
    distance_to_hum_goal: u32,
    elp_goal: &'a str,
    distance_to_elp_goal: u32,
    total_preasure_released: u32,
    minutes_remaining: u32,
    valve_graph: &'a HashMap<(&'a str, &'a str), u32>,
    closed_valves: HashSet<(&'a str, &'a u32)>,
}

impl<'a> State<'a> {
    fn new(
        hum_goal: &'a str,
        distance_to_hum_goal: u32,
        elp_goal: &'a str,
        distance_to_elp_goal: u32,
        total_preasure_released: u32,
        minutes_remaining: u32,
        valve_graph: &'a HashMap<(&'a str, &'a str), u32>,
        closed_valves: HashSet<(&'a str, &'a u32)>,
    ) -> State<'a> {
        let state: State<'_> = State {
            hum_goal,
            distance_to_hum_goal,
            elp_goal,
            distance_to_elp_goal,
            total_preasure_released,
            minutes_remaining,
            valve_graph,
            closed_valves,
        };
        state
    }

    fn generate_next_states<'b>(self: &'b Self) -> Vec<State<'b>> {
        let mut next_states = vec![];
        if self.distance_to_hum_goal == 0 {
            // Open valve and find new goal
            for (new_target, flow_rate) in &self.closed_valves {
                let mut new_closed_valves = self.closed_valves.clone();
                // println!("Closing valve {} (human)", new_target);
                new_closed_valves.remove(&(new_target, flow_rate));
                let distance = self.valve_graph.get(&(self.hum_goal, *new_target)).unwrap();
                if distance >= &self.minutes_remaining {
                    continue;
                }
                let time_new_valve_open = self.minutes_remaining - (distance);
                // println!("{}: {} * {}", new_target, time_new_valve_open, flow_rate);
                next_states.push(State::new(
                    new_target,
                    *distance,
                    self.elp_goal,
                    self.distance_to_elp_goal,
                    self.total_preasure_released + (time_new_valve_open * *flow_rate),
                    self.minutes_remaining,
                    self.valve_graph,
                    new_closed_valves,
                ))
            }
        } else if self.distance_to_elp_goal == 0 {
            // Open valve and find new goal
            for (new_target, flow_rate) in &self.closed_valves {
                let mut closed_valves = self.closed_valves.clone();
                // println!("Closing valve {} (elp)", new_target);
                closed_valves.remove(&(new_target, flow_rate));
                let distance = self.valve_graph.get(&(self.hum_goal, *new_target)).unwrap();
                if distance >= &self.minutes_remaining {
                    continue;
                }
                let time_new_valve_open = self.minutes_remaining - (distance);
                next_states.push(State::new(
                    self.hum_goal,
                    self.distance_to_hum_goal,
                    &new_target,
                    *distance,
                    self.total_preasure_released + (time_new_valve_open * *flow_rate),
                    self.minutes_remaining,
                    self.valve_graph,
                    closed_valves,
                ))
            }
        } else {
            // time jump
            let time_jump = u32::min(self.distance_to_hum_goal, self.distance_to_elp_goal);
            if time_jump > self.minutes_remaining {
                next_states = vec![];
            } else {
                next_states.push(State::new(
                    self.hum_goal,
                    self.distance_to_hum_goal - time_jump,
                    self.elp_goal,
                    self.distance_to_elp_goal - time_jump,
                    self.total_preasure_released,
                    self.minutes_remaining - time_jump,
                    self.valve_graph,
                    self.closed_valves.clone(),
                ))
            }
        }
        next_states
    }
}

fn recursively_open_valves_part_2<'a>(state: &'a State, i: u32) -> u32 {
    let mut max = state.total_preasure_released;
    let mut best_path = vec![];
    for next_state in state.generate_next_states() {
        let other = recursively_open_valves_part_2(&next_state.clone(), i + 1);
        if other > max {
            max = other;
        }
    }
    best_path.push(state.clone());
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read_day_input;

    use rstest::*;

    #[fixture]
    fn example_input_str() -> &'static str {
        Box::leak(read_day_input("test_day_16").into_boxed_str())
    }

    #[fixture]
    fn example_graph(example_input_str: &'static str) -> Vec<Valve<'static>> {
        parse_input(example_input_str)
    }

    #[rstest]
    fn test_example_part_1(example_graph: Vec<Valve<'static>>) {
        assert_eq!(solve_part_1(&example_graph, 30), 1651)
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(2, 0)]
    #[case(3, 20)]
    #[case(12, 573)]
    #[case(25, 1626)]
    #[case(26, 1707)]
    fn test_example_part_2(
        example_graph: Vec<Valve<'static>>,
        #[case] minutes_remaining: u32,
        #[case] total_pressure_released: u32,
    ) {
        assert_eq!(
            solve_part_2(&example_graph, minutes_remaining),
            total_pressure_released
        )
    }

    #[rstest]
    #[case("Valve AW has flow rate=0; tunnels lead to valves LG, TL", vec![Valve{name:"AW", flow_rate:0, connects_to: vec!["LG", "TL"]}])]
    #[case("Valve AW has flow rate=10; tunnels lead to valves LG, TL", vec![Valve{name:"AW", flow_rate:10, connects_to: vec!["LG", "TL"]}])]
    fn test_parse_input(#[case] example_input_str: &str, #[case] expected: Vec<Valve>) {
        assert_eq!(parse_input(example_input_str), expected)
    }
}
