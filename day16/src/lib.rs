mod parser;

use std::collections::{hash_map::Entry, BTreeSet, HashMap, VecDeque};

use parser::parse_input;

pub fn solve_part_1(input: &str) -> u32 {
    let valves = parse_input(input);
    let cave = Cave::new(valves);

    *simulate_flows(&cave, 30)
        .values()
        .max()
        .expect("Failed to find max pressure released")
}

#[allow(clippy::map_entry)]
pub fn solve_part_2(input: &str) -> u32 {
    let valves = parse_input(input);
    let cave = Cave::new(valves);

    let flows = simulate_flows(&cave, 26);

    let mut table: HashMap<BTreeSet<&str>, u32> = HashMap::new();
    for ((_, _, open_valves), flow) in flows {
        if table.contains_key(&open_valves) {
            *table.get_mut(&open_valves).unwrap() = table[&open_valves].max(flow);
        } else {
            table.insert(open_valves, flow);
        }
    }

    let mut result = 0u32;
    let all_valve_names: BTreeSet<&str> = cave.valves.values().map(|v| v.name).collect();
    for valve_names in power_set(&all_valve_names.iter().collect::<Vec<_>>()) {
        let other_names: BTreeSet<&str> = all_valve_names
            .clone()
            .into_iter()
            .filter(|v| !valve_names.contains(&&v))
            .collect();

        let val = *table.get(&other_names).unwrap_or(&0);

        result = result.max(val);

        let mut valve_names_2: BTreeSet<&str> = valve_names.iter().map(|&&&v| v).collect();
        while !valve_names_2.is_empty() {
            let val2 = *table.get(&valve_names_2).unwrap_or(&0);
            result = result.max(val + val2);
            valve_names_2.pop_last();
        }
    }

    result
}

fn simulate_flows<'a>(
    cave: &'a Cave,
    time: u32,
) -> HashMap<(&'a str, u32, BTreeSet<&'a str>), u32> {
    // (current_valve, time_left, open_valves)
    let mut dynamic = HashMap::new();
    dynamic.insert(("AA", time, BTreeSet::new()), 0);

    let mut queue = VecDeque::from([("AA", time, BTreeSet::new())]);
    while let Some(entry) = queue.pop_front() {
        let flow = dynamic[&entry];

        let (valve_name, time_left, mut open_valves) = entry;

        let valve = &cave.valves[&valve_name];

        for &other_name in cave.valves.keys() {
            let move_time = cave.distances[&(valve_name, other_name)];
            if move_time <= time_left {
                let key = (other_name, time_left - move_time, open_valves.clone());
                if !dynamic.contains_key(&key) || dynamic[&key] < flow {
                    dynamic.insert(key.clone(), flow);
                    queue.push_back(key);
                }
            }
        }

        if valve.flow_rate > 0 && !open_valves.contains(&valve_name) && time_left > 0 {
            let flow_here = (time_left - 1) * valve.flow_rate;
            open_valves.insert(valve_name);

            let key = (valve_name, time_left - 1, open_valves);
            if !dynamic.contains_key(&key) || dynamic[&key] < flow + flow_here {
                dynamic.insert(key.clone(), flow + flow_here);
                queue.push_back(key);
            }
        }
    }

    dynamic
}

fn power_set<T>(vec: &[T]) -> impl Iterator<Item = Vec<&T>> {
    (0..2usize.pow(vec.len() as u32)).map(|i| {
        vec.iter()
            .enumerate()
            .filter(|&(j, _)| (i >> j) & 1 == 1)
            .map(|(_, el)| el)
            .collect()
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    adjacencies: Vec<&'a str>,
}

#[derive(Debug)]
struct Cave<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
    distances: HashMap<(&'a str, &'a str), u32>,
}

impl<'a> Valve<'a> {
    fn new(name: &'a str, flow_rate: u32, adjacencies: Vec<&'a str>) -> Self {
        Self {
            name,
            flow_rate,
            adjacencies,
        }
    }
}

impl<'a> Cave<'a> {
    fn new(valves: Vec<Valve<'a>>) -> Self {
        let valves: HashMap<_, _> = valves.into_iter().map(|v| (v.name, v)).collect();

        let num_valves = valves.len();
        let mut distances = HashMap::with_capacity(num_valves * num_valves);
        for &start_valve_name in valves.keys() {
            distances.insert((start_valve_name, start_valve_name), 0);

            let mut to_consider = VecDeque::from([start_valve_name]);
            while let Some(valve_name) = to_consider.pop_front() {
                let root_key = (start_valve_name, valve_name);

                for &adjacent_name in &valves[valve_name].adjacencies {
                    let adj_key = (start_valve_name, adjacent_name);
                    let dist = distances[&root_key] + 1;
                    if let Entry::Vacant(entry) = distances.entry(adj_key) {
                        entry.insert(dist);
                        to_consider.push_back(adjacent_name);
                    }
                }
            }
        }

        // let valves = valves
        //     .into_iter()
        //     .filter(|(_, v)| v.flow_rate > 0 || v.name == "AA")
        //     .collect();

        Self { valves, distances }
    }
}
