use serde_json::Value::Array;
use serde_json::Value::Number;
use serde_json::{Result, Value};
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_input(data: &str) -> Result<Value> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;
    Ok(v)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Debug)]
enum Status {
    Correct,
    Incorrect,
    Continue,
}

fn status_to_ordering(status: Status) -> Ordering {
    match status {
        Status::Correct => Ordering::Less,
        Status::Continue => Ordering::Equal,
        Status::Incorrect => Ordering::Greater,
    }
}

fn compare(value_0: &Value, value_1: &Value) -> Status {
    match (value_0, value_1) {
        (Number(x_0), Number(x_1)) => {
            if x_0.as_i64() < x_1.as_i64() {
                Status::Correct
            } else if x_0.as_i64() == x_1.as_i64() {
                Status::Continue
            } else {
                Status::Incorrect
            }
        }
        (Array(_), Number(_)) => compare(value_0, &Array(vec![value_1.clone()])),
        (Number(_), Array(_)) => compare(&Array(vec![value_0.clone()]), value_1),
        (Array(x_0), Array(x_1)) => {
            if x_0.len() > 0 && x_1.len() == 0 {
                Status::Incorrect
            } else if x_0.len() == 0 && x_1.len() > 0 {
                Status::Correct
            } else {
                for i in 0..x_0.len().min(x_1.len()) {
                    let status = compare(&value_0[i], &value_1[i]);
                    if status != Status::Continue {
                        return status;
                    }
                }
                if x_0.len() < x_1.len() {
                    Status::Correct
                } else if x_0.len() == x_1.len() {
                    Status::Continue
                } else {
                    Status::Incorrect
                }
            }
        }
        _ => panic!("Error!!!!"),
    }
}

fn decoder_packets() -> Vec<Value> {
    vec![parse_input("[[2]]").unwrap(), parse_input("[[6]]").unwrap()]
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            // let r = lines
            //     .filter_map(|x| x.ok())
            //     .collect::<Vec<String>>()
            //     .into_iter()
            //     .map(|line| parse_input(&line))
            //     .filter_map(|x| x.ok())
            //     .collect::<Vec<Value>>()
            //     .chunks(2)
            //     .enumerate()
            //     .filter(|(_, chunk)| compare(&chunk[0], &chunk[1]) == Status::Correct)
            //     .map(|(i, _)| i + 1)
            //     .sum::<usize>();
            let mut p = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|line| parse_input(&line))
                .filter_map(|x| x.ok())
                .collect::<Vec<Value>>();

            p.append(&mut decoder_packets());
            p.sort_by(|v_0, v_1| status_to_ordering(compare(v_0, v_1)));
            let a = p.iter().position(|signal| signal.to_string() == "[[2]]").unwrap();
            let b = p.iter().position(|signal| signal.to_string() == "[[6]]").unwrap();
            println!("{:?}", (a + 1) * (b + 1))
        }
    }
}
