use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

advent_of_code::solution!(17);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
    last_direction: Option<(i32, i32)>,
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
    // let mut dist = vec![vec![std::u32::MAX; cols]; rows];
    let mut best: HashMap<((usize, usize), (i32, i32), u32), u32> = HashMap::new();

    // 使用时，如果键不存在，返回无穷大
    let mut heap = BinaryHeap::new();

    let start = (0, 0);
    heap.push(State {
        cost: 0,
        position: start,
        last_direction: None,
        moves_in_last_direction: 0,
    });

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(State {
        cost,
        position,
        last_direction,
        moves_in_last_direction,
    }) = heap.pop()
    {
        let (row, col) = position;

        if position == (rows - 1, cols - 1) {
            return Some(cost);
        }
        for &new_direction in &directions {
            //reverse direction ignore.
            if last_direction.is_some()
                && last_direction.expect("").0 + new_direction.0 == 0
                && last_direction.expect("").1 + new_direction.1 == 0
            {
                continue;
            }

            let new_moves_in_last_direction = if Some(new_direction) == last_direction {
                moves_in_last_direction + 1
            } else {
                1
            };

            if new_moves_in_last_direction == 4 {
                continue;
            }
            let new_position: (usize, usize) = (
                (row as i32 + new_direction.0) as usize,
                (col as i32 + new_direction.1) as usize,
            );

            if new_position.0 < rows && new_position.1 < cols {
                let new_cost = cost + matrix[new_position.0][new_position.1];

                let key: ((usize, usize), (i32, i32), u32) =
                    (new_position, new_direction, new_moves_in_last_direction);
                let value = *best.get(&key).unwrap_or(&u32::MAX);

                if new_cost >= value {
                    continue;
                }
                best.insert(key, new_cost);
                let state = State {
                    cost: new_cost,
                    position: new_position,
                    last_direction: Some(new_direction),
                    moves_in_last_direction: new_moves_in_last_direction,
                };
                heap.push(state)
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
    // let mut dist = vec![vec![std::u32::MAX; cols]; rows];
    let mut best: HashMap<((usize, usize), (i32, i32), u32), u32> = HashMap::new();

    // 使用时，如果键不存在，返回无穷大
    let mut heap = BinaryHeap::new();

    let start = (0, 0);
    heap.push(State {
        cost: 0,
        position: start,
        last_direction: None,
        moves_in_last_direction: 0,
    });

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(State {
        cost,
        position,
        last_direction,
        moves_in_last_direction,
    }) = heap.pop()
    {
        let (row, col) = position;

        if position == (rows - 1, cols - 1) && moves_in_last_direction >= 4 {
            return Some(cost);
        }
        for &new_direction in &directions {
            //reverse direction ignore.
            if last_direction.is_some()
                && last_direction.expect("").0 + new_direction.0 == 0
                && last_direction.expect("").1 + new_direction.1 == 0
            {
                continue;
            }

            if last_direction.is_some()
                && new_direction != last_direction.unwrap()
                && moves_in_last_direction < 4
            {
                continue;
            }

            let new_moves_in_last_direction = if Some(new_direction) == last_direction {
                moves_in_last_direction + 1
            } else {
                1
            };

            if new_moves_in_last_direction == 11 {
                continue;
            }
            let new_position: (usize, usize) = (
                (row as i32 + new_direction.0) as usize,
                (col as i32 + new_direction.1) as usize,
            );

            if new_position.0 < rows && new_position.1 < cols {
                let new_cost = cost + matrix[new_position.0][new_position.1];

                let key: ((usize, usize), (i32, i32), u32) =
                    (new_position, new_direction, new_moves_in_last_direction);
                let value = *best.get(&key).unwrap_or(&u32::MAX);

                if new_cost >= value {
                    continue;
                }
                best.insert(key, new_cost);
                let state = State {
                    cost: new_cost,
                    position: new_position,
                    last_direction: Some(new_direction),
                    moves_in_last_direction: new_moves_in_last_direction,
                };
                heap.push(state)
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
