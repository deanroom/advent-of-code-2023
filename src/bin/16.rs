use std::collections::HashSet;

use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use num_complex::Complex;
advent_of_code::solution!(16);

fn get_next_positions(
    position: (Complex<i32>, Complex<i32>, char),
) -> Vec<(Complex<i32>, Complex<i32>)> {
    let (pos, dir, c) = position;
    let mut positions = vec![];
    let mut dirs = vec![dir];
    match c {
        '|' => {
            if dir.re != 0 {
                dirs = vec![Complex::new(0, 1), Complex::new(0, -1)];
            }
        }
        '-' => {
            if dir.im != 0 {
                dirs = vec![Complex::new(1, 0), Complex::new(-1, 0)];
            }
        }
        '/' => {
            dirs = vec![(dir * Complex::new(0, 1)).conj()];
        }
        '\\' => {
            dirs = vec![(dir * Complex::new(0, -1)).conj()];
        }
        _ => {
            dirs = vec![dir];
        }
    }

    dirs.iter().for_each(|&d| {
        positions.push((pos + d, d));
    });

    positions
}

fn get_positions(
    position: (Complex<i32>, Complex<i32>),
    map: &LinkedHashMap<Complex<i32>, char>,
    seen: &mut HashSet<(Complex<i32>, Complex<i32>)>,
) {
    if let Some(c) = map.get(&position.0) {
        if !seen.insert(position) {
            return;
        }
        let positions = get_next_positions((position.0, position.1, *c));
        if positions.is_empty() {
            return;
        }
        for (pos, dir) in positions {
            get_positions((pos, dir), map, seen);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: LinkedHashMap<Complex<i32>, char> = LinkedHashMap::new();
    let mut seen: HashSet<(Complex<i32>, Complex<i32>)> = HashSet::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            map.insert(Complex::new(col as i32, row as i32), c);
        })
    });
    let start_position = Complex::new(0, 0);
    let start_dir = Complex::new(1, 0);
    get_positions((start_position, start_dir), &map, &mut seen);

    Some(
        seen.iter()
            .map(|x: &(Complex<i32>, Complex<i32>)| x.0)
            .unique()
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: LinkedHashMap<Complex<i32>, char> = LinkedHashMap::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            map.insert(Complex::new(col as i32, row as i32), c);
        })
    });
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    let mut start_positions = HashSet::new();

    map.keys().filter(|x| x.re == 0).for_each(|k| {
        start_positions.insert((k, Complex::new(1, 0)));
    });

    map.keys().filter(|x| x.re == width - 1).for_each(|k| {
        start_positions.insert((k, Complex::new(-1, 0)));
    });

    map.keys().filter(|x| x.im == 0).for_each(|k| {
        start_positions.insert((k, Complex::new(0, 1)));
    });

    map.keys().filter(|x| x.im == height - 1).for_each(|k| {
        start_positions.insert((k, Complex::new(0, -1)));
    });
    let output = start_positions
        .iter()
        .map(|(&pos, dir)| {
            let mut seen: HashSet<(Complex<i32>, Complex<i32>)> = HashSet::new();
            get_positions((pos, *dir), &map, &mut seen);
            let output = seen.iter().map(|x| x.0).unique().count() as u32;
            output
        })
        .max()
        .unwrap();

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
