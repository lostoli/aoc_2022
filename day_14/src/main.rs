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

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

const STARTING_POINT: Point = Point { x: 500, y: 0 };

fn expand_window(p0: &Point, p1: &Point) -> Vec<Point> {
    if p0.x == p1.x && p0.y > p1.y {
        (p1.y..p0.y + 1).map(|y| Point { x: p0.x, y }).collect()
    } else if p0.x == p1.x {
        (p0.y..p1.y + 1).map(|y| Point { x: p0.x, y }).collect()
    } else if p0.x > p1.x {
        (p1.x..p0.x + 1).map(|x| Point { x, y: p0.y }).collect()
    } else {
        (p0.x..p1.x + 1).map(|x| Point { x, y: p0.y }).collect()
    }
}

fn parse_line(line: String) -> Vec<Point> {
    let ps = line
        .replace(" -> ", ",")
        .split(",")
        .into_iter()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| Point {
            x: chunk[0].parse::<i32>().unwrap(),
            y: chunk[1].parse::<i32>().unwrap(),
        })
        .collect::<Vec<Point>>();
    ps.windows(2)
        .map(|w| expand_window(&w[0], &w[1]))
        .flatten()
        .collect::<Vec<Point>>()
}

fn add_sand(cliff: &mut HashSet<Point>, max_depth: i32) -> Option<bool> {
    let mut p = STARTING_POINT;
    loop {
        if !cliff.contains(&Point { x: p.x, y: p.y + 1 }) {
            p.y += 1;
        } else if !cliff.contains(&Point {
            x: p.x - 1,
            y: p.y + 1,
        }) {
            p.y += 1;
            p.x -= 1;
        } else if !cliff.contains(&Point {
            x: p.x + 1,
            y: p.y + 1,
        }) {
            p.y += 1;
            p.x += 1;
        } else {
            break;
        }
    }
    if p.y == 0 {
        return None;
    }
    Some(cliff.insert(p))
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let mut forbidden_points = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|line| parse_line(line))
                .flatten()
                .collect::<HashSet<Point>>();
            let max_depth = forbidden_points.iter().map(|p| p.y).max().unwrap();
            forbidden_points.extend(expand_window(
                &Point {
                    x: -500,
                    y: max_depth + 2,
                },
                &Point {
                    x: 1500,
                    y: max_depth + 2,
                },
            ));
            let mut c = 0;
            loop {
                match add_sand(&mut forbidden_points, max_depth) {
                    None => break,
                    Some(_) => c += 1,
                }
            }
            println!("{:?}", c + 1)
        }
    }
}
