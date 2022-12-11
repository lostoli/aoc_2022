use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
const STACKS: &str = "[V]         [T]         [J]
[Q]         [M] [P]     [Q]     [J]
[W] [B]     [N] [Q]     [C]     [T]
[M] [C]     [F] [N]     [G] [W] [G]
[B] [W] [J] [H] [L]     [R] [B] [C]
[N] [R] [R] [W] [W] [W] [D] [N] [F]
[Z] [Z] [Q] [S] [F] [P] [B] [Q] [L]
[C] [H] [F] [Z] [G] [L] [V] [Z] [H]
 1   2   3   4   5   6   7   8   9";

const TEST_STACKS: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn create_stacks(stack_string: &str) -> HashMap<char, Vec<char>> {
    let lines = stack_string.split("\n").collect::<Vec<&str>>();
    let last_line = lines.last().unwrap();
    let mut output: HashMap<char, Vec<char>> = HashMap::new();
    let mut reversed_lines = stack_string
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    reversed_lines.pop();
    reversed_lines.reverse();

    last_line.chars().enumerate().for_each(|(i, x)| {
        if x != ' ' {
            output.insert(x, vec![]);
            for line in reversed_lines.to_owned() {
                let c = line.chars().nth(i).unwrap_or(' ');
                if c != ' ' {
                    output.get_mut(&x).unwrap().push(c)
                }
            }
        } else {
        }
    });

    output
}

#[allow(dead_code)]
fn move_from_i_to_j(stacks: &mut HashMap<char, Vec<char>>, i: char, j: char) {
    let v = stacks.get_mut(&i).unwrap().pop().unwrap();
    stacks.get_mut(&j).unwrap().push(v)
}

fn move_block_from_i_to_j(
    stacks: &mut HashMap<char, Vec<char>>,
    block_size: u32,
    i: char,
    j: char,
) {
    let mut block: Vec<char> = vec![];
    for _ in 0..block_size {
        block.push(stacks.get_mut(&i).unwrap().pop().unwrap())
    }
    block.reverse();
    for v in block {
        stacks.get_mut(&j).unwrap().push(v)
    }

}

#[allow(dead_code)]
fn parse_line(line: String) -> (u32, char, char) {
    let split_line = line.split(' ').collect::<Vec<&str>>();
    (
        split_line.iter().nth(1).unwrap().parse::<u32>().unwrap(),
        split_line.iter().nth(3).unwrap().chars().nth(0).unwrap(),
        split_line.iter().nth(5).unwrap().chars().nth(0).unwrap(),
    )
}

fn print_message(stacks: HashMap<char, Vec<char>>) {
    let message: Vec<&char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        .iter()
        .map(|c| {
            if stacks.get(c).is_some() {
                stacks.get(c).unwrap().last().unwrap()
            } else {
                &'x'
            }
        })
        .collect::<Vec<&char>>();
    println!("{}", message.into_iter().collect::<String>())
}

fn main() {
    let mut stacks = create_stacks(STACKS);
    println!("{:?}", stacks);
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let moves = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|x| parse_line(x));
            for m in moves {
                move_block_from_i_to_j(&mut stacks, m.0, m.1, m.2)
            }
            println!("{:?}", stacks);
            print_message(stacks)
        }
    }
}
