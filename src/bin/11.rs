use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Space,
    Star,
}

#[derive(Default, Debug)]
struct Star {
    x: i64,
    y: i64,
    e_x: i64,
    e_y: i64,
}

impl Clone for Star {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            e_x: self.e_x,
            e_y: self.e_y,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
impl Star {
    fn new(x: i64, y: i64) -> Star {
        Star {
            x,
            y,
            e_x: x,
            e_y: y,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Symbol>> {
    let output = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Symbol::Space,
                    '#' => Symbol::Star,
                    _ => panic!("wrong char.{}", c),
                })
                .collect()
        })
        .collect();
    output
}

fn expand(input: Vec<Star>, expansion: i64) -> Vec<Star> {
    let expansion = expansion - 1;
    let mut universe_original: Vec<Star> = input;

    universe_original.sort_unstable_by_key(|x| x.x);
    let mut universe_expanded: Vec<Star> = universe_original.clone();

    let axis_x: Vec<i64> = universe_original
        .iter()
        .unique_by(|x| x.x)
        .map(|x| x.x)
        .collect();

    let mut axis_prev = 0;
    //expand row
    for value in axis_x.iter() {
        let mut distance = value - axis_prev;
        if distance >= 1 {
            distance = (distance - 1) * expansion;
        }
        axis_prev = *value;
        for x in universe_expanded.iter_mut().filter(|x| x.x >= *value) {
            x.e_x = x.e_x + distance;
        }
    }
    universe_original.sort_unstable_by_key(|x| x.y);
    universe_expanded.sort_unstable_by_key(|x| x.y);

    let axis_y: Vec<i64> = universe_original
        .iter()
        .unique_by(|x| x.y)
        .map(|x| x.y)
        .collect();
    let mut axis_prev = 0;
    //expand row
    for value in axis_y.iter() {
        let mut distance = value - axis_prev;
        if distance >= 1 {
            distance = (distance - 1) * expansion;
        }
        axis_prev = *value;
        for x in universe_expanded.iter_mut().filter(|x| x.y >= *value) {
            x.e_y = x.e_y + distance;
        }
    }

    universe_expanded
}

fn find_stars(input: Vec<Vec<Symbol>>) -> Vec<Star> {
    let mut output: Vec<Star> = vec![];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == Symbol::Star {
                output.push(Star::new(j as i64, i as i64));
            }
        }
    }
    output
}

fn get_pairs(input: Vec<Star>) -> Vec<(Star, Star)> {
    let mut output: Vec<(Star, Star)> = vec![];

    for i in 1..input.len() {
        let mut pair: Vec<(Star, Star)> = input
            .iter()
            .zip(&input[i..])
            .map(|x| (x.0.clone(), x.1.clone()))
            .collect();
        output.append(&mut pair)
    }
    output
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    let input = find_stars(input);
    let input = expand(input, 2);
    let input = get_pairs(input);

    let result: i64 = input
        .iter()
        .map(|point| (point.0.e_x - point.1.e_x).abs() + (point.0.e_y - point.1.e_y).abs())
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    part_two_with_parmeter(input, 1000000)
}

pub fn part_two_with_parmeter(input: &str, expansion: i64) -> Option<i64> {
    let input = parse(input);
    let input = find_stars(input);
    let input = expand(input, expansion);
    let input = get_pairs(input);

    let result: i64 = input
        .iter()
        .map(|point| (point.0.e_x - point.1.e_x).abs() + (point.0.e_y - point.1.e_y).abs())
        .sum();
    Some(result)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let result = &advent_of_code::template::read_file("examples", DAY);

        let output = parse(result);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two_with_parmeter(&advent_of_code::template::read_file("examples", DAY), 10);

        assert_eq!(result, Some(1030));
    }
}
