use std::collections::BTreeMap;
advent_of_code::solution!(8);
//TODO
// BTreeMap 和 HahMap 的区别
// 最小公倍数计算

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
            .expect(format!("node [{}] must be found.", node).as_str())
            .left
            .as_str();
        next_node
    }
    fn right(&self, node: &str) -> &str {
        let next_node = self
            .nodes
            .get(node)
            .expect(format!("node [{}] must be found.", node).as_str())
            .right
            .as_str();
        next_node
    }
    fn run(&self, node: &'a str) -> &str {
        let mut out_node: &str = node;
        for instruction in self.instructions.chars() {
            out_node = match instruction {
                'L' => self.left(out_node),
                'R' => self.right(out_node),
                _ => panic!(),
            };
        }
        out_node
    }
}

fn parse(input: &str) -> Map {
    let mut lines = input.lines();
    let instructions = lines.next().expect("must be instructions.");

    lines.next();
    let output = lines
        .map(|line| {
            let output: Vec<&str> = line
                .split(&[' ', '=', '(', ',', ')'][..])
                .filter(|x| *x != " " && *x != "")
                .collect();
            (
                output[0].to_string(),
                Node {
                    left: output[1].to_string(),
                    right: output[2].to_string(),
                },
            )
        })
        .into_iter();

    Map {
        instructions,
        nodes: BTreeMap::<String, Node>::from_iter(output),
    }
}
pub fn lcm(numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lcm(&numbers[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut node = "AAA";
    let mut steps = 0;
    while node != "ZZZ" {
        node = map.run(node);
        node = node;
        steps += map.instructions.len();
    }

    Some(steps as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse(input);
    let start_nodes: Vec<&str> = map
        .nodes
        .iter()
        .filter(|x| x.0.ends_with("A"))
        .map(|x| x.0.as_str())
        .collect();

    let steps: Vec<usize> = start_nodes
        .iter()
        .map(|x| {
            let mut node = *x;
            let mut steps = 0;
            while !node.ends_with("Z") {
                node = map.run(node);
                node = node;
                steps += map.instructions.len();
            }
            steps
        })
        .collect();
    let result = lcm(&steps);
    Some(result)
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
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
