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
fn parse_line(line: String) -> u32 {
    match &*line {
        "A X" => 4, // Rock draw
        "A Y" => 8, // Paper win
        "A Z" => 3, // Scissors loose
        "B X" => 1, // Rock loose
        "B Y" => 5, // Paper draw
        "B Z" => 9, // Scissors win
        "C X" => 7, // Rock win
        "C Y" => 2, // Paper loose
        "C Z" => 6, // Scissors draw
        _ => 0,
    }
}

fn parse_line_2(line: String) -> u32 {
    match &*line {
        "A X" => 3, // Scissors loose
        "A Y" => 4, // Rock draw 
        "A Z" => 8, // Paper win
        "B X" => 1, // Rock loose
        "B Y" => 5, // Paper draw
        "B Z" => 9, // Scissors win
        "C X" => 2, // Paper loose
        "C Y" => 6, // Scissors draw
        "C Z" => 7, // Rock win
        _ => 0,
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let s = lines
                .map(|x| if x.is_ok() { parse_line_2(x.unwrap()) } else { 0 })
                .sum::<u32>();
            println!("{}", s)
        }
    }
}
