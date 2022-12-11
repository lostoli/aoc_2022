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

fn check_adjacent(head_postion: &mut (i32, i32), tail_position: &mut (i32, i32)) -> bool {
    (((head_postion.0 - tail_position.0).abs().pow(2)
        + (head_postion.1 - tail_position.1).abs().pow(2)) as f64)
        .sqrt()
        < (2 as f64)
}

fn move_right(head_position: &mut (i32, i32), tail_position: &mut (i32, i32)) {
    let old_head_position = head_position.clone();
    head_position.0 += 1;
    if !check_adjacent(head_position, tail_position) {
        tail_position.0 = old_head_position.0;
        tail_position.1 = old_head_position.1;
    };
}
fn move_up(head_position: &mut (i32, i32), tail_position: &mut (i32, i32)) {
    let old_head_position = head_position.clone();
    head_position.1 += 1;
    if !check_adjacent(head_position, tail_position) {
        tail_position.0 = old_head_position.0;
        tail_position.1 = old_head_position.1;
    };
}
fn move_left(head_position: &mut (i32, i32), tail_position: &mut (i32, i32)) {
    let old_head_position = head_position.clone();
    head_position.0 -= 1;
    if !check_adjacent(head_position, tail_position) {
        tail_position.0 = old_head_position.0;
        tail_position.1 = old_head_position.1;
    };
}
fn move_down(head_position: &mut (i32, i32), tail_position: &mut (i32, i32)) {
    let old_head_position = head_position.clone();
    head_position.1 -= 1;
    if !check_adjacent(head_position, tail_position) {
        tail_position.0 = old_head_position.0;
        tail_position.1 = old_head_position.1;
    };
}

// fn move_snake(new_head_posit)

#[allow(dead_code)]
fn parse_line(
    line: String,
    head_postion: &mut (i32, i32),
    tail_position: &mut (i32, i32),
) -> Vec<(i32, i32)> {
    let mut tail_positions: Vec<(i32, i32)> = vec![];
    if let Some((direction, num_steps)) = line.split_once(' ') {
        let num_steps_int = num_steps.parse::<u32>().unwrap();
        match direction {
            "R" => (0..num_steps_int).for_each(|_| {
                move_right(head_postion, tail_position);
                tail_positions.push(tail_position.clone());
            }),
            "U" => (0..num_steps_int).for_each(|_| {
                move_up(head_postion, tail_position);
                tail_positions.push(tail_position.clone());
            }),
            "L" => (0..num_steps_int).for_each(|_| {
                move_left(head_postion, tail_position);
                tail_positions.push(tail_position.clone());
            }),
            "D" => (0..num_steps_int).for_each(|_| {
                move_down(head_postion, tail_position);
                tail_positions.push(tail_position.clone());
            }),
            _ => panic!("Direction not understood..."),
        }
    } else {
        panic!("Could not parse line!")
    }
    tail_positions
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let mut head_postion = (0, 0);
            let mut tail_position = (0, 0);
            let r = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|line| parse_line(line, &mut head_postion, &mut tail_position))
                .flatten()
                .collect::<HashSet<_>>();
            println!("{:?}", r.len())
        }
    }
}
