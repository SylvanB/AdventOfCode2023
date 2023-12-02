use std::fs::File;
use std::io::{BufRead, BufReader};
use aho_corasick::AhoCorasick;

pub (crate) fn day1(path: String) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let patterns = [
        "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    let ac = AhoCorasick::new(patterns).unwrap();
    let total = buffer.lines().fold(0, |acc, l| acc + parse_calibration_string(&ac, &l.unwrap()));
    println!("Sum of calibration values: {}", total);
}

pub(crate) fn parse_calibration_string(ac: &AhoCorasick, input: &str ) -> u32 {
    let matches: Vec<u32> = ac.find_overlapping_iter(input).map(|mat| {
        let pattern_idx = mat.pattern().as_u32();
        match pattern_idx {
            0..=8 => pattern_idx + 1,
            9..=17 => pattern_idx - 8,
            _ => 0
        }
    }).collect();

    let num = match matches.len() {
        0 => (0, 0),
        1 => (matches[0], matches[0]),
        n => (matches[0], matches[n - 1])
    };

    num.0 * 10 + num.1
}
