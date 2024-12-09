use clap::Parser;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn part_one(raw: &String) -> i32 {
    let regex = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)").unwrap();
    regex.captures_iter(raw.as_str()).fold(0, |mut acc, curr| {
        acc += curr["x"].parse::<i32>().unwrap() * curr["y"].parse::<i32>().unwrap();
        acc
    })
}

fn part_two(raw: &String) -> i32 {
    let regex =
        Regex::new(r"(mul\((?<x>\d+),(?<y>\d+)\)|(?<do>do\(\))|(?<dont>don't\(\)))").unwrap();
    let mut enabled = true;
    regex.captures_iter(raw.as_str()).fold(0, |mut acc, curr| {
        if let Some(_) = curr.name("do") {
            enabled = true;
        } else if let Some(_) = curr.name("dont") {
            enabled = false;
        } else if enabled {
            acc += curr["x"].parse::<i32>().unwrap() * curr["y"].parse::<i32>().unwrap();
        }
        acc
    })
}

fn main() {
    let args = Args::parse();
    let input = read_to_string(args.input).unwrap();
    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}
