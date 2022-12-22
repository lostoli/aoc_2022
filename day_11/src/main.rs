use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num::integer::lcm;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Eq)]
struct Monkey {
    items: Vec<u64>,
    operation: String,
    test: u64,
    if_true: u32,
    if_false: u32,
    count: u64,
}

impl Monkey {
    fn play(&mut self, lcm: u64) -> (u64, u32) {
        let item = self.items[0];
        self.items = self.items[1..].iter().cloned().collect();
        self.count += 1;
        let new_worry_level = parse_fn(&self.operation)(item) % lcm;
        if new_worry_level % self.test == 0 {
            (new_worry_level, self.if_true)
        } else {
            (new_worry_level, self.if_false)
        }
    }
}

impl Ord for Monkey {
    fn cmp(&self, other: &Monkey) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Monkey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Monkey) -> bool {
        self.count == other.count
    }
}

fn parse_fn(pattern: &String) -> fn(u64) -> u64 {
    match &**pattern {
        "old * 19" => {
            fn operation(x: u64) -> u64 {
                x * 19
            }
            operation
        }
        "old + 8" => {
            fn operation(x: u64) -> u64 {
                x + 8
            }
            operation
        }
        "old * 13" => {
            fn operation(x: u64) -> u64 {
                x * 13
            }
            operation
        }
        "old + 6" => {
            fn operation(x: u64) -> u64 {
                x + 6
            }
            operation
        }
        "old + 5" => {
            fn operation(x: u64) -> u64 {
                x + 5
            }
            operation
        }
        "old * old" => {
            fn operation(x: u64) -> u64 {
                x * x
            }
            operation
        }
        "old + 2" => {
            fn operation(x: u64) -> u64 {
                x + 2
            }
            operation
        }
        "old + 3" => {
            fn operation(x: u64) -> u64 {
                x + 3
            }
            operation
        }
        _ => panic!("Error"),
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let mut monkeys: Vec<Monkey> = vec![];
            for chunk in lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .chunks(7)
            {
                let items = chunk[1]
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let operation = chunk[2].split(" = ").nth(1).unwrap().to_string();
                let test = chunk[3].split(' ').last().unwrap().parse::<u64>().unwrap();
                let if_true = chunk[4].chars().last().unwrap().to_digit(10).unwrap();
                let if_false = chunk[5].chars().last().unwrap().to_digit(10).unwrap();
                let monkey = Monkey {
                    items,
                    operation,
                    test,
                    if_true,
                    if_false,
                    count: 0,
                };
                monkeys.push(monkey);
            }
            let lcm = monkeys.iter().map(|x| x.test).reduce(|acc, item| lcm(acc, item));
            println!("lcm: {:?}", lcm);
            for _ in 0..10000 {
                for i in 0..monkeys.len() {
                    while monkeys[i].items.len() > 0 {
                        let (item, give_to) = monkeys[i].play(lcm.unwrap());
                        monkeys[give_to as usize].items.push(item);
                    }
                }
                // println!("{:?}", monkeys);
            }
            monkeys.sort();
            println!(
                "{:?}",
                monkeys.pop().unwrap().count * monkeys.pop().unwrap().count
            )
        }
    }
}
