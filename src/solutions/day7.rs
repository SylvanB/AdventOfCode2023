use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::str::FromStr;
use std::sync::mpsc::TrySendError::Full;
use itertools::Itertools;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{space1, u32};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use strum_macros::EnumString;
use crate::solutions::day7::HandType::{FiveOfAKind, HighCard, ThreeOfAKind, TwoPair};

pub fn day7(path: String) {
    let file = File::open(path).unwrap();
    let mut buffer = BufReader::new(file);
    let mut i = String::new();
    _ = buffer.read_to_string(&mut i);

    let (i, mut hands) = parse_hands(&i).unwrap();
    hands.sort();
    let total_winnings = hands.iter().enumerate().fold(0, |acc, (i,h)| acc + (h.bid * i as u32));

    println!("{}", total_winnings);
}

/*
32T3K (3*5 + 2*4 + 10*3 + 3*2 + 13*1) = 72
T55J5 (10*5 + 5*4 + 5*3 + 11*2 + 5*1) = 112
KK677 (13*5 + 12*4 + 6*3 + 7*2 + 7*1) = 152
KTJJT (13*5 + 10*4 + 11*3 + 11*2 + 10*1) = 170
QQQJA (12*5 + 12*4 + 12*3 + 11*2 + 14*1) = 180

1. 32T3K = 72
2. KTJJT = 170
3. KK677 = 152
4. T55J5 = 112
5. QQQJA = 180
*/

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString)]
#[repr(u32)]
enum Card {
    #[strum(serialize = "1")]
    One = 1,
    #[strum(serialize = "2")]
    Two = 2,
    #[strum(serialize = "3")]
    Three = 3,
    #[strum(serialize = "4")]
    Four = 4,
    #[strum(serialize = "5")]
    Five = 5,
    #[strum(serialize = "6")]
    Six = 6,
    #[strum(serialize = "7")]
    Seven = 7,
    #[strum(serialize = "8")]
    Eight = 8,
    #[strum(serialize = "9")]
    Nine = 9,
    #[strum(serialize = "T")]
    Ten = 10,
    #[strum(serialize = "J")]
    Jack = 11,
    #[strum(serialize = "Q")]
    Queen = 12,
    #[strum(serialize = "K")]
    King = 13,
    #[strum(serialize = "A")]
    Ace = 14
}

#[derive(Eq,PartialEq, Ord, PartialOrd, Hash)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Eq)]
struct Hand {
    card_value: u32,
    // cards: HashMap<Card, u32>,
    bid: u32,
    hand_type: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.card_value == other.card_value
    }

}

impl Hash for Hand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hand_type.hash(state);
        self.card_value.hash(state);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.hand_type, &self.card_value).cmp(&(&other.hand_type, &other.card_value))
    }
}

impl Hand {
    pub fn new(cards: &str, bid: u32) -> Self {
        let cards= cards
            .split("")
            .filter(|x| !x.is_empty())
            .map(|c| Card::from_str(c).unwrap())
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

        let mut hand = Self {
            card_value: cards.iter().fold(0, |acc, (c, n)| acc + ((*c) as u32 * n)),
            // cards,
            bid,
            hand_type: HighCard
        };

        hand.hand_type = get_type(&cards);

        hand
    }

}

fn get_type(cards: &HashMap<Card, u32>) -> HandType {
    // Sorts the cards incorrectly by Card not count
    let sorted: Vec<(&Card, &u32)> = cards.iter()
        .sorted_by(|a, b| b.1.cmp(a.1))
        .collect();

    match sorted[0] {
        (_, 5) => {
            FiveOfAKind
        },
        (_, 4) => TwoPair,
        (_, 3) => {
            match sorted[1] {
                (_, 2) => HandType::FullHouse,
                _ => ThreeOfAKind
            }
        },
        (_, 2) => {
            match sorted[1] {
                (_, 2) => TwoPair,
                _ => HandType::OnePair,
            }
        },
        (_, 1) => HighCard,
        _ => { unreachable!("All cards are of some type - at minimum a hand must be a HighCard")}
    }
}

fn parse_hand(i: &str) -> IResult<&str, Hand> {
    tuple((is_a("123456789TJQKA"), space1, u32))(i).map(|(i, (cards, _, bid))| (i, Hand::new(cards, bid)))
}

fn parse_hands(i: &str) -> IResult<&str, Vec<Hand>> {
    let (i, hand) = separated_list1(tag("\n"), parse_hand)(i)?;

    Ok((i, hand))
}