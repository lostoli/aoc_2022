use pathfinding::prelude::bfs;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(u32, u32);

impl Pos {
    fn successors(&self, board: &Vec<Vec<char>>) -> Vec<Pos> {
        let &Pos(x, y) = self;

        vec![
            Pos(x + 1, y),
            Pos(if x > 0 { x - 1 } else { 0 }, y),
            Pos(x, if y > 0 { y - 1 } else { 0 }),
            Pos(x, y + 1),
        ]
        .iter()
        .filter(|p| p.is_on_board(board) && p.lookup_height(board) <= self.lookup_height(board) + 1)
        .cloned()
        .collect()
    }

    fn lookup_height(&self, board: &Vec<Vec<char>>) -> u32 {
        let &Pos(x, y) = self;
        lookup_height(board[x as usize][y as usize])
    }

    fn is_on_board(&self, board: &Vec<Vec<char>>) -> bool {
        let &Pos(x, y) = self;
        x < (board.len() as u32) && y < (board[0].len() as u32)
    }
}

static ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

fn lookup_height(c: char) -> u32 {
    match c {
        'E' => 25,
        'S' => 0,
        _ => ALPHA.chars().position(|x| x == c).unwrap() as u32,
    }
}

fn lookup_goal(board: &Vec<Vec<char>>, c: char) -> Pos {
    let height = board.len();
    let width = board[0].len();
    let mut x = 0;
    let mut y = 0;
    for i in 0..width {
        for j in 0..height {
            if board[j][i] == c {
                (x, y) = (j, i)
            }
        }
    }
    Pos(x as u32, y as u32)
}

fn lookup_low_points(board: &Vec<Vec<char>>) -> Vec<Pos> {
    let height = board.len();
    let width = board[0].len();
    let mut low_points = vec![];
    for i in 0..width {
        for j in 0..height {
            if board[j][i] == 'a' || board[j][i] == 'S' {
                low_points.push(Pos(j as u32, i as u32))
            }
        }
    }
    low_points
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

#[allow(dead_code)]
fn parse_line(line: String) -> Vec<char> {
    line.chars().collect()
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let map: Vec<Vec<char>> = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|line| parse_line(line))
                .collect();
            let start = lookup_goal(&map, 'S');
            let end = lookup_goal(&map, 'E');
            let low_points = lookup_low_points(&map);
            let result = bfs(&start, |p| p.successors(&map), |p| *p == end);
            println!("{:?}", result.unwrap().len() - 1);

            let part_2 = low_points
                .into_iter()
                .map(|lp| bfs(&lp, |p| p.successors(&map), |p| *p == end))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap().len() - 1)
                .min()
                .unwrap();
            println!("{}", part_2)
        }
    }
}
