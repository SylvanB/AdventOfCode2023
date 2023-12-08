use std::cmp::Ordering;
use std::collections::{BTreeMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::str::FromStr;
use itertools::Itertools;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{space1, u32};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use strum_macros::{Display, EnumString};

pub fn day7(path: String) {
    let file = File::open(path).unwrap();
    let mut buffer = BufReader::new(file);
    let mut i = String::new();
    _ = buffer.read_to_string(&mut i);

    let total_winnings = get_total_winnings(&i);

    println!("{}", total_winnings);
}

fn get_total_winnings(i: &str) -> u32 {
    let (_, mut hands) = parse_hands(&i).unwrap();
    hands.sort();
    let mut total_winnings = 0;
    for (idx, h) in hands.iter().enumerate() {
        let a = idx + 1;
        println!("{}", format!("Rank: {} Type: {} Value: {} Raw: {}", idx + 1, &h.hand_type, &h.card_value, &h.raw_hand));
        total_winnings += h.bid * a as u32;
    }

    total_winnings
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumString)]
enum Card {
    #[strum(serialize = "2")]
    Two     = 2,
    #[strum(serialize = "3")]
    Three   = 4,
    #[strum(serialize = "4")]
    Four    = 8,
    #[strum(serialize = "5")]
    Five    = 16,
    #[strum(serialize = "6")]
    Six     = 32,
    #[strum(serialize = "7")]
    Seven   = 64,
    #[strum(serialize = "8")]
    Eight   = 128,
    #[strum(serialize = "9")]
    Nine    = 256,
    #[strum(serialize = "T")]
    Ten     = 512,
    #[strum(serialize = "J")]
    Jack    = 1024,
    #[strum(serialize = "Q")]
    Queen   = 2048,
    #[strum(serialize = "K")]
    King    = 4096,
    #[strum(serialize = "A")]
    Ace     = 8192
}

#[derive(Display, Debug, Eq,PartialEq, Ord, PartialOrd, EnumString)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Debug, Eq, Ord)]
struct Hand<'a> {
    raw_hand: &'a str,
    cards: Vec<Card>,
    card_value: u32,
    bid: u32,
    hand_type: HandType,
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.card_value == other.card_value
    }

}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = if &self.hand_type.cmp(&other.hand_type) == &Ordering::Equal {
            let mut res = Ordering::Equal;
            for (c1, c2) in self.cards.iter().zip(&other.cards) {
                if c1.cmp(c2) == Ordering::Equal {
                    continue;
                } else {
                    res = c1.cmp(&c2);
                    break;
                }
            }
            Some(res)
        } else {
            Some(self.hand_type.cmp(&other.hand_type))
        };

        result
    }
}

