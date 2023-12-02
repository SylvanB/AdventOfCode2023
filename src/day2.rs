use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day2(path: String) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let given_set = HashMap::from([
        (Colour::Red, 12),
        (Colour::Green, 13),
        (Colour::Blue, 14),
    ]);

    let mut total_of_ids = 0;
    let mut power_of_minimums = 0;
    for lines in buffer.lines() {
        let l = lines.unwrap();
        let game_data = GameData::new_from_line(l).unwrap();
        let x = is_game_valid_for_given_set(&game_data, &given_set);
        total_of_ids += x.unwrap_or(0);

        let y = get_power_of_minimum_required(&game_data);
        power_of_minimums += y;
    }

    println!("Total of possible game Ids: {}", total_of_ids);
    println!("Power of minimums: {}", power_of_minimums);

}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Colour {
    Red,
    Green,
    Blue,
    None
}

impl From<&str> for Colour {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_ref() {
            "red" => Colour::Red,
            "green" => Colour::Green,
            "blue" => Colour::Blue,
            _ => Colour::None
        }
    }
}

struct GameData {
    pub id: u32,
    pub revealed: Vec<Vec<BallData>>
}

impl GameData {
    pub fn new_from_raw_data(id: u32, revealed:Vec<&str>) -> Self {
        let raw_ball_data: Vec<Vec<&str>> = revealed.iter().map(|rev| rev.split(", ").collect::<Vec<&str>>()).collect();
        let ball_data: Vec<Vec<BallData>> = raw_ball_data.iter().map(|raw| raw.iter().map(|data| BallData::from(*data)).collect()).collect();

        Self {
            id,
            revealed: ball_data
        }
    }

    pub fn new_from_line (line: String) -> Option<Self> {
        let parts: Vec<&str> = line.split(&[':', ';'][..]).collect();

        let id_str: &str = parts
            .first()?
            .split(" ")
            .skip(1).
            take(1)
            .collect::<Vec<&str>>()
            .first()?;

        let id = id_str.parse::<u32>().ok()?;

        let revealed_sets_parts = &parts[1..];
        Some(GameData::new_from_raw_data(id, revealed_sets_parts.to_vec()))
    }

    fn get_max_colours_seen(&self) -> HashMap<Colour, u32> {
        self.revealed.iter().fold(HashMap::new(), |mut acc: HashMap<Colour, u32>, data: &Vec<BallData>| {
            for ball_data in data {
                if !acc.contains_key(&ball_data.colour) {
                    acc.insert(ball_data.colour.clone(), ball_data.count);
                } else {
                    let val = acc.get_mut(&ball_data.colour).unwrap();
                    if *val < ball_data.count {
                        *val = ball_data.count
                    }
                }
            }

            acc
        })
    }

    pub fn is_possible(&self, given_set: &HashMap<Colour, u32>) -> Option<bool> {
        let max_seen = self.get_max_colours_seen();
        let is_possible = given_set.get(&Colour::Red)? >= max_seen.get(&Colour::Red)?
            && given_set.get(&Colour::Blue)? >= max_seen.get(&Colour::Blue)?
            && given_set.get(&Colour::Green)? >= max_seen.get(&Colour::Green)?;

        Some(is_possible)
    }

}

struct BallData {
    colour: Colour,
    count: u32,
}

impl BallData {
    pub fn new(colour: Colour, count: u32) -> Self {
        Self {
            colour,
            count
        }
    }
}

impl From<&str> for BallData {
    fn from(value: &str) -> Self {
        let split = value.trim().split(" ").collect::<Vec<&str>>();
        let colour = Colour::from(split[1]);
        let count = str::parse(split[0]).unwrap_or(0);

        BallData::new(colour, count)
    }
}

pub(crate) fn is_game_valid_for_given_set(game_data: &GameData, given_set: &HashMap<Colour, u32>) -> Option<u32> {
    let is_possible = game_data.is_possible(given_set)?;

    if is_possible { Some(game_data.id) } else { None }
}

fn get_power_of_minimum_required(game_data: &GameData) -> u32 {
    let minimum_required = game_data.get_max_colours_seen();
    minimum_required.iter().fold(1, |acc, (colour, count)| acc * count)
}
