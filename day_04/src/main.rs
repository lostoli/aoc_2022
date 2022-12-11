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
fn parse_line(line: &String) -> ([i32; 2], [i32; 2]) {
    let positions: Vec<&str> = line
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.split("-").collect::<Vec<&str>>())
        .flatten()
        .collect();
    let _p = |index: usize| -> i32 { positions.iter().nth(index).unwrap().parse::<i32>().unwrap() };
    ([_p(0), _p(1)], [_p(2), _p(3)])
}

#[allow(dead_code)]
fn is_contained(i_s: ([i32; 2], [i32; 2])) -> u32 {
    let mut sorted_intervals = i_s;
    if i_s.0[1] - i_s.0[0] > i_s.1[1] - i_s.1[0] {
        sorted_intervals = (i_s.1, i_s.0)
    }
    println!("{:?}", sorted_intervals);
    if sorted_intervals.0[0] >= sorted_intervals.1[0]
        && sorted_intervals.0[1] <= sorted_intervals.1[1]
    {
        1
    } else {
        0
    }
}

fn overlaps(i_s: ([i32; 2], [i32; 2])) -> u32 {
    let set_0: HashSet<i32> = (i_s.0[0]..i_s.0[1] + 1).collect();
    let set_1: HashSet<i32> = (i_s.1[0]..i_s.1[1] + 1).collect();
    if set_0.intersection(&set_1).into_iter().nth(0).is_some() {
        1
    } else {
        0
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let r: u32 = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .iter()
                .map(|x| overlaps(parse_line(x)))
                .sum::<u32>();

            println!("{:?}", r)
        }
    }
}
