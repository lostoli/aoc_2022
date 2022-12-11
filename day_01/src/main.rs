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

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let mut m = 0;
            let mut n: [i32; 3] = [1, 2, 3];
            for line in lines {
                if let Ok(string_line) = line {
                    let parsed_line = string_line.parse::<i32>();
                    if parsed_line.is_ok() {
                        m += parsed_line.unwrap()
                    } else {
                        let min = n.iter().min().unwrap();
                        let index = n.iter().position(|e| e == min).unwrap();
                        if m > *min {
                            n[index] = m;
                        }
                        m = 0
                    }
                }
            }
            let min = n.iter().min().unwrap();
            let index = n.iter().position(|e| e == min).unwrap();
            if m > *min {
                n[index] = m;
            }
            println!("{}", n.iter().sum::<i32>());
        }
    }
}
