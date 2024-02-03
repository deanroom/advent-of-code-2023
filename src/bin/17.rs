use std::{cmp::Ordering, collections::BinaryHeap};

advent_of_code::solution!(17);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    value: u32,
    position: (usize, usize),
    last_position: (usize, usize),
    direction: Option<(i32, i32)>,
    last_direction: Option<(i32, i32)>,
    cost: u32,
    moves_in_last_direction: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| {
                    let output = n.to_digit(10).unwrap();
                    output
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut dist = vec![vec![std::u32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();

    let start = (0, 0);
    dist[start.0][start.1] = matrix[start.0][start.1];
    heap.push(State {
        value: matrix[start.0][start.1],
        cost: dist[start.0][start.1],
        position: start,
        direction: None,
        last_direction: Some((1, 0)),
        last_position: start,
        moves_in_last_direction: 0,
    });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    print_matrix(&matrix);

    while let Some(State {
        value: _,
        cost,
        position,
        direction,
        last_direction,
        last_position: _,
        moves_in_last_direction,
    }) = heap.pop()
    {
        let (row, col) = position;

        if position == (rows - 1, cols - 1) {
            // break;
        }

        if cost > dist[row][col] {
            continue;
        }

        for &new_direction in &directions {
            let new_position: (usize, usize) = (
                (row as i32 + new_direction.0) as usize,
                (col as i32 + new_direction.1) as usize,
            );

            if new_position.0 < rows && new_position.1 < cols {
                let new_cost = cost + matrix[new_position.0][new_position.1];
                let new_moves_in_last_direction = if Some(new_direction) == last_direction {
                    moves_in_last_direction + 1
                } else {
                    1
                };

                if new_moves_in_last_direction <= 3
                    && new_cost < dist[new_position.0][new_position.1]
                {
                    dist[new_position.0][new_position.1] = new_cost;
                    let state = State {
                        value: matrix[new_position.0][new_position.1],
                        cost: new_cost,
                        position: new_position,
                        direction: Some(new_direction),
                        last_direction: direction,
                        last_position: position,
                        moves_in_last_direction: new_moves_in_last_direction,
                    };
                    heap.push(state);
                }
            }
        }
        heap.iter().for_each(|x| println!("{:?}", x));
        println!("-----------------");
    }
    print_matrix(&dist);
    Some(dist[rows - 1][cols - 1])
}

fn print_matrix(dist: &Vec<Vec<u32>>) {
    for row in dist {
        for value in row {
            print!("{:>3} ", value);
        }
        println!();
    }
    println!("-----------------");
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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
