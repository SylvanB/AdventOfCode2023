use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day4(path: String) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let split_lines = lines
        .iter()
        .map(|l| l.split(&[':', '|']).collect::<Vec<&str>>())

        .collect::<Vec<Vec<&str>>>();
    let winners_for_card = split_lines
        .iter()
        .map(|l| get_winning_numbers(l))
        .collect::<Vec<Vec<u32>>>();

    let total_points = winners_for_card
        .iter()
        .fold(0, |acc, winners| acc + calculate_points_for_card(winners));

    let mut card_count = vec![0_u32; winners_for_card.len()];
    for (idx, winners) in winners_for_card.iter().enumerate() {
        card_count[idx] += 1;
        let is_winner = winners.len() > 0;
        if is_winner {
            for _count in 0..card_count[idx] {
                for n in 0..winners.len() {
                    card_count[idx + n + 1] += 1;
                }
            }
        }
    }

    let total_card_count = card_count.into_iter().fold(0, |acc, n| acc + n);
    println!("Total Points: {}", total_points);
    println!("Total Count of Cards: {}", total_card_count);
}

fn get_winning_numbers(split_line: &Vec<&str>) -> Vec<u32> {
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
        .map(|x| x.parse::<u32>().unwrap().to_owned())
        .collect::<Vec<u32>>();

    mine.into_iter().filter(|x| winning.contains(x)).collect()
}

fn calculate_points_for_card(winners: &Vec<u32>) -> u32 {
    match winners.len() {
        0 => 0,
        1 => 1,
        n => {
            let exp = n.saturating_sub(1);
            2_u32.pow(exp as u32)
        }
    }
}