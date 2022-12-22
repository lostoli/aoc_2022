use std::collections::HashMap;
use std::env;
use std::fmt;
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

type OpName = [char; 4];

#[derive(Clone, Copy, Debug)]
enum OpType {
    MINUS,
    PLUS,
    MUL,
    DIV,
}

#[derive(Clone, Copy, Debug)]
struct Op {
    op_type: OpType,
    operands: (OpName, OpName),
}

#[derive(Clone, Copy, Debug)]
struct OpOrNum {
    num: Option<f64>,
    op: Option<Op>,
}

fn to_opname(name: &str) -> OpName {
    name.chars().collect::<Vec<char>>()[..4].try_into().unwrap()
}

fn parse_line(line: String) -> (OpName, OpOrNum) {
    if let Some((name, part)) = line.split_once(": ") {
        let x: OpName = to_opname(name);
        if let Some((name0, name1)) = part.split_once(" + ") {
            (
                x,
                OpOrNum {
                    op: Some(Op {
                        op_type: OpType::PLUS,
                        operands: (to_opname(name0), to_opname(name1)),
                    }),
                    num: None,
                },
            )
        } else if let Some((name0, name1)) = part.split_once(" - ") {
            (
                x,
                OpOrNum {
                    op: Some(Op {
                        op_type: OpType::MINUS,
                        operands: (to_opname(name0), to_opname(name1)),
                    }),
                    num: None,
                },
            )
        } else if let Some((name0, name1)) = part.split_once(" * ") {
            (
                x,
                OpOrNum {
                    op: Some(Op {
                        op_type: OpType::MUL,
                        operands: (to_opname(name0), to_opname(name1)),
                    }),
                    num: None,
                },
            )
        } else if let Some((name0, name1)) = part.split_once(" / ") {
            (
                x,
                OpOrNum {
                    op: Some(Op {
                        op_type: OpType::DIV,
                        operands: (to_opname(name0), to_opname(name1)),
                    }),
                    num: None,
                },
            )
        } else if let Ok(n) = part.parse::<f64>() {
            (
                x,
                OpOrNum {
                    num: Some(n),
                    op: None,
                },
            )
        } else {
            panic!("This shouldn't happen!")
        }
    } else {
        panic!("Neither should this!")
    }
}

fn expand(name: OpName, data: &HashMap<OpName, OpOrNum>) -> f64 {
    let op_or_num = data.get(&name).unwrap();
    if let Some(num) = op_or_num.num {
        num
    } else if let Some(op) = op_or_num.op {
        match op.op_type {
            OpType::MUL => expand(op.operands.0, data) * expand(op.operands.1, data),
            OpType::DIV => expand(op.operands.0, data) / expand(op.operands.1, data),
            OpType::MINUS => expand(op.operands.0, data) - expand(op.operands.1, data),
            OpType::PLUS => expand(op.operands.0, data) + expand(op.operands.1, data),
        }
    } else {
        panic!("This shouldn't happen")
    }
}

fn binary_search(
    humn: OpName,
    m0: OpName,
    target: f64,
    min: f64,
    max: f64,
    data: &mut HashMap<OpName, OpOrNum>,
) -> f64 {
    println!("{:?}", (min, max));
    let mid_point: f64 = (min + max) / 2.0;
    *data.get_mut(&humn).unwrap() = OpOrNum {
        num: Some(mid_point),
        op: None,
    };
    let e = expand(m0, &data);
    if e == target {
        mid_point
    } else if e > target {
        binary_search(humn, m0, target, min, mid_point, data)
    } else {
        binary_search(humn, m0, target, mid_point, max, data)
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(arg1) {
            // Consumes the iterator, returns an (Optional) String
            let r: HashMap<_, _> = lines
                .filter_map(|x| x.ok())
                .collect::<Vec<String>>()
                .into_iter()
                .map(parse_line)
                .collect();

            let root = to_opname("root");
            let part_1 = expand(root, &r);
            println!("part_1: {:?}", part_1);

            let mut data = r.clone();

            let humn = to_opname("humn");

            let (m0, m1) = r.get(&root).unwrap().op.unwrap().operands;

            let target = expand(m1, &data);

            let part_2 = binary_search(humn, m0, target, 0.0, 10000000000000.0, &mut data);

            println!("part_2: {:?}", part_2);

            println!("target: {}", target);

            assert_eq!(data.get(&humn).unwrap().num.unwrap(), part_2);

            assert_eq!(expand(m0, &data), target)
        }
    }
}