impl<'Hand> Hand<'Hand> {
    pub fn new(cards: &'Hand str, bid: u32) -> Hand<'Hand> {
        let parsed_cards: Vec<Card> = cards
            .split("")
            .filter(|x| !x.is_empty())
            .map(|c| Card::from_str(c).unwrap())
            .collect();

        let cards_freq = parsed_cards.clone().into_iter()
            .fold(BTreeMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

        let mut hand = Self {
            raw_hand: cards,
            cards: parsed_cards.clone(),
            card_value: Self::calculate_value(parsed_cards.clone()),
            bid,
            hand_type: HandType::HighCard
        };

        hand.hand_type = get_type(&cards_freq);

        hand
    }

    fn calculate_value(cards: Vec<Card>) -> u32 {
        let mut score = 0;

        for (idx, card) in cards.into_iter().enumerate() {
            let card_pos: u32 = 5 - idx as u32;
            score += card as u32 * card_pos;
        }

        score
    }
}

fn get_type(cards: &BTreeMap<Card, u32>) -> HandType {
    let sorted: Vec<(&Card, &u32)> = cards.iter()
        .sorted_by(|a, b| a.1.cmp(b.1)).rev()
        .collect();

    match sorted[0] {
        (_, 5) => {
            HandType::FiveOfAKind
        },
        (_, 4) => HandType::FourOfAKind,
        (_, 3) => {
            match sorted[1] {
                (_, 2) => HandType::FullHouse,
                _ => HandType::ThreeOfAKind
            }
        },
        (_, 2) => {
            match sorted[1] {
                (_, 2) => HandType::TwoPair,
                _ => HandType::OnePair,
            }
        },
        (_, 1) => HandType::HighCard,
        _ => { unreachable!("All cards are of some type - at minimum a hand must be a HighCard")}
    }
}

fn parse_hand(i: &str) -> IResult<&str, Hand> {
    tuple((is_a("23456789TJQKA"), space1, u32))(i).map(|(i, (cards, _, bid))| (i, Hand::new(cards, bid)))
}

fn parse_hands(i: &str) -> IResult<&str, Vec<Hand>> {
    let (i, hand) = separated_list1(tag("\n"), parse_hand)(i)?;

    Ok((i, hand))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_hand_correctly() {
        assert_eq!(parse_hand("A2345 101").unwrap(), ("", Hand::new("A2345", 101)));
        assert_eq!(parse_hand("AAKKQ 202").unwrap(), ("", Hand::new("AAKKQ", 202)));
        assert_eq!(parse_hand("AAKJQ 303").unwrap(), ("", Hand::new("AAKJQ", 303)));
    }

    #[test]
    fn should_parse_multiple_hands_correctly() {
        assert_eq!(parse_hands("A2345 101\nAAKKQ 202\nAAKJQ 303").unwrap(), (
            "",
            vec![
                Hand::new("A2345", 101),
                Hand::new("AAKKQ", 202),
                Hand::new("AAKJQ", 303)
            ]));
    }

    #[test]
    fn hand_should_order_by_highest_value_first_occurring_card() {
        assert_eq!(Hand::new("A2345", 0), Hand::new("A2345", 0));
        assert!(Hand::new("2A345", 0) < Hand::new("A2345", 0));
        assert!(Hand::new("A2345", 0) > Hand::new("2A345", 0));
        assert!(Hand::new("QA234", 0) < Hand::new("QAA34", 0));
    }

    #[test]
    fn parses_correct_hand_type() {
        assert_eq!(Hand::new("A2345", 0).hand_type, HandType::HighCard);
        assert_eq!(Hand::new("AA345", 0).hand_type, HandType::OnePair);
        assert_eq!(Hand::new("A345A", 0).hand_type, HandType::OnePair);
        assert_eq!(Hand::new("AAK5K", 0).hand_type, HandType::TwoPair);
        assert_eq!(Hand::new("A4AA5", 0).hand_type, HandType::ThreeOfAKind);
        assert_eq!(Hand::new("AA5AA", 0).hand_type, HandType::FourOfAKind);
        assert_eq!(Hand::new("AAAAA", 0).hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand::new("AKAKK", 0).hand_type, HandType::FullHouse);
    }

    #[test]
    fn should_order_hands_by_type_then_value() {
        let hands = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
        let (_, mut hands) = parse_hands(hands).unwrap();

        hands.sort();
        assert_eq!(
            hands,
            vec![
                Hand::new("32T3K", 765),
                Hand::new("KTJJT", 220),
                Hand::new("KK677", 28),
                Hand::new("T55J5", 684),
                Hand::new("QQQJA", 483)
            ]
        );
    }

    #[test]
    fn should_calculate_correct_winnings() {
        let hands = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

        assert_eq!(get_total_winnings(hands), 6440);
    }

    #[test]
    fn should_calculate_correct_winnings2() {
        let hands = r#"AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43
"#;
        assert_eq!(get_total_winnings(hands), 1343);
    }

    #[test]
    fn should_calculate_correct_winnings3() {
        let hands = r#"23456 22
56789 19
KJJKK 2
AAAAJ 3
JJ243 7
QJ256 6
QQ562 5
Q8Q24 4
AAAAT 3
TJJJJ 2
6789T 18
789TJ 17
22345 13
34567 21
45678 20
32245 12
33245 11
89TJQ 16
9TJQK 15
TJQKA 14
3J245 10
J3425 9
J5432 8
JJJJJ 1
"#;
        assert_eq!(get_total_winnings(hands), 2237);
    }

}