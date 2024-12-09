use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn sorted_nums_from_file(filename: &Path) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(filename).expect("couldn't read file.");
    let reader = BufReader::new(file);
    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];
    reader.lines().for_each(|l| {
        let line = l.unwrap();
        let mut split = line.split_whitespace();
        if let (Some(first), Some(second)) = (split.next(), split.next()) {
            v1.push(first.parse::<i32>().unwrap());
            v2.push(second.parse::<i32>().unwrap());
        }
    });
    v1.sort();
    v2.sort();
    (v1, v2)
}

fn part_one(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v1.into_iter()
        .zip(v2.into_iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>()
}

fn part_two(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut counts = v2.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    v1.into_iter()
        .map(|num| &*counts.entry(num).or_insert(0) * num)
        .sum::<i32>()
}

fn main() {
    let args = Args::parse();
    let fp = Path::new(&args.input);
    let (v1, v2) = sorted_nums_from_file(fp);
    println!("p1 {}", part_one(&v1, &v2));
    println!("p2 {}", part_two(&v1, &v2));
}
