use std::collections::HashMap;
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

fn get_size_dir(lines: &Vec<String>, dir: usize, sizes: &mut HashMap<usize, u32>) -> u32 {
    if let Some(size) = sizes.get(&dir) {
        return *size;
    }
    // let start_index = lines
    //     .into_iter()
    //     .position(|line| line == &format!("$ cd {}", dir))
    //     .unwrap();
    let start_index = dir;
    if lines.into_iter().nth(start_index + 1).unwrap() == &format!("$ ls") {
        println!("hello");
        let size = lines[start_index + 2..]
            .into_iter()
            .enumerate()
            .take_while(|(_, line)| line.chars().nth(0).unwrap() != '$')
            .map(|(dir, line)| {
                let parts = &line.split(' ').into_iter().collect::<Vec<&str>>()[..2];
                if parts[0] != "dir" {
                    parts[0].parse::<u32>().unwrap()
                } else {
                    get_size_dir(lines, dir + start_index + 2, sizes)
                }
            })
            .sum();
        sizes.insert(dir, size);
        println!("{}", size);
        size
    } else {
        0
    }
}

fn list_dirs(lines: &Vec<String>) -> Vec<usize> {
    lines
        .into_iter()
        .enumerate()
        .filter(|(_, line)| {
            line.split(' ').into_iter().nth(1).unwrap() == "cd"
                && line.split(' ').nth(0).unwrap() != ".."
        })
        .map(|x| x.0)
        .collect()
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let vec_lines = lines.filter_map(|x| x.ok()).collect::<Vec<String>>();

            let mut files: Vec<(Vec<String>, String)> = vec![];
            let mut dirs: Vec<Vec<String>> = vec![vec!["/".into()]];
            let mut path: Vec<String> = vec![];

            for line in vec_lines {
                if let Some(("$", p)) = line.split_once(" cd ") {
                    match p {
                        ".." => {
                            path.pop();
                        }
                        _ => path.push(p.to_string()),
                    }
                } else if let Some(("$ ", _)) = line.split_once("ls") {
                } else if let Some(("dir", d)) = line.split_once(" ") {
                    let dir_path = [path.clone(), vec![d.to_string()]].concat();
                    dirs.push(dir_path)
                } else if let Some((size, f)) = line.split_once(" ") {
                    let file_path = [path.clone(), vec![f.to_string()]].concat();
                    files.push((file_path, size.to_string()))
                }
            }
            // println!("{:?}", files);
            // println!("{:?}", dirs);

            let mut sum = 0;

            let total_size = files
                .iter()
                .map(|file| file.1.parse::<u32>().unwrap())
                .sum::<u32>();

            for d in dirs.clone() {
                println!("{:?}", d);
                let dir_size = files
                    .iter()
                    .filter(|file| file.0.iter().zip(d.clone()).all(|(ff, dd)| *ff == dd))
                    .map(|file| file.1.parse::<u32>().unwrap())
                    .sum::<u32>();
                println!("{:?}", dir_size);
                if dir_size < 100000 {
                    sum += dir_size
                }
            }
            println!("{:?}", sum);
            println!("{:?}", total_size);

            let a = dirs
                .into_iter()
                .filter(|dir| {
                    files
                        .iter()
                        .filter(|file| file.0.iter().zip(dir.clone()).all(|(ff, dd)| *ff == dd))
                        .map(|file| file.1.parse::<u32>().unwrap())
                        .sum::<u32>()
                        > total_size - 40000000
                })
                .map(|dir| {
                    files
                        .iter()
                        .filter(|file| file.0.iter().zip(dir.clone()).all(|(ff, dd)| *ff == dd))
                        .map(|file| file.1.parse::<u32>().unwrap())
                        .sum::<u32>()
                }).min();

            println!("{:?}", a);

            // let size = get_size_dir(&vec_lines, String::from("a"));
            // println!("{}", answer);
        }
    }
}
