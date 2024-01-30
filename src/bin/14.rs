use num_complex::Complex;
use std::collections::HashSet;
type Position = HashSet<Complex<i32>>;

advent_of_code::solution!(14);

fn tilt(rounds: &Position, d: Complex<i32>, board: &Position, blocked: &Position) -> Position {
    let mut rounds = rounds.clone();
    loop {
        let free: HashSet<_> = board
            .difference(&rounds)
            .collect::<HashSet<&Complex<i32>>>()
            .difference(&blocked.iter().collect::<HashSet<&Complex<i32>>>())
            .cloned()
            .collect();

        let new_rounds: HashSet<_> = rounds
            .iter()
            .map(|&z| if free.contains(&(z - d)) { z - d } else { z })
            .collect();
        if new_rounds == rounds {
            return new_rounds;
        }
        rounds = new_rounds;
    }
}

fn cycle(rounds: &Position, board: &Position, blocked: &Position) -> Position {
    let directions = [
        Complex::new(1, 0),
        Complex::new(0, 1),
        Complex::new(-1, 0),
        Complex::new(0, -1),
    ];
    let mut rounds = rounds.clone();
    for &d in &directions {
        rounds = tilt(&rounds, d, board, blocked);
    }
    rounds
}

fn parse(input: &str) -> (Position, Position, Position) {
    let mut board = HashSet::new();
    let mut blocked = HashSet::new();
    let mut rounds = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let loc = Complex::new(i as i32, j as i32);
            board.insert(loc);
            match c {
                '#' => {
                    blocked.insert(loc);
                }
                'O' => {
                    rounds.insert(loc);
                }
                _ => {}
            }
        }
    }
    (board, blocked, rounds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count() as i32;
    let (board, blocked, rounds) = parse(input);
    let result: i32 = tilt(&rounds, Complex::new(1, 0), &board, &blocked)
        .iter()
        .map(|z| height - z.re)
        .sum();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut seen: Vec<Position> = vec![];
    let height = input.lines().count() as i32;
    let (board, blocked, rounds) = parse(input);
    let mut rounds: HashSet<Complex<i32>> = rounds;
    for i in 0.. {
        rounds = cycle(&rounds, &board, &blocked);
        if let Some(start) = seen.iter().position(|x| *x == rounds) {
            let result: i32 = seen[start + (1_000_000_000 - i) % (i - start) - 1]
                .iter()
                .map(|z| height - z.re)
                .sum();

            return Some(result as u32);
        }
        seen.push(rounds.clone());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
