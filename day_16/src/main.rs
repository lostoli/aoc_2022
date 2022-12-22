use pathfinding::prelude::bfs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Valve {
    label: String,
    flow_rate: u32,
    connections: Vec<String>,
}

impl Valve {
    fn new(label: String, flow_rate: u32, connections: Vec<String>) -> Valve {
        Valve {
            label,
            flow_rate,
            connections,
        }
    }
}

#[allow(dead_code)]
fn parse_line(line: String) -> Valve {
    let pattern: Regex =
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]* lead[s]* to valve[s]* (.+)")
            .unwrap();
    let captures = pattern.captures(&line).unwrap();
    Valve::new(
        captures.get(1).unwrap().as_str().to_string(),
        captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|v| v.to_string())
            .collect(),
    )
}

fn shortest_paths(
    valves: &HashMap<String, Valve>,
    starting_valve: &String,
    unvisited_valves: &Vec<Valve>,
) -> Vec<Vec<Valve>> {
    unvisited_valves
        .into_iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| get_shortest_path(v, &valves, starting_valve))
        .collect()
}

fn get_shortest_path(
    v: &Valve,
    valves: &HashMap<String, Valve>,
    starting_valve: &String,
) -> Vec<Valve> {
    bfs(
        &valves.get(starting_valve).unwrap(),
        |node| node.connections.iter().map(|c| valves.get(c).unwrap()),
        |node| *node == v,
    )
    .unwrap()
    .into_iter()
    .map(|x| x.clone())
    .collect()
}

fn path_pressure(path: &Vec<Valve>, time_left: u64) -> u64 {
    path.iter()
        .enumerate()
        .filter(|(i, _)| i < &30)
        .map(|(i, v)| (time_left - i as u64) * (v.flow_rate as u64))
        .sum()
}

fn path_unvisited_valves(path: &Vec<Valve>, valves: &HashMap<String, Valve>) -> Vec<Valve> {
    valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .filter(|v| !path.contains(v))
        .map(|v| v.clone())
        // .inspect(|v| println!("{:?}", v))
        .collect()
}

fn to_hash_map(v: &Vec<Valve>) -> HashMap<String, Valve> {
    v.iter().map(|v| (v.label.clone(), v.clone())).collect()
}

fn all_paths(
    source: Valve,
    goal: Valve,
    visited: &mut HashSet<String>,
    path: &mut Vec<Valve>,
    paths: &mut Vec<Vec<Valve>>,
    valves: &HashMap<String, Valve>,
) {
    visited.insert(source.label.clone());
    path.push(source.clone());
    if source == goal {
        paths.push(path.clone())
    } else {
        for connection in source.connections {
            if !visited.contains(&connection) {
                all_paths(
                    valves.get(&connection).unwrap().clone(),
                    goal.clone(),
                    visited,
                    path,
                    paths,
                    valves,
                );
            }
        }
    }

    path.pop();
    visited.remove(&source.label.clone());
}

fn get_input() -> HashMap<String, Valve> {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            //
            lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(parse_line)
                .collect::<Vec<Valve>>()
                .into_iter()
                .map(|v| (v.label.clone(), v))
                .collect()
        } else {
            HashMap::new()
        }
    } else {
        HashMap::new()
    }
}

fn search(
    time: u32,
    current_valve: String,
    pressure: u64,
    max_so_far: &mut u64,
    open_valves: &mut HashSet<String>,
    seen: &mut HashMap<(u32, String), u64>,
    valves: &HashMap<String, Valve>,
) {
    let cvc = current_valve.clone();

    let is_seen = seen.get(&(time, cvc));

    if is_seen.is_some() && is_seen.unwrap() >= &pressure {
        return;
    }

    let cvc = current_valve.clone();

    seen.insert((time, cvc), pressure);

    if time == 30 {
        *max_so_far = (*max_so_far).max(pressure);
        return;
    } else {
        let net_pressure: u64 = open_valves
            .iter()
            .map(|v| valves.get(v).unwrap().flow_rate)
            .sum::<u32>() as u64;

        let cvc = current_valve.clone();
        if !open_valves.contains(&current_valve)
            && valves.get(&current_valve).unwrap().flow_rate > 0
        {
            open_valves.insert(cvc);
            let cvc = current_valve.clone();
            search(
                time + 1,
                cvc,
                pressure + net_pressure + valves.get(&current_valve).unwrap().flow_rate as u64,
                max_so_far,
                open_valves,
                seen,
                valves,
            );
            open_valves.remove(&current_valve);
        }

        for valve in valves.get(&current_valve).unwrap().connections.iter() {
            search(
                time + 1,
                valve.clone(),
                pressure + net_pressure,
                max_so_far,
                open_valves,
                seen,
                valves,
            )
        }
    }
}

fn main() {
    let valves = get_input();
    let valves_to_open: HashSet<_> = valves
        .values()
        .filter(|valve| valve.flow_rate > 0)
        .collect();

    let mut open_valves: HashSet<String> = HashSet::new();
    let mut seen: HashMap<(u32, String), u64> = HashMap::new();
    let mut max_so_far: u64 = 0;

    search(
        1,
        "AA".to_string(),
        0,
        &mut max_so_far,
        &mut open_valves,
        &mut seen,
        &valves,
    );

    println!("{:?}", max_so_far);
}
