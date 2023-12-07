use std::fs::File;
use std::io::{BufReader, Read};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1, u32};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

pub fn day6(path: String) {
    let file = File::open(path).unwrap();
    let mut buffer = BufReader::new(file);
    let mut i = String::new();
    _ = buffer.read_to_string(&mut i);

    let (_, race_data) = parse_race_data(&i).unwrap();
    let win_variants = race_data.iter().fold(1, |acc, r| acc * r.combinations_to_beat_record());

    println!("Win variants: {}", win_variants);

    let (_, (time, record)) = parse_race_data_combined(&i).unwrap();
    let combos_pt2 =  (0..time).collect::<Vec<u64>>().iter().fold(0, |acc, x| {
        let distance_travelled = (time - x) * x;
        if distance_travelled >= record {
            return acc + 1
        }
        return acc
    });

    println!("Win variants pt2: {}", combos_pt2);
}

struct Race {
    time: u32,
    record: u32
}

impl Race {
    pub fn new(time: u32, record: u32) -> Self {
        Self {
            time,
            record
        }
    }

    pub fn combinations_to_beat_record(&self) -> u32 {
        (0..self.time).collect::<Vec<u32>>().iter().fold(0, |acc, x| {
            let distance_travelled = (self.time - x) * x;
            if distance_travelled >= self.record {
                return acc + 1
            }
            return acc
        })
    }
}

fn parse_race_data(i: &str) -> IResult<&str, Vec<Race>> {
    let (i, times) = tuple((tag("Time:"), space1, separated_list1(space1, u32), newline))(i)
        .map(|(i, (_, _, times, _))| (i, times))?;

    let (i, records) = tuple((tag("Distance:"), space1, separated_list1(space1, u32)))(i)
        .map(|(i, (_, _, records))| (i, records))?;

    Ok((i, times.into_iter().zip(records).map(|(t, r)| Race::new(t, r)).collect()))
}

fn parse_race_data_combined(i: &str) -> IResult<&str, (u64, u64)> {
    let (i, times) = tuple((tag("Time:"), space1, separated_list1(space1, digit1), newline))(i)
        .map(|(i, (_, _, times, _))| (i, times))?;

    let (i, records) = tuple((tag("Distance:"), space1, separated_list1(space1, digit1)))(i)
        .map(|(i, (_, _, records))| (i, records))?;

    let time = times.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x)).parse::<u64>().unwrap();
    let distance = records.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x)).parse::<u64>().unwrap();

    Ok((i, (time, distance)))
}