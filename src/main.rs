use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use regex::Regex;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, required=true)]
    day: u8,

    #[arg(short, long, required=true)]
    input_path: String,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1(args.input_path),
        _ => {}
    };
}

fn day1(path: String) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let num_regex = Regex::new("\\d").unwrap();

    let total = buffer.lines()
        .fold(0, |acc, x| {
            let line = x.unwrap();
            let nums: Vec<char> = line
                .chars()
                .filter(|char| char.is_digit(10))
                .collect();

            let num_str = match nums.len() {
                0 => { return acc; },
                1 => format!("{}{}", &nums[0], &nums[0]),
                n => format!("{}{}", &nums[0], &nums[n-1])
            };

            let num = num_str.parse::<u32>().unwrap();
            acc + num
        });

    println!("Sum of calibration values: {}", total);
}

