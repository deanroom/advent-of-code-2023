advent_of_code::solution!(10);

#[derive(Debug)]
enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Self::Up => Self::Up,
            Self::Down => Self::Down,
            Self::Left => Self::Left,
            Self::Right => Self::Right,
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Debug)]
struct Node {
    outer_direction: Vec<Direction>,
    inner_direction: Vec<Direction>,
    // previous: Option<Box<Node>>,
    next: Option<Box<Node>>,
    position: (i32, i32),
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            outer_direction: self.outer_direction.clone(),
            inner_direction: self.inner_direction.clone(),
            // previous: self.previous.clone(),
            next: self.next.clone(),
            position: self.position,
        }
    }
}

impl Node {
    fn new() -> Node {
        Node {
            outer_direction: vec![],
            inner_direction: vec![],
            next: None,
            // previous: None,
            position: (0, 0),
        }
    }
    fn find_possible_directions(&self, other: &Node) -> Vec<&Direction> {
        let this_iter = matched_direction(&self.outer_direction[..], &self.inner_direction[..]);
        let other_iter = matched_direction(&other.outer_direction[..], &other.inner_direction[..]);
        let other: Vec<&Direction> = other_iter.collect();
        let output: Vec<&Direction> = this_iter
            .filter(move |x: &&Direction| match x {
                Direction::Up => other.iter().any(|y| y == &&Direction::Down),
                Direction::Down => other.iter().any(|y| y == &&Direction::Up),
                Direction::Left => other.iter().any(|y| y == &&Direction::Right),
                Direction::Right => other.iter().any(|y| y == &&Direction::Left),
            })
            .collect();
        output
    }

    fn link_nodes(&mut self, nodes: Vec<Node>) -> &Node {
        let neighbours: Vec<&Node> = nodes
            .iter()
            .filter(|x| {
                let mut result = false;
                let node: &&Node = x;
                let position = x.position;
                let possible_directions = self.find_possible_directions(node);
                if position == (self.position.0 - 1, self.position.1) {
                    result = possible_directions.iter().any(|x| **x == Direction::Up)
                }

                if position == (self.position.0 + 1, self.position.1) {
                    result = possible_directions.iter().any(|x| **x == Direction::Down);
                }

                if position == (self.position.0, self.position.1 + 1) {
                    result = possible_directions.iter().any(|x| **x == Direction::Right);
                }

                if position == (self.position.0, self.position.1 - 1) {
                    result = possible_directions.iter().any(|x| **x == Direction::Left);
                }
                result
            })
            .collect();
        if !neighbours.is_empty() {
            let next = neighbours[0].clone();
            self.next = Some(Box::new(next));
            let nodes: Vec<Node> = nodes
                .into_iter()
                .filter(|x| x.position != self.position)
                .collect();
            if !self.next.as_ref().expect("").is_start() {
                self.next.as_mut().expect("").link_nodes(nodes);
            }
        }
        self
    }
    fn is_start(&self) -> bool {
        self.inner_direction
            == vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
    }
}
fn matched_direction<'a>(
    position: &'a [Direction],
    direction: &'a [Direction],
) -> impl Iterator<Item = &'a Direction> {
    let matches = position
        .iter()
        .filter(|x| direction.iter().any(|y| *x == y));
    matches
}

fn parse(input: &str) -> Vec<Vec<Node>> {
    let row_len = input.lines().count();
    let col_len = input
        .lines()
        .next()
        .expect("a row")
        .chars()
        .count();
    let mut nodes: Vec<Vec<Node>> = vec![];
    for (r, row) in input.lines().enumerate() {
        let mut nodes_row: Vec<Node> = vec![];
        for (c, col) in row.chars().enumerate() {
            let mut node = Node::new();
            node.inner_direction = parse_direction(col);
            node.position = (r as i32, c as i32);
            if r > 0 {
                node.outer_direction.push(Direction::Up);
            }
            if r < row_len - 1 {
                node.outer_direction.push(Direction::Down);
            }
            if c > 0 {
                node.outer_direction.push(Direction::Left);
            }
            if c < col_len - 1 {
                node.outer_direction.push(Direction::Right);
            }
            nodes_row.push(node);
        }
        nodes.push(nodes_row);
    }
    nodes
}
fn parse_direction(c: char) -> Vec<Direction> {
    match c {
        '|' => vec![Direction::Up, Direction::Down],
        '-' => vec![Direction::Left, Direction::Right],
        'L' => vec![Direction::Up, Direction::Right],
        'J' => vec![Direction::Up, Direction::Left],
        '7' => vec![Direction::Down, Direction::Left],
        'F' => vec![Direction::Down, Direction::Right],
        '.' => vec![],
        'S' => vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ],
        _ => panic!("wrong char."),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let output = parse(input);
    let nodes: Vec<Node> = output.into_iter().flatten().collect();
    let mut start_node: Node = nodes
        .iter()
        .find(|x| x.is_start())
        .expect("must ind start node.")
        .clone();
    let mut root = start_node.link_nodes(nodes);
    let mut count = 1;
    while root.next.is_some() {
        root = root.next.as_ref().expect("node");
        count += 1;
    }
    Some((count + 1) / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = parse(input);
    let nodes: Vec<Node> = output.into_iter().flatten().collect();
    println!("{:?}", nodes);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let output = parse(input);
        let nodes: Vec<Node> = output.into_iter().flatten().collect();
        assert_eq!(nodes.len(), 25);
    }

    #[test]
    fn test_move() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let output = parse(input);
        let nodes: Vec<Node> = output.into_iter().flatten().collect();
        let mut start_node: Node = nodes
            .iter()
            .find(|x| x.is_start())
            .expect("must ind start node.")
            .clone();
        let root = start_node.link_nodes(nodes);
        println!("{:?}", root);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
