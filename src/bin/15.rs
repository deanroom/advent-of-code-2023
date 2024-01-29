use std::collections::BTreeMap;

use itertools::Itertools;
use linked_hash_map::LinkedHashMap;

advent_of_code::solution!(15);

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, f| ((acc + f as u32) * 17) % 256)
}
pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .split(',')
        .map(|x| x.chars().fold(0, |acc, f| ((acc + f as u32) * 17) % 256))
        .sum::<u32>();
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: BTreeMap<u32, Box<LinkedHashMap<&str, u32>>> = BTreeMap::new();
    input.trim().split(',').for_each(|x| match x.strip_suffix('-') {
        Some(key) => {
            map.entry(hash(key) + 1)
                .or_insert(Box::new(LinkedHashMap::new()))
                .remove(key);
        }
        None => {
            let lens = x.splitn(2, '=').collect_vec();
            if lens.len() == 2 {
                let key: &str = lens[0];
                let value = lens[1].parse::<u32>().unwrap();
                map.entry(hash(key) + 1)
                    .or_insert_with(|| Box::new(LinkedHashMap::new()))
                    .entry(key)
                    .and_modify(|x: &mut u32| *x = value)
                    .or_insert(value);
            } else {
                panic!("Invalid input: |{}|", x);
            }
        }
    });
    Some(map.iter().fold(0, |acc, (k, v)| {
        acc + v.iter().enumerate().fold(0, |acc, (order, (_, focal))| {
            acc + k * (order as u32 + 1) * focal
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
