use std::fs::File;
use std::io::{BufRead, BufReader};

const LINE_LENGTH: usize = 140;

pub(crate) fn day3(path: String) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let char_list = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let total = parse_engine_schematic(&char_list);

    println!("Total of found parts: {}", total);
}

fn parse_engine_schematic(lines: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    for (r, l) in lines.iter().enumerate() {
        let symbol_idxs = l
            .iter()
            .enumerate()
            .filter(|(_, c)| *c != &'.' && !c.is_digit(10))
            .collect::<Vec<(usize, &char)>>();

        let mut found_numbers: Vec<u32> = vec![];
        for (i, _) in symbol_idxs {
            let mut found_for_symbol = get_surrounding_numbers(lines, i, r);
            found_numbers.append( &mut found_for_symbol);
        }

        total += found_numbers.iter().fold(0, |acc, n| acc + n);
    }

    total
}

fn get_surrounding_numbers(lines: &Vec<Vec<char>>, col: usize, row: usize) -> Vec<u32> {
    let mut found_numbers = vec![];
    let row_count = lines.len();

    let l = lines[row][col_idx(col.saturating_sub(1))];
    if l.is_digit(10) {
        found_numbers.push(get_number(lines, row, col_idx(col - 1), Direction::Backwards));
    }

    let r = lines[row][col_idx(col.saturating_add(1))];
    if r.is_digit(10) {
        found_numbers.push(get_number(lines, row, col_idx(col + 1), Direction::Forward));
    }

    let prev_row = row.saturating_sub(1);
    if prev_row < row {
        get_values_from_target_row(lines, col, prev_row, &mut found_numbers);
    }

    let next_row = row.saturating_add(1);
    if next_row <= row_count {
        get_values_from_target_row(lines, col, next_row, &mut found_numbers);
    }

    found_numbers
}

fn get_values_from_target_row(lines: &Vec<Vec<char>>, col: usize, row: usize, found_numbers: &mut Vec<u32>) {
    let l = lines[row][col_idx(col.saturating_sub(1))];
    let m = lines[row][col_idx(col)];
    let r = lines[row][col_idx(col.saturating_add(1))];

    if !m.is_digit(10) {
        if l.is_digit(10) {
            found_numbers.push(get_number(lines, row, col_idx(col - 1), Direction::Backwards));
        }

        if r.is_digit(10) {
            found_numbers.push(get_number(lines, row, col_idx(col + 1), Direction::Forward));
        }
    } else if m.is_digit(10) && (r.is_digit(10) || l.is_digit(10)) {
        found_numbers.push(backtracking_get_number(lines, row, col_idx(col)));
    } else {
        found_numbers.push(m.to_digit(10).unwrap());
    }
}

fn backtracking_get_number(lines: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut curr_col = col;
    let mut curr = lines[row][col];
    while curr.is_digit(10) {
        curr_col = curr_col.saturating_sub(1);
        curr = lines[row][curr_col];
    }

    let num = get_number(lines, row, curr_col.saturating_add(1), Direction::Forward);

    num
}

fn get_number(lines: &Vec<Vec<char>>, row: usize, col: usize, direction: Direction) -> u32 {
    let mut number_str = String::new();
    let mut curr = lines[row][col];
    let mut col = col;
    while curr.is_digit(10) {
        number_str.push(curr);

        let next_col = col_idx(match direction {
            Direction::Forward => col.saturating_add(1),
            Direction::Backwards => col.saturating_sub(1)
        });

        if col == 0 {
            break;
        }

        if next_col == LINE_LENGTH {
            break;
        }

        col = next_col;
        curr = lines[row][col];
    }

    if direction == Direction::Backwards {
        number_str = number_str.chars().rev().collect::<String>();
    }
    number_str.parse::<u32>().unwrap_or(0)
}

#[derive(PartialEq)]
enum Direction {
    Forward,
    Backwards
}

fn col_idx(idx: usize) -> usize {
    if idx > LINE_LENGTH { LINE_LENGTH } else { idx }
}
