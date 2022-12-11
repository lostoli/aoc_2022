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

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(item: &char) -> usize {
    let index = ALPHABET.chars().position(|e| &e == item).unwrap();
    return index + 1;
}

#[allow(dead_code)]
fn parse_line(line: String) -> Vec<usize> {
    let len = line.chars().count();
    let mut first_compartment: HashSet<char> = vec![].into_iter().collect();
    let mut second_compartment: HashSet<char> = vec![].into_iter().collect();
    line.chars().enumerate().for_each(|(i, x)| {
        if i <= len / 2 - 1 {
            first_compartment.insert(x);
        } else {
            second_compartment.insert(x);
        };
    });
    let mut output: Vec<usize> = vec![];
    first_compartment
        .intersection(&second_compartment)
        .for_each(|x| output.push(get_priority(x)));
    return output;
}

fn parse_triple_lines(lines: &[String]) -> usize {
    let intersection = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<HashSet<char>>>()
        .into_iter()
        .reduce(|x, y| x.intersection(&y).cloned().collect());
    if let Some(v) = intersection {
        if let Some(c) = v.iter().next() {
            get_priority(c)
        } else {
            0
        }
    } else {
        0
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let s = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .chunks(3)
                .map(|x| parse_triple_lines(x))
                .sum::<usize>();
            println!("{}", s)
        }
    }
}
