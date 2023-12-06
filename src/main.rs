mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
    println!(".: 🎄 Advent of Code 2023 🎄 :.");
    let args = Args::parse();

    match args.day {
        1 => day1::day1(args.input_path),
        2 => day2::day2(args.input_path),
        3 => day3::day3(args.input_path),
        4 => day4::day4(args.input_path),
        5 => day5::day5(args.input_path),
        n => { println!("Implementation for provided day (day {n}) not found. ")}
    };
}
