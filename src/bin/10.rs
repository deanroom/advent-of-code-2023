advent_of_code::solution!(10);

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North = 0b0001,
    South = 0b0010,
    West = 0b0100,
    East = 0b1000,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
    Ground,
}

#[derive(Debug, Clone)]
struct Node {
    position: (i32, i32),
    pipe_type: PipeType,
    outer_direction: Vec<Direction>,
    inner_direction: Vec<Direction>,
    next: Option<Box<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            outer_direction: vec![],
            inner_direction: vec![],
            next: None,
            pipe_type: PipeType::Ground,
            // next_direction: Direction::None,
            // previous_direction: Direction::None,
            position: (0, 0),
        }
    }
    fn try_connect(&self, other: &Node) -> Vec<(Direction, Direction)> {
        let other = other.get_directions();
        let output = self
            .get_directions()
            .iter()
            .filter(move |x: &&Direction| other.iter().any(|y| *y == self.reverse_direction(x)))
            .map(|x| (x.clone(), self.reverse_direction(x)))
            .collect();
        output
    }

    fn link_nodes(&mut self, nodes: Vec<Node>) -> &Node {
        let neighbors: Vec<Node> = nodes
            .iter()
            .filter_map(|x| {
                let node = x.clone();
                let position = x.position;
                let possible_directions = self.try_connect(&node);

                if position == (self.position.0 - 1, self.position.1)
                    && possible_directions.iter().any(|x| x.0 == Direction::North)
                {
                    Some(node)
                } else if position == (self.position.0 + 1, self.position.1)
                    && possible_directions.iter().any(|x| x.0 == Direction::South)
                {
                    Some(node)
                } else if position == (self.position.0, self.position.1 + 1)
                    && possible_directions.iter().any(|x| x.0 == Direction::East)
                {
                    Some(node)
                } else if position == (self.position.0, self.position.1 - 1)
                    && possible_directions.iter().any(|x| x.0 == Direction::West)
                {
                    Some(node)
                } else {
                    None
                }
            })
            .collect();

        if !neighbors.is_empty() {
            let next = neighbors[0].clone();

            self.next = Some(Box::new(next));
            let nodes: Vec<Node> = nodes
                .into_iter()
                .filter(|x| x.position != self.position)
                .collect();

            self.next.as_mut().expect("").link_nodes(nodes);
        }
        self
    }

    fn is_start(&self) -> bool {
        self.pipe_type == PipeType::StartingPosition
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
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
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
            let (direction, pip_type) = parse_direction(col);
            node.inner_direction = direction;
            node.position = (r as i32, c as i32);
            if r > 0 {
                node.outer_direction.push(Direction::North);
            }
            if r < row_len - 1 {
                node.outer_direction.push(Direction::South);
            }
            if c > 0 {
                node.outer_direction.push(Direction::West);
            }
            if c < col_len - 1 {
                node.outer_direction.push(Direction::East);
            }
            node.pipe_type = pip_type;
            nodes_row.push(node);
        }
        nodes.push(nodes_row);
    }
    nodes
}

fn parse_direction(c: char) -> (Vec<Direction>, PipeType) {
    match c {
        '|' => (vec![Direction::North, Direction::South], PipeType::Vertical),
        '-' => (vec![Direction::West, Direction::East], PipeType::Horizontal),
        'L' => (vec![Direction::North, Direction::East], PipeType::NorthEast),
        'J' => (vec![Direction::North, Direction::West], PipeType::NorthWest),
        '7' => (vec![Direction::South, Direction::West], PipeType::SouthWest),
        'F' => (vec![Direction::South, Direction::East], PipeType::SouthEast),
        '.' => (vec![], PipeType::Ground),
        'S' => (
            vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            PipeType::StartingPosition,
        ),
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

fn is_point_in_polygon(p: (i32, i32), polygon: &[(i32, i32)]) -> bool {
    let n = polygon.len();
    let mut inside = false;

    for i in 0..n {
        let j = (i + 1) % n;

        let vi = &polygon[i];
        let vj = &polygon[j];

        // 检查射线与多边形边的交点
        if (vi.1 > p.1) != (vj.1 > p.1)
            && p.0 < ((vj.0 - vi.0) * (p.1 - vi.1) / (vj.1 - vi.1) + vi.0)
        {
            inside = !inside;
        }
    }
    inside
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = parse(input);
    let all_nodes: Vec<Node> = output.into_iter().flatten().collect();
    let mut start_node: Node = all_nodes
        .iter()
        .find(|x| x.is_start())
        .expect("must in start node.")
        .clone();
    let mut root = start_node.link_nodes(all_nodes.clone());
    let mut nodes_in_graph = vec![root];
    while root.next.is_some() {
        root = root.next.as_ref().expect("node");
        nodes_in_graph.push(root);
    }

    let nodes_in_graph: Vec<(i32, i32)> = nodes_in_graph
        .iter()
        .map(|p| (p.position.1, p.position.0))
        .collect();

    let result: Vec<(i32, i32)> = all_nodes
        .iter()
        .filter(|x| !nodes_in_graph.contains(&(x.position.1, x.position.0)))
        .map(|x| (x.position.1, x.position.0))
        .filter(|x| is_point_in_polygon(*x, &nodes_in_graph[..]))
        .collect();

    // println!("{:?}", result);

    Some(result.len() as u32)
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
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
