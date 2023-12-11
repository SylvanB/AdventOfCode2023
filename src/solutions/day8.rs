use std::collections::HashMap;
use std::str::FromStr;
use nom::bytes::complete::{is_a, take, take_while};
use nom::character::complete::{alpha1, newline};
use nom::bytes::complete::tag;
use nom::IResult;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::{pair, tuple};
use strum_macros::EnumString;
use crate::utilities::load_input;

pub fn day8(path: String) {
    let i = load_input(path);
    let steps_to_way_out = calculate_steps_to_way_out(&i);

    println!("{}", steps_to_way_out);
}

fn calculate_steps_to_way_out(i: &str) -> usize {
    let (i, (directions, nodes)) = parse_map_file(i).unwrap();

    let node_tree = nodes.iter().fold(MapNodeTree::new(), |mut tree, node| {
        _ = tree.insert_node(node);
        tree
    });

    let mut curr_node_id: MapNodeId = "AAA";
    let mut i = 0;
    while curr_node_id != "ZZZ" {
        let curr_direction = &directions[i % directions.len()];
        let node = node_tree.get_node(&curr_node_id);
        curr_node_id = match curr_direction {
            Direction::L => node.left.unwrap(),
            Direction::R => node.right.unwrap(),
        };
        i += 1;
    }

    println!("Steps to find ZZZ: {}", i);

    i
}

#[derive(EnumString)]
enum Direction {
    #[strum(serialize = "L")]
    L,
    #[strum(serialize = "R")]
    R,
}

type MapNodeId<'arena> = &'arena str;

struct MapNodeTree<'arena> {
    nodes: HashMap<&'arena str, MapNode<'arena>>,
}

impl<'arena> MapNodeTree<'arena> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new()
        }
    }

    pub fn get_node(&self, node_idx: &MapNodeId) -> &MapNode {
        &self.nodes[*node_idx]
    }

    pub fn update_node(&mut self, node_idx: &'arena MapNodeId, updated: &'arena MapNode) {
        if let Some(node) = self.nodes.get_mut(node_idx) {
            *node = updated.clone();
        }
    }

    pub fn insert_node(&mut self, node: &'arena MapNode) -> MapNodeId {
        self.nodes.insert(&node.node_id, node.clone());
        node.node_id
    }

}

#[derive(Debug, Clone)]
struct MapNode<'arena> {
    node_id: &'arena str,
    left: Option<MapNodeId<'arena>>,
    right: Option<MapNodeId<'arena>>
}

impl<'arena> MapNode<'arena> {
    pub fn new(node_id: &'arena str, left: &'arena str, right: &'arena str ) -> Self {
        Self {
            node_id,
            left: Some(left),
            right: Some(right),
        }
    }
}

fn parse_directions(i: &str) -> IResult<&str, Vec<Direction>> {

    pair(is_a("LR"), newline)(i)
        .map(|(i, (mut directions, _))| {
            let d: Vec<Direction> = directions.split("").filter(|x| !x.is_empty()).map(|x| {
                Direction::from_str(x).unwrap()
            }).collect();
            (i, d)
        })
}

fn parse_identifier(i: &str) -> IResult<&str, &str> {
    let (i, alphabetic_str) = take(3usize)(i)?;
    let (_, parsed) = alpha1(alphabetic_str)?;

    Ok((i, parsed))
}

fn parse_map_node(i: &str) -> IResult<&str, MapNode> {
    tuple((parse_identifier, tag(" = ("), parse_identifier, tag(", "), parse_identifier, tag(")"), newline))(i)
        .map(|(i, (curr, _, left, _, right, _, _))| (i, MapNode::new(curr, left, right)))
}

fn parse_map_nodes(i: &str) -> IResult<&str, Vec<MapNode>> {
    many1(parse_map_node)(i)
}

fn parse_map_file(i: &str) -> IResult<&str, (Vec<Direction>, Vec<MapNode>)> {
    let (i, directions) = parse_directions(i)?;
    let (i, (_)) = newline(i)?;
    let (i, nodes) = parse_map_nodes(i)?;

    Ok((i, (directions, nodes)))
}