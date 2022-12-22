use regex::Regex;
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

#[allow(dead_code)]
fn parse_line(line: String) -> BluePrint {
    let ore_pattern: Regex = Regex::new(r"Each ore robot costs (\d+) ore.").unwrap();
    let ore_captures = ore_pattern.captures(&line).unwrap();
    let clay_pattern: Regex = Regex::new(r"Each clay robot costs (\d+) ore.").unwrap();
    let clay_captures = clay_pattern.captures(&line).unwrap();
    let obsidon_pattern: Regex =
        Regex::new(r"Each obsidian robot costs (\d+) ore and (\d+) clay.").unwrap();
    let obsidon_captures = obsidon_pattern.captures(&line).unwrap();
    let geode_pattern: Regex =
        Regex::new(r"Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let geode_captures = geode_pattern.captures(&line).unwrap();

    BluePrint {
        ore_robot_cost: ore_captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap(),
        clay_robot_cost: clay_captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap(),
        obsidon_robot_cost: (
            obsidon_captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
            obsidon_captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
        ),
        geode_robot_cost: (
            geode_captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
            geode_captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
        ),
    }
}

#[derive(Debug, Clone, Copy)]
struct BluePrint {
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidon_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    num_resources: [u32; 4],
    num_robots: [u32; 4],
}

fn add(a0: [u32; 4], a1: [u32; 4], a2: Option<[i32; 4]>) -> [u32; 4] {
    let mut output: [i32; 4] = [0; 4];
    for i in 0..4 {
        output[i] = (a0[i] + a1[i]) as i32
    }
    if let Some(a) = a2 {
        for i in 0..4 {
            output[i] += a[i]
        }
    }
    output.map(|i| i.max(0) as u32)
}

fn children(state: &State, bp: &BluePrint, time_left: u32) -> Vec<State> {
    let mut children = vec![];
    if state.num_resources[0] >= bp.geode_robot_cost.0
        && state.num_resources[2] >= bp.geode_robot_cost.1
    {
        children.push(State {
            num_resources: add(
                state.num_resources,
                state.num_robots,
                Some([
                    -(bp.geode_robot_cost.0 as i32),
                    0,
                    -(bp.geode_robot_cost.1 as i32),
                    0,
                ]),
            ),
            num_robots: add(state.num_robots, [0, 0, 0, 1], None),
        });
        return children;
    };
    if state.num_resources[0] >= bp.obsidon_robot_cost.0
        && state.num_resources[1] >= bp.obsidon_robot_cost.1
    {
        let new_state = State {
            num_resources: add(
                state.num_resources,
                state.num_robots,
                Some([
                    -(bp.obsidon_robot_cost.0 as i32),
                    -(bp.obsidon_robot_cost.1 as i32),
                    0,
                    0,
                ]),
            ),
            num_robots: add(state.num_robots, [0, 0, 1, 0], None),
        };
        // if state.num_resources[2] + time_left * (state.num_robots[2] + 1) >= bp.geode_robot_cost.1 {
        //     return vec![new_state];
        // }
        children.push(new_state);
    };
    if state.num_resources[0] >= bp.ore_robot_cost
        && state.num_robots[0]
            <= bp.ore_robot_cost
                .max(bp.clay_robot_cost)
                .max(bp.geode_robot_cost.0)
                .max(bp.obsidon_robot_cost.0)
    {
        // Buy ore robot...
        let new_state = State {
            num_resources: add(
                state.num_resources,
                state.num_robots,
                Some([-(bp.ore_robot_cost as i32), 0, 0, 0]),
            ),
            num_robots: add(state.num_robots, [1, 0, 0, 0], None),
        };
        if state.num_resources[1] + time_left > bp.obsidon_robot_cost.1 {
            children.push(new_state);
        }
    }
    if state.num_resources[0] >= bp.clay_robot_cost && state.num_robots[1] <= bp.geode_robot_cost.0 {
        // Buy clay robot...
        children.push(State {
            num_resources: add(
                state.num_resources,
                state.num_robots,
                Some([-(bp.clay_robot_cost as i32), 0, 0, 0]),
            ),
            num_robots: add(state.num_robots, [0, 1, 0, 0], None),
        });
    }
    // if state.num_resources[0]
    //     < bp.clay_robot_cost
    //         .max(bp.ore_robot_cost)
    //         .max(bp.obsidon_robot_cost.0)
    //         .max(bp.geode_robot_cost.0)
    // {
    //     // Do nothing...
    //     children.push(State {
    //         num_resources: add(state.num_resources, state.num_robots, None),
    //         num_robots: state.num_robots,
    //     });
    // }
    children.push(State {
        num_resources: add(state.num_resources, state.num_robots, None),
        num_robots: state.num_robots,
    });

    children
}

fn search(
    max_so_far: &mut u32,
    state: State,
    time: u32,
    bp: &BluePrint,
    seen: &mut HashSet<State>,
) {
    if seen.contains(&state) {
        return;
    }

    seen.insert(state);

    if time == 24 {
        *max_so_far = (*max_so_far).max(state.num_resources[3]);
        return;
    } else {
        for child in children(&state, bp, 24 - time) {
            search(max_so_far, child, time + 1, bp, seen)
        }
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let r: Vec<_> = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(parse_line)
                .collect();

            let starting_state = State {
                num_resources: [0; 4],
                num_robots: [1, 0, 0, 0],
            };

            let mut total: u64 = 0;

            for (i, bp) in r.iter().enumerate() {
                let mut max_so_far: u32 = 0;
                let mut seen = HashSet::new();
                search(&mut max_so_far, starting_state, 0, &bp, &mut seen);
                println!("{:?}", max_so_far);
                total += (i as u64 + 1) * max_so_far as u64
            }
            println!("{:?}", total)
        }
    }
}
