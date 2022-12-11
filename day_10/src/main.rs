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
fn parse_line(line: String) {
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let r = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
            println!("{}", r)
        }
    }
}
