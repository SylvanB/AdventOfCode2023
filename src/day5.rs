use std::fs::File;
use std::io::{BufReader, Read};
use nom;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace1};
use nom::character::complete::u64 as nom_u64;
use nom::IResult;
use nom::multi::{many_till, separated_list1};
use nom::sequence::{pair, separated_pair, tuple};

#[derive(Debug, PartialEq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => unimplemented!("no other categories supported"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct CategoryMap {
    source: Category,
    destination: Category,
    conversion_ranges: Vec<(u64, u64, u64)>
}

impl CategoryMap {
    pub fn new(source: Category, destination: Category, conversion_ranges: Vec<(u64, u64, u64)>) -> Self {
        Self {
            source,
            destination,
            conversion_ranges
        }
    }
}

pub fn day5(path: String) {
    let file = File::open(path).unwrap();
    let mut buffer = BufReader::new(file);
    let mut i = String::new();
    _ = buffer.read_to_string(&mut i);

    let parsed_input = parse_full_conversion_map_file(&i).unwrap();
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<u64>> {
    tuple((tag("seeds: "), separated_list1(tag(" "), nom_u64), tag("\n")))(i)
        .map(|(i, (_, s, _))| (i, s))
}

fn parse_map_categories(i: &str) -> IResult<&str, (Category, Category)>{
    tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(i)
        .map(|(i, (s, _, d, _))| (i, (s.into(), d.into())))
}

fn parse_conversion_tuple(i: &str) -> IResult<&str, (u64, u64, u64)> {
        tuple((nom_u64, tag(" "), nom_u64, tag(" "), nom_u64))(i)
            .map(|(i, (d1, _, d2, _, d3))| (i, (d1, d2, d3)))
}

fn parse_map(i: &str) -> IResult<&str, CategoryMap> {
    let (i, (source, destination)) = parse_map_categories(i)?;
    let (i, conversion_ranges) = separated_list1(tag("\n"), parse_conversion_tuple)(i)?;
    Ok((i, CategoryMap::new(source, destination, conversion_ranges)))
}

fn parse_maps(i: &str) -> IResult<&str, Vec<CategoryMap>>{
    separated_list1(multispace1, parse_map)(i)
}

fn parse_full_conversion_map_file(i: &str) -> IResult<&str, (Vec<u64>, Vec<CategoryMap>)> {
    let (i, (seeds, _, category_maps)) = tuple ((
        parse_seeds,
        multispace1,
        parse_maps
    ))(i)?;

    Ok((i, (seeds, category_maps)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_extract_map_categories() {
        assert_eq!(parse_map_categories("seed-to-soil map:\n"), Ok(("", (Category::Seed, Category::Soil))));
        assert_eq!(parse_map_categories("temperature-to-location map:\n"), Ok(("", (Category::Temperature, Category::Location))));
        assert_eq!(parse_map_categories("water-to-fertilizer map:\n"), Ok(("", (Category::Water, Category::Fertilizer))));
    }

    #[test]
    fn should_extract_conversion_tuple() {
        assert_eq!(parse_conversion_tuple("88 18 7"), Ok(("", (88, 18, 7))));
        assert_eq!(parse_conversion_tuple("34344 43242342 5435345"), Ok(("", (34344, 43242342, 5435345))));
        assert_eq!(parse_conversion_tuple("4043589752 1302126249 251377544"), Ok(("", (4043589752, 1302126249, 251377544))));
    }

    #[test]
    fn should_parse_seed_list() {
        assert_eq!(parse_seeds("seeds: 1 2 3 4 5\n"), Ok(("", vec![1,2,3,4,5])))
    }

    #[test]
    fn should_extract_category_map() {
        let map_input = "seed-to-soil map:\n34344 43242342 5435345\n123 123 123\n4043589752 1302126249 251377544";
        assert_eq!(parse_map(map_input),
                   Ok(("", CategoryMap {
                       source: Category::Seed,
                       destination: Category::Soil,
                       conversion_ranges: vec![(34344, 43242342, 5435345), (123, 123, 123), (4043589752, 1302126249, 251377544)]
                   })));
    }

    #[test]
    fn should_parse_entire_file() {
        let map_input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"#;

        assert_eq!(parse_full_conversion_map_file(map_input),
                   Ok(("",
                       (vec![79, 14, 55, 13],
                       vec![
                           CategoryMap {
                               source: Category::Seed,
                               destination: Category::Soil,
                               conversion_ranges: vec![(50, 98, 2),(52, 50, 48)]
                           },
                           CategoryMap {
                               source: Category::Soil,
                               destination: Category::Fertilizer,
                               conversion_ranges: vec![(0, 15,37), (37, 52, 2), (39, 0, 15)]
                           }
                       ]))));
    }

}