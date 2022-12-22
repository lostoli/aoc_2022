use std::collections::HashMap;
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

#[derive(Debug)]
enum PosType {
    EMPTY,
    ROCK,
}

type Map = HashMap<(isize, isize), PosType>;
type Edges = HashMap<(isize, isize, i8), (isize, isize)>;

#[derive(Debug)]
struct CurrentPos {
    pos: (isize, isize),
    direction: i8,
}

impl CurrentPos {
    fn turn_left(&mut self) {
        self.direction = (self.direction - 1).rem_euclid(4)
    }
    fn turn_right(&mut self) {
        self.direction = (self.direction + 1).rem_euclid(4)
    }

    fn score(&self) -> isize {
        1000 * (self.pos.1 + 1) + 4 * (self.pos.0 + 1) + self.direction.rem_euclid(4) as isize
    }

    fn update_pos(&mut self, magnitude: isize, map: &Map, edges: &Edges) {
        match self.direction {
            3 => self.move_north(magnitude, map, edges),
            0 => self.move_east(magnitude, map, edges),
            1 => self.move_south(magnitude, map, edges),
            2 => self.move_west(magnitude, map, edges),
            _ => panic!("This shouldn't happen"),
        }
    }

    fn update_dir(&mut self, dir: char) {
        match dir {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            _ => panic!("This shouldn't happen."),
        }
    }

    fn move_north(&mut self, magnitude: isize, map: &Map, edges: &Edges) {
        for _ in 0..magnitude {
            let new_pos = (self.pos.0, self.pos.1 - 1);
            if let Some(pos_type) = map.get(&new_pos) {
                // We don't need to wrap
                match pos_type {
                    PosType::ROCK => {
                        break;
                    }
                    PosType::EMPTY => self.pos = new_pos,
                }
            } else {
                // We have to wrap
                // let wrapped_pos = (self.pos.0, max_in_col(self.pos.0, map));
                let wrapped_pos = edges.get(&(new_pos.0, new_pos.1, self.direction)).unwrap();
                if let Some(pos_type) = map.get(&wrapped_pos) {
                    match pos_type {
                        PosType::ROCK => {
                            break;
                        }
                        PosType::EMPTY => self.pos = *wrapped_pos,
                    }
                }
            }
        }
    }
    fn move_east(&mut self, magnitude: isize, map: &Map, edges: &Edges) {
        for _ in 0..magnitude {
            let new_pos = (self.pos.0 + 1, self.pos.1);
            if let Some(pos_type) = map.get(&new_pos) {
                // We don't need to wrap
                match pos_type {
                    PosType::ROCK => {
                        break;
                    }
                    PosType::EMPTY => self.pos = new_pos,
                }
            } else {
                // We have to wrap
                // let wrapped_pos = (min_in_row(self.pos.1, map), self.pos.1);
                let wrapped_pos = edges.get(&(new_pos.0, new_pos.1, self.direction)).unwrap();
                if let Some(pos_type) = map.get(&wrapped_pos) {
                    match pos_type {
                        PosType::ROCK => {
                            break;
                        }
                        PosType::EMPTY => self.pos = *wrapped_pos,
                    }
                }
            }
        }
    }
    fn move_south(&mut self, magnitude: isize, map: &Map, edges: &Edges) {
        for _ in 0..magnitude {
            let new_pos = (self.pos.0, self.pos.1 + 1);
            if let Some(pos_type) = map.get(&new_pos) {
                // We don't need to wrap
                match pos_type {
                    PosType::ROCK => {
                        break;
                    }
                    PosType::EMPTY => self.pos = new_pos,
                }
            } else {
                // We have to wrap
                // println!("Moving south but wrapping..")
                // let wrapped_pos = (self.pos.0, min_in_col(self.pos.0, map));
                let wrapped_pos = edges.get(&(new_pos.0, new_pos.1, self.direction)).unwrap();
                if let Some(pos_type) = map.get(&wrapped_pos) {
                    match pos_type {
                        PosType::ROCK => {
                            break;
                        }
                        PosType::EMPTY => self.pos = *wrapped_pos,
                    }
                }
            }
        }
    }
    fn move_west(&mut self, magnitude: isize, map: &Map, edges: &Edges) {
        for _ in 0..magnitude {
            let new_pos = (self.pos.0 - 1, self.pos.1);
            if let Some(pos_type) = map.get(&new_pos) {
                // We don't need to wrap
                match pos_type {
                    PosType::ROCK => {
                        break;
                    }
                    PosType::EMPTY => self.pos = new_pos,
                }
            } else {
                // We have to wrap
                // let wrapped_pos = (max_in_row(self.pos.1, map), self.pos.1);
                println!("{:?}", new_pos);
                let wrapped_pos = edges.get(&(new_pos.0, new_pos.1, self.direction)).unwrap();
                if let Some(pos_type) = map.get(&wrapped_pos) {
                    match pos_type {
                        PosType::ROCK => {
                            break;
                        }
                        PosType::EMPTY => self.pos = *wrapped_pos,
                    }
                }
            }
        }
    }
}

