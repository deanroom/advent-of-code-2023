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
    previous: Option<Box<Node>>,
    next: Option<Box<Node>>,
    position: (usize, usize),
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            outer_direction: self.outer_direction.clone(),
            inner_direction: self.inner_direction.clone(),
            previous: self.previous.clone(),
            next: self.next.clone(),
            position: self.position.clone(),
        }
    }
}

impl Node {
    fn new() -> Node {
        Node {
            outer_direction: vec![],
            inner_direction: vec![],
            next: None,
            previous: None,
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

    fn link_nodes(&mut self, nodes: &[Node]) -> &Node {
        let neighbours: Vec<&Node> = nodes
            .iter()
            .filter(|x| {
                let mut result = false;
                if x.position == (self.position.0 + 1, self.position.1) {
                    result = self.find_possible_directions(x)[..]
                        .iter()
                        .any(|x| **x == Direction::Up)
                }

                if self.position.0 > 0 && x.position == (self.position.0 - 1, self.position.1) {
                    result = self
                        .find_possible_directions(x)
                        .iter()
                        .any(|x| **x == Direction::Down);
                }
                if x.position == (self.position.0, self.position.1 + 1) {
                    result = self
                        .find_possible_directions(x)
                        .iter()
                        .any(|x| **x == Direction::Right);
                }
                if self.position.1 > 0 && x.position == (self.position.0, self.position.1 - 1) {
                    result = self
                        .find_possible_directions(x)
                        .iter()
                        .any(|x| **x == Direction::Left);
                }
                result
            })
            .collect();
        if neighbours.len() == 2 {
            self.next = Some(Box::new(neighbours[0].clone()));
            if !self.next.as_ref().expect("").is_start() {
                self.next.as_mut().expect("").link_nodes(nodes);
            }
            self.previous = Some(Box::new(neighbours[1].clone()));
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
        .filter(|x| direction.iter().any(|y| *x == y))
        .map(|x| x);
    matches
}

fn parse(input: &str) -> Vec<Vec<Node>> {
    let row_len = input.lines().count();
    let col_len = input
        .lines()
        .into_iter()
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
            node.position = (r, c);
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
    None
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
        let output = parse(input);
    }

    #[test]
    fn test_move() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let output = parse(input);
        let nodes: Vec<Node> = output.into_iter().flatten().collect();
        let start_node = nodes
            .iter()
            .find(|x| x.is_start())
            .expect("must find start node.");
        let nodes = &nodes[..];
        let mut start = start_node.clone();

        let root = start.link_nodes(nodes);
        println!("{:?}", root);
        assert_eq!(root.next.as_ref().expect("next node").position, (2, 1));
        assert_eq!(root.previous.as_ref().expect("next node").position, (3, 0));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}