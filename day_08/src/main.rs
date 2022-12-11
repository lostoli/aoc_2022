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
fn parse_line(line: String) -> Vec<u32> {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[allow(dead_code)]
fn check_slices(element: u32, slices: Vec<Vec<u32>>) -> bool {
    let result = slices
        .iter()
        .any(|slice| element > *slice.iter().max().unwrap_or(&0) || slice.len() == 0);
    result
}

fn scenic_score(element: u32, slices: Vec<Vec<u32>>) -> usize {
    let p = slices
        .iter()
        .map(|slice| {
            (
                slice.iter().take_while(|&x| x < &element).count(),
                slice.len(),
            )
        })
        .inspect(|x| println!("{:?}", x))
        .map(|(s, l)| if s < l { s + 1 } else { s })
        // .inspect(|x| println!("{:?}", x))
        .product();
    println!("{:?}", (p, slices, element));
    p
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let map = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(parse_line)
                .collect::<Vec<Vec<u32>>>();

            let mut total = 0;
            let mut max_scenic_score = 0;

            for (j, row) in map.iter().enumerate() {
                for (i, &element) in row.iter().enumerate() {
                    if check_slices(
                        element,
                        vec![
                            map[..j]
                                .iter()
                                .map(|row| *row.iter().nth(i).unwrap())
                                .rev()
                                .collect(),
                            map[j + 1..]
                                .iter()
                                .map(|row| *row.iter().nth(i).unwrap())
                                .collect(),
                            map.iter().nth(j).unwrap()[..i]
                                .to_vec()
                                .iter()
                                .rev()
                                .cloned()
                                .collect(),
                            map.iter().nth(j).unwrap()[i + 1..].to_vec(),
                        ],
                    ) {
                        total += 1
                    }
                    let ss = scenic_score(
                        element,
                        vec![
                            map[..j]
                                .iter()
                                .map(|row| *row.iter().nth(i).unwrap())
                                .rev()
                                .collect(),
                            map[j + 1..]
                                .iter()
                                .map(|row| *row.iter().nth(i).unwrap())
                                .collect(),
                            map.iter().nth(j).unwrap()[..i]
                                .to_vec()
                                .iter()
                                .rev()
                                .cloned()
                                .collect(),
                            map.iter().nth(j).unwrap()[i + 1..].to_vec(),
                        ],
                    );
                    if ss > max_scenic_score {
                        max_scenic_score = ss
                    }
                }
            }

            println!("{}", max_scenic_score);
            println!("{:?}", total)
        }
    }
}
