advent_of_code::solution!(10);

#[derive(Debug)]
enum Direction {
    None = 0b0000,
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
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
    position: (i32, i32),
    next_direction: Direction,
    previous_direction: Direction,
    outer_direction: Vec<Direction>,
    inner_direction: Vec<Direction>,
    next: Option<Box<Node>>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            outer_direction: self.outer_direction.clone(),
            inner_direction: self.inner_direction.clone(),
            next: self.next.clone(),
            next_direction: self.next_direction.clone(),
            previous_direction: self.previous_direction.clone(),
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
            next_direction: Direction::None,
            previous_direction: Direction::None,
            position: (0, 0),
        }
    }
    fn try_connect(&self, other: &Node) -> Vec<(Direction, Direction)> {
        let other = other.get_directions();
        let output = self
            .get_directions()
            .iter()
            .filter(move |x: &&Direction| match x {
                Direction::Up => other.iter().any(|y| y == &Direction::Down),
                Direction::Down => other.iter().any(|y| y == &Direction::Up),
                Direction::Left => other.iter().any(|y| y == &Direction::Right),
                Direction::Right => other.iter().any(|y| y == &Direction::Left),
                _ => false,
            })
            .map(|x| (x.clone(), self.reverse_direction(x)))
            .collect();
        output
    }

    fn link_nodes(&mut self, nodes: Vec<Node>) -> &Node {
        let neighbors: Vec<Node> = nodes
            .iter()
            .filter_map(|x| {
                let mut node = Node::new();
                node.position = x.position;
                node.outer_direction = x.outer_direction.clone();
                node.inner_direction = x.inner_direction.clone();

                let mut result: Option<Node> = None;
                let mut direction = Direction::None;
                let position = x.position;
                let possible_directions = self.try_connect(&node);

                if position == (self.position.0 - 1, self.position.1)
                    && possible_directions.iter().any(|x| x.0 == Direction::Up)
                {
                    direction = Direction::Up;
                }

                if position == (self.position.0 + 1, self.position.1)
                    && possible_directions.iter().any(|x| x.0 == Direction::Down)
                {
                    direction = Direction::Down;
                }

                if position == (self.position.0, self.position.1 + 1)
                    && possible_directions.iter().any(|x| x.0 == Direction::Right)
                {
                    direction = Direction::Right;
                }

                if position == (self.position.0, self.position.1 - 1)
                    && possible_directions.iter().any(|x| x.0 == Direction::Left)
                {
                    direction = Direction::Left;
                }
                
                if direction != Direction::None {
                    self.next_direction = direction.clone();
                    node.previous_direction = self.reverse_direction(&direction);
                    result = Some(node);
                }

                result
            })
            .collect();

        if !neighbors.is_empty() {
            let next = neighbors[0].clone();
            if neighbors.len() == 2 {
                self.previous_direction = neighbors[1].previous_direction.clone();
            }
            self.next = Some(Box::new(next));
            let nodes: Vec<Node> = nodes
                .into_iter()
                .filter(|x| x.position != self.position)
                .collect();

            self.next.as_mut().expect("").link_nodes(nodes);
        } else {
            let directions = self.get_directions();
            let direction = directions
                .iter()
                .find(|x| **x != self.next_direction)
                .expect("node.");
            self.next_direction = direction.clone();
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
    fn get_directions(&self) -> Vec<Direction> {
        let matches = self
            .outer_direction
            .iter()
            .filter(|x| self.inner_direction.iter().any(|y| *x == y))
            .cloned();
        matches.collect()
    }
    fn reverse_direction(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Node>> {
    let row_len = input.lines().count();
    let col_len = input.lines().next().expect("a row").chars().count();
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
    let mut start_node: Node = nodes
        .iter()
        .find(|x| x.is_start())
        .expect("must ind start node.")
        .clone();
    let mut root = start_node.link_nodes(nodes);
    let mut nodes = vec![root];
    while root.next.is_some() {
        root = root.next.as_ref().expect("node");
        nodes.push(root);
    }
    nodes
        .iter()
        // .filter(|x| *x.inner_direction == vec![Direction::Down, Direction::Right])
        .for_each(|x| {
            println!(
                "{:?}, {:?}, {:?}",
                x.position, x.previous_direction, x.next_direction
            );
        });

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {}

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
