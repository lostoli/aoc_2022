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
fn parse_line(line: String, register_values: &mut Vec<(i32, i32)>) {
    match &line[..4] {
        "noop" => register_values.push(*register_values.last().unwrap()),
        "addx" => {
            register_values.push(*register_values.last().unwrap());
            if let Some(("addx", num)) = line.split_once(' ') {
                let int_num = num.parse::<i32>().unwrap();
                let last_register_value = register_values.last().unwrap().clone();
                register_values.push((last_register_value.1, last_register_value.1 + int_num))
            }
        }
        _ => panic!("Error"),
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let mut register_values: Vec<(i32, i32)> = vec![(1, 1)];
            lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .for_each(|line| parse_line(line, &mut register_values));
            println!(
                "{:?}",
                vec![20, 60, 100, 140, 180, 220]
                    .into_iter()
                    .map(|x| (x as i32) * register_values[x - 1].1)
                    .sum::<i32>()
            );

            register_values.chunks(40).for_each(|chunk| {
                println!(
                    "{}",
                    chunk
                        .into_iter()
                        .enumerate()
                        .map(|(i, (before, after))| {
                            if vec![after - 1, *after, after + 1].contains(&(i as i32)) {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .collect::<String>()
                )
            });
        }
    }
}
