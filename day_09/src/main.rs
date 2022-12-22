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

fn check_adjacent(head_postion: (i32, i32), tail_position: (i32, i32)) -> bool {
    (((head_postion.0 - tail_position.0).abs().pow(2)
        + (head_postion.1 - tail_position.1).abs().pow(2)) as f64)
        .sqrt()
        < (2 as f64)
}

fn move_whole_snake(
    snake_positions: &mut [(i32, i32); 10],
    displacement: (i32, i32),
    num_steps_int: u32,
    tail_positions: &mut Vec<(i32, i32)>,
) {
    (0..num_steps_int).for_each(|_| {
        // let old_snake_positions = snake_positions.clone();
        match displacement {
            (1, 0) => snake_positions[9].0 += 1,
            (0, 1) => snake_positions[9].1 += 1,
            (-1, 0) => snake_positions[9].0 -= 1,
            (0, -1) => snake_positions[9].1 -= 1,
            _ => {}
        }
        for i in (0..9).rev() {
            println!("{:?}", (i, snake_positions[i], snake_positions[i + 1]));
            if !check_adjacent(snake_positions[i].clone(), snake_positions[i + 1].clone()) {
                let new_position =
                    move_tail(snake_positions[i].clone(), snake_positions[i + 1].clone());
                snake_positions[i] = new_position;
            };
        }
        tail_positions.push(snake_positions[0]);
    })
}

fn move_tail(tail_position: (i32, i32), head_postion: (i32, i32)) -> (i32, i32) {
    let mut tail_position = tail_position;
    if tail_position.0 == head_postion.0 {
        tail_position.1 += {
            if head_postion.1 > tail_position.1 {
                1
            } else {
                -1
            }
        }
    } else if tail_position.1 == head_postion.1 {
        tail_position.0 += {
            if head_postion.0 > tail_position.0 {
                1
            } else {
                -1
            }
        }
    } else {
        tail_position.1 += {
            if head_postion.1 > tail_position.1 {
                1
            } else {
                -1
            }
        };
        tail_position.0 += {
            if head_postion.0 > tail_position.0 {
                1
            } else {
                -1
            }
        }
    };
    tail_position
}

#[allow(dead_code)]
fn parse_line(line: String, snake_positions: &mut [(i32, i32); 10]) -> Vec<(i32, i32)> {
    let mut tail_positions: Vec<(i32, i32)> = vec![];
    if let Some((direction, num_steps)) = line.split_once(' ') {
        let num_steps_int = num_steps.parse::<u32>().unwrap();
        match direction {
            "R" => move_whole_snake(snake_positions, (1, 0), num_steps_int, &mut tail_positions),
            "U" => move_whole_snake(snake_positions, (0, 1), num_steps_int, &mut tail_positions),
            "L" => move_whole_snake(snake_positions, (-1, 0), num_steps_int, &mut tail_positions),
            "D" => move_whole_snake(snake_positions, (0, -1), num_steps_int, &mut tail_positions),
            _ => panic!("Direction not understood..."),
        }
    } else {
        panic!("Could not parse line!")
    }
    println!("{:?}", snake_positions);
    tail_positions
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let mut snake_positions: [(i32, i32); 10] = [(0, 0); 10];
            let r = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|line| parse_line(line, &mut snake_positions))
                .flatten()
                .collect::<HashSet<_>>();
            println!("{:?}", r.len())
        }
    }
}
