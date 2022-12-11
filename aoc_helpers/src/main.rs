use clap::Parser;
use reqwest; // 0.10.0
use std::env;
use std::fs;
use std::process::Command;
use tokio; // 0.2.6

#[derive(Parser, Default, Debug)]
/// Set up a new Rust project for an AOC problem.
#[clap(author = "Oliver Sargent", long_about=None)]
struct Arguments {
    /// day of problem to set up
    #[clap(short, long)]
    day: u8,
    /// year of problem to set up
    #[clap(short, long, default_value_t = 2022)]
    year: u32,
}

const TEMPLATE: &str = r#"use std::env;
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
"#;

#[tokio::main]
async fn get_input(year: u32, day: u8) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let cookie = env::var("AOC_SESSION_COOKIE").unwrap_or("none".to_string());
    let res = client
        .get(format!(
            "https://adventofcode.com/{year}/day/{day}/input",
            year = year,
            day = day
        ))
        .header("Cookie", cookie)
        .send()
        .await?
        .text()
        .await;

    fs::write(
        format!("day_{day}/input", day = pad_day_with_zero(day)),
        res.unwrap(),
    )
    .expect("Unable to write file");

    Ok(())
}

fn pad_day_with_zero(day: u8) -> String {
    if day < 10 {
        format!("0{day}", day = day)
    } else {
        format!("{day}", day = day)
    }
}

fn copy_template_to_main(day: u8) {
    fs::write(
        format!("day_{day}/src/main.rs", day = pad_day_with_zero(day)),
        TEMPLATE,
    )
    .expect("Unable to write template");
}

fn set_up_new_project(day: u8) {
    let mut cargo_new = Command::new("cargo");
    cargo_new
        .arg("new")
        .arg(format!("day_{day}", day = pad_day_with_zero(day)));
    let output = cargo_new.output().expect("failed to execute process");
    println!("{:?}", output);
}

fn main() {
    let args = Arguments::parse();
    set_up_new_project(args.day);
    copy_template_to_main(args.day);
    get_input(args.year, args.day).expect("Could not get input!");
    println!("{}", "done!")
}
