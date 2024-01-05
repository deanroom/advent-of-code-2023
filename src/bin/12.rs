use std::{convert, slice::Iter};

use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Status {
    Unknown = 0,
    Operational = 1,
    Damaged = 2,
}
impl Status {
    fn guess_status(&self) -> Vec<Status> {
        if *self == Status::Unknown {
            vec![Status::Operational, Status::Damaged]
        } else {
            vec![self.clone()]
        }
    }
}

#[derive(Debug)]
struct Springs {
    springs: Vec<Status>,
    groups: Vec<u32>,
}

impl Springs {
    fn get_composites(&self) -> Vec<Vec<Status>> {
        let mut path: Vec<Status> = vec![];
        let mut output: Vec<Vec<Status>> = vec![];
        composite(&self.springs[..], &mut path, &mut output);
        output
    }
    fn get_matched(&self) -> u32 {
        let result = self.get_composites();

        let result = result
            .iter()
            .filter_map(|x| {
                // let groups: Vec<u32> = x
                //     .iter()
                //     .group_by(|x| **x == Status::Damaged)
                //     .into_iter()
                //     .filter_map(|(key, group)| {
                //         if key {
                //             let count = group.count() as u32;
                //             return Some(count);
                //         }
                //         None
                //     })
                //     .collect();
                let groups = convert_to_group(x);
                if groups.len() == self.groups.len() {
                    Some(groups)
                } else {
                    None
                }
            })
            .filter(|x| x.len() == self.groups.len())
            .filter(|x: &Vec<u32>| *x == self.groups[..])
            .count() as u32;
        result
    }
}

fn composite(springs: &[Status], path: &mut Vec<Status>, output: &mut Vec<Vec<Status>>) {
    if springs.is_empty() {
        return;
    }
    springs[0].guess_status().iter().for_each(|x| {
        path.push(x.clone());
        if springs.len() > 1 {
            composite(&springs[1..], path, output)
        } else {
            output.push(path.clone());
        }
        path.pop();
    });
}
fn convert_to_group(springs: &[Status]) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut it: Iter<'_, Status> = springs.iter();
    while let Some(status) = it.next() {
        if *status != Status::Damaged {
            continue;
        }

        let mut damaged = 1;
        damaged += eat_damage(&mut it);
        result.push(damaged);
    }
    // println!("create new group:{}/{:?}", total, result);
    result
}
fn eat_damage(it: &mut Iter<'_, Status>) -> u32 {
    let mut result = 0;
    if let Some(status) = it.next() {
        if *status == Status::Damaged {
            result = 1;
            result += eat_damage(it);
        } else {
            return 0;
        }
    }
    result
}

fn parse(input: &str) -> Vec<Springs> {
    input
        .lines()
        .map(|line| {
            let splitted_strings: Vec<&str> = line.split(' ').collect();
            if splitted_strings.len() != 2 {
                panic!("parsed failed.");
            }

            Springs {
                springs: splitted_strings[0]
                    .chars()
                    .map(|c| match c {
                        '.' => Status::Operational,
                        '#' => Status::Damaged,
                        '?' => Status::Unknown,
                        _ => panic!("parse failed,char {}", c),
                    })
                    .collect(),
                groups: splitted_strings[1]
                    .split(',')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| {
                        x.parse()
                            .expect("group number should be parsed successfully.")
                    })
                    .collect(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let output = parse(input);
    let output = output.iter().fold(0, |acc, x| acc + x.get_matched());
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = parse(input);
    let output = output.iter().fold(0, |acc, x| acc + x.get_matched());
    Some(output)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_loop() {
        for i in 1..5 {
            println!("{}", i);
        }
    }

    #[test]
    fn test_convert_group() {
        let result = convert_to_group(
            &vec![
                Status::Damaged,
                Status::Operational,
                Status::Damaged,
                Status::Operational,
                Status::Damaged,
                Status::Damaged,
                Status::Damaged,
            ][..],
        );
        assert_eq!(result, vec![1, 1, 3]);
    }

    #[test]
    fn test_composite() {
        let springs = Springs {
            springs: vec![Status::Operational, Status::Unknown],
            groups: vec![],
        };
        let result = springs.get_composites();
        assert_eq!(
            result,
            vec![
                [Status::Operational, Status::Operational],
                [Status::Operational, Status::Damaged]
            ]
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
