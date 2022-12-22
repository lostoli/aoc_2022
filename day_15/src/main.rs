use regex::Regex;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_line_to_sensor(line: &String) -> Point {
    let sensor: Regex = Regex::new(r"Sensor at x=(-*\d+), y=(-*\d+)").unwrap();
    let captures = sensor.captures(&line).unwrap();
    Point {
        x: captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        y: captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
    }
}

fn manhatten(p_0: &Point, p_1: &Point) -> u64 {
    ((p_0.x - p_1.x).abs() + (p_0.y - p_1.y).abs()) as u64
}

fn parse_line_to_beacon(line: String) -> Point {
    let beacon: Regex = Regex::new(r"beacon is at x=(-*\d+), y=(-*\d+)").unwrap();
    let captures = beacon.captures(&line).unwrap();
    Point {
        x: captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        y: captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
    }
}

fn ball_line_intersects(center: &Point, radius: u64, line: i64) -> (Option<Point>, Option<Point>) {
    let closest_point = Point {
        x: center.x,
        y: line,
    };
    if manhatten(&closest_point, &center) > radius {
        (None, None)
    } else if manhatten(&closest_point, &center) == radius {
        (Some(closest_point), Some(closest_point))
    } else {
        let h = (radius as i64) - (closest_point.y - center.y).abs();
        (
            Some(Point {
                x: center.x - h,
                y: line,
            }),
            Some(Point {
                x: center.x + h,
                y: line,
            }),
        )
    }
}

fn add_interval_to_disjoint_collection(
    intervals: Vec<(Point, Point)>,
    new_interval: (Point, Point),
) -> Vec<(Point, Point)> {
    let mut new_intervals = vec![];
    let mut big_interval = new_interval;
    for interval in intervals {
        if (interval.0.x <= big_interval.0.x && big_interval.0.x <= interval.1.x)
            || (big_interval.0.x <= interval.0.x && interval.0.x <= big_interval.1.x)
        {
            big_interval.0.x = interval.0.x.min(big_interval.0.x);
            big_interval.1.x = interval.1.x.max(big_interval.1.x);
        } else {
            new_intervals.push(interval)
        }
    }
    new_intervals.push(big_interval);
    new_intervals
}

fn make_intervals_disjoint(
    intervals: Vec<(Point, Point)>,
    new_interval: (Option<Point>, Option<Point>),
) -> Vec<(Point, Point)> {
    match new_interval {
        (Some(p_0), Some(p_1)) => add_interval_to_disjoint_collection(intervals, (p_0, p_1)),
        (None, None) => intervals,
        (None, Some(_)) => panic!("{}", "Errror"),
        (Some(_), None) => panic!("{}", "Errror"),
    }
}

static M: i64 = 4000000;
static m: i64 = 4000000;

fn count_locations(known_locations: &Vec<(Point, Point)>, line: i64) -> Vec<(Point, Point)> {
    let intervals = known_locations
        .iter()
        .map(|loc| ball_line_intersects(&loc.0, manhatten(&loc.0, &loc.1), line))
        .fold(vec![], |accum, value| make_intervals_disjoint(accum, value));

    subtract_intervals(
        (Point { x: 0, y: line }, Point { x: M, y: line }),
        intervals,
    )
    .iter()
    .filter(|i| i.0.x == i.1.x)
    .cloned()
    .collect()
    // .map(|i| (i.0.x - i.1.x).abs())
    // .sum::<i64>()
    // .try_into()
    // .unwrap()
}

fn subtract_intervals(
    big_interval: (Point, Point),
    intervals: Vec<(Point, Point)>,
) -> Vec<(Point, Point)> {
    let mut new_intervals = intervals;
    new_intervals.sort_by_key(|i| i.0.x);
    new_intervals
        .windows(2)
        .map(|w| {
            (
                Point {
                    x: w[0].1.x + 1,
                    y: w[0].1.y,
                },
                Point {
                    x: w[1].0.x - 1,
                    y: w[1].0.y,
                },
            )
        })
        .filter(|i| i.0.x > big_interval.0.x && i.1.x < big_interval.1.x)
        .filter(|i| i.0.x <= i.1.x)
        .collect()
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let r = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|line| (parse_line_to_sensor(&line), parse_line_to_beacon(line)))
                .collect::<Vec<(Point, Point)>>();
            for line in 0..M {
                let count = count_locations(&r, line);
                if count.len() == 1 {
                    println!("{:?}", count[0].0.x * m + count[0].0.y);
                    break;
                }
            }
            // println!("{:?}", count)
        }
    }
}
