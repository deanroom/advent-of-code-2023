use nom::character::complete::{self, alpha1, alphanumeric1, space1};
use nom::sequence::tuple;
use nom::IResult;
use std::collections::{BTreeMap, HashMap};
advent_of_code::solution!(8);
//TODO
// btreemap 和hashmap的区别

#[derive(Debug, Default)]
struct Map<'a> {
    instructions: &'a str,
    nodes: BTreeMap<String, Node>,
}

#[derive(Debug, Default)]
struct Node {
    left: String,
    right: String,
}

impl<'a> Map<'a> {
    fn left(&self, node: &str) -> &str {
        let next_node = self
            .nodes
            .get(node)
            .expect("node must be found.")
            .left
            .as_str();
        next_node
    }
    fn right(&self, node: &str) -> &str {
        let next_node = self
            .nodes
            .get(node)
            .expect("node must be found.")
            .right
            .as_str();
        next_node
    }
    fn run(&self, node: &'a str) -> (&str, u32) {
        let mut out_node: &str = node;
        let mut steps: u32 = 0;
        for instruction in self.instructions.chars() {
            out_node = match instruction {
                'L' => self.left(out_node),
                'R' => self.right(out_node),
                _ => panic!(),
            };
            steps += 1;
        }
        (out_node, steps)
    }
}

// fn parse_line(input: &str) -> IResult<&str, Node> {
//     let result  = tuple((alpha1, space1,space1,space1,alpha1))(input)?;
//     Ok((input,Node{ left: todo!(), right: todo!() }))
// }
fn parse(input: &str) -> Map {
    let mut lines = input.lines();
    let instructions = lines.next().expect("must be instructions.");

    lines.next();
    let ouput = lines
        .map(|line| {
            let ouput: Vec<&str> = line
                .split(&[' ', '=', '(', ',', ')'][..])
                .filter(|x| *x != " " && *x != "")
                .collect();
            (
                ouput[0].to_string(),
                Node {
                    left: ouput[1].to_string(),
                    right: ouput[2].to_string(),
                },
            )
        })
        .into_iter();

    Map {
        instructions,
        nodes: BTreeMap::<String, Node>::from_iter(ouput),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut node = "AAA";
    let mut steps = 0;
    while node != "ZZZ" {
        let (new_node, step) = map.run(node);
        node = new_node;
        steps += step;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let map = parse(&input);
        assert_eq!(map.instructions, "RL");
        assert_eq!(map.nodes.len(), 7);
        assert_eq!(map.nodes.get("CCC").expect("A node.").right, "GGG");
        println!("{:?}", map);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