fn max_in_row(row: isize, map: &Map) -> isize {
    map.iter()
        .filter(|(j, _)| j.1 == row)
        .map(|(j, _)| j.0)
        .max()
        .unwrap()
}

fn min_in_row(row: isize, map: &Map) -> isize {
    map.iter()
        .filter(|(j, _)| j.1 == row)
        .map(|(j, _)| j.0)
        .min()
        .unwrap()
}

fn max_in_col(col: isize, map: &Map) -> isize {
    map.iter()
        .filter(|(j, _)| j.0 == col)
        .map(|(j, _)| j.1)
        .max()
        .unwrap()
}

fn min_in_col(col: isize, map: &Map) -> isize {
    map.iter()
        .filter(|(j, _)| j.0 == col)
        .map(|(j, _)| j.1)
        .min()
        .unwrap()
}

#[allow(dead_code)]
fn parse_line(idx: isize, line: String, map: &mut Map) {
    line.chars().enumerate().for_each(|(j, c)| {
        if c == '.' {
            map.insert((j as isize, idx), PosType::EMPTY);
        } else if c == '#' {
            map.insert((j as isize, idx), PosType::ROCK);
        }
    })
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let raw = lines.filter_map(|x| x.ok()).collect::<Vec<String>>();
            let map_lines = raw
                .clone()
                .into_iter()
                .take_while(|line| line != "")
                .collect::<Vec<String>>();

            let instructions = raw.clone().into_iter().last().unwrap();

            let mut map: Map = HashMap::new();

            map_lines
                .into_iter()
                .enumerate()
                .for_each(|(i, line)| parse_line(i as isize, line, &mut map));

            let stating_x = min_in_row(0, &map);

            let mut pos = CurrentPos {
                pos: (stating_x, 0),
                direction: 0,
            };

            let magnitudes: Vec<isize> = instructions
                .split(|c| c == 'L' || c == 'R')
                .map(|s| s.parse::<isize>().unwrap())
                .collect();

            let directions: Vec<char> = instructions
                .chars()
                .filter(|&c| c == 'L' || c == 'R')
                .collect();

            let mut magnitudes_iter = magnitudes.iter().peekable();
            let mut directions_iter = directions.iter();
            let mut edges: Edges = HashMap::new();

            for i in 0..50 {
                //i

                //moving south
                edges.insert((100 + i, 50, 1), (99, 50 + i));

                //moving east
                edges.insert((100, 50 + i, 0), (100 + i, 49));
            }

            for i in 0..50 {
                //ii

                // moving east
                edges.insert((150, i, 0), (99, 149 - i));

                // moving east
                edges.insert((100, 100 + i, 0), (149, -i));
            }

            for i in 0..50 {
                //iii

                // moving south
                edges.insert((50 + i, 150, 1), (49, 150 + i));

                // moving east
                edges.insert((50, 150 + i, 0), (50 + i, 149));
            }

            for i in 0..50 {
                //iv

                // moving north
                edges.insert((i, 99, 3), (50, 50 + i));

                // moving west
                edges.insert((49, 50 + i, 2), (i, 100));
            }

            for i in 0..50 {
                //v

                // moving west
                edges.insert((49, i, 2), (0, 149 - i));

                //moving west
                edges.insert((-1, 100 + i, 2), (49, -i));
            }

            for i in 0..50 {
                //vi

                // moving north
                edges.insert((50 + i, -1, 3), (0, 150 + i));

                // moving west
                edges.insert((-1, 150 + i, 2), (50 + i, 0));
            }

            for i in 0..50 {
                //vii

                //moving north
                edges.insert((100 + i, -1, 3), (0 + i, 199));

                //move_south
                edges.insert((0 + i, 200, 1), (100 + i, 0));
            }

            while magnitudes_iter.peek().is_some() {
                pos.update_pos(*magnitudes_iter.next().unwrap(), &map, &edges);
                if let Some(d) = directions_iter.next() {
                    println!("{:?}", pos);
                    println!("{:?}", d);
                    pos.update_dir(*d);
                }
            }

            println!("{:?}", pos.pos);
            println!("{:?}", pos.score());
            println!("{}", (0 as i8).rem_euclid(4));
            // println!("{:?}", instructions);
        }
    }
}
