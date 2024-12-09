use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn parse_input(filename: &Path) -> Vec<Vec<i32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let split = line.split_whitespace();
            split.map(|num| num.parse::<i32>().unwrap()).collect()
        })
        .collect()
}

trait Pair {
    fn safe(&self) -> bool;
}

struct Report {
    content: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        true
    }
}

impl From<&str> for Report {
    fn from(s: &str) -> Report {
        let split = s.split_whitespace();
        Report {
            content: split.map(|num| num.parse::<i32>().unwrap()).collect(),
        }
    }
}

fn part_one(v: &Vec<Vec<i32>>) -> usize {
    v.into_iter()
        .map(|line| {
            let (mut was_inc, mut was_dec, mut eps) = (false, false, 0);
            let w = line.windows(2);
            let mut res = true;
            for pair in w {
                eps = pair[0].abs_diff(pair[1]);
                if (eps < 1 || eps > 3)
                    || (was_inc && pair[0] > pair[1])
                    || (was_dec && pair[0] < pair[1])
                {
                    res = false;
                    break;
                }
                (was_inc, was_dec) = (pair[0] < pair[1], pair[0] > pair[1]);
            }
            res
        })
        .filter(|b| *b)
        .count()
}

fn part_two(v: &Vec<Vec<i32>>) -> usize {
    v.into_iter()
        .map(|line| {
            let (mut was_inc, mut was_dec, mut eps) = (false, false, 0);
            let w = line.windows(2);
            let mut res = true;
            for pair in w {
                eps = pair[0].abs_diff(pair[1]);
                if (eps < 1 || eps > 3)
                    || (was_inc && pair[0] > pair[1])
                    || (was_dec && pair[0] < pair[1])
                {
                    res = false;
                    break;
                }
                (was_inc, was_dec) = (pair[0] < pair[1], pair[0] > pair[1]);
            }
            res
        })
        .filter(|b| *b)
        .count()
}

fn main() {
    let args = Args::parse();
    let parsed = parse_input(Path::new(&args.input));
    println!("p1 {}", part_one(&parsed));
    println!("p2 {}", part_two(&parsed));
}
