use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day4(path: String) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let split_lines = lines.iter().map(|l| l.split(&[':', '|']).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let total_points = split_lines.iter().fold(0, |acc, x| acc + parse_winning_numbers(x));

    println!("{}", total_points);
}

fn parse_winning_numbers(split_line: &Vec<&str>) -> u32{
    let winning = split_line[1]
        .trim()
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mine = split_line[2]
        .trim()
        .split(" ")
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let winners = mine.iter().filter(|x| winning.contains(x)).collect::<Vec<&u32>>();

    match winners.len() {
        0 => 0,
        1 => 1,
        n => {
            let exp = n.saturating_sub(1);
            2_u32.pow(exp as u32)
        }
    }

}