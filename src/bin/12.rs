use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::VecDeque, slice::Iter, time::Instant};

advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Eq, Clone,Copy)]
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
    group_count: u32,
}

impl Springs {
    fn get_matched(&self) -> u32 {
        let now = Instant::now();
        let result = self.get_composites();
        println!(
            "got composites: {:?},with count {}",
            now.elapsed().as_millis(),
            result.len()
        );
        let result: Vec<Vec<u32>> = result
            .iter()
            .filter_map(|x| {
                let groups = convert_to_group(x);
                if groups.len() == self.groups.len() {
                    Some(groups)
                } else {
                    None
                }
            })
            .collect();
        println!(
            "got group: {:?}, with count {}",
            now.elapsed().as_millis(),
            result.len()
        );
        let result = result.iter().filter(|x| **x == self.groups[..]).count() as u32;
        println!(
            "got match count: {:?}, with count {}",
            now.elapsed().as_millis(),
            result
        );

        result
    }
    fn get_composites(&self) -> Vec<Vec<Status>> {
        let mut path: Vec<Status> = Vec::new();
        let mut output: Vec<Vec<Status>> = vec![];
        self.composite(&self.springs[..], &mut path, &mut output);
        output
    }
    fn composite(&self, springs: &[Status], path: &mut Vec<Status>, output: &mut Vec<Vec<Status>>) {
        if springs.is_empty() {
            return;
        }
        springs[0].guess_status().iter().for_each(|x| {
            path.push(x.clone());
            let tmp_group = convert_to_group(&path[..]);

            // println!(
            //     "tmp_groups: {:?}, slice: {:?}",
            //     tmp_group,
            //     &self.groups[0..tmp_group.len()]
            // );
            if springs.len() > 1 {
                if self.compare_group(&tmp_group) {
                    self.composite(&springs[1..], path, output)
                }
            } else {
                output.push(path.clone());
            }

            path.pop();
        });
    }
    fn compare_group(&self, other_group: &[u32]) -> bool {
        if other_group.len() > self.groups.len() {
            return false;
        }

        if self.groups.len() > 1 && other_group.len() > 1 {
            if self.groups[0..other_group.len() - 1] == other_group[..other_group.len() - 1] {
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

fn convert_to_group(springs: &[Status]) -> Vec<u32> {
    // let groups: Vec<u32> = springs
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
    // groups
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
            let groups: Vec<u32> = splitted_strings[1]
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| {
                    x.parse()
                        .expect("group number should be parsed successfully.")
                })
                .collect();
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
                groups: groups,
                group_count: 0,
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
    let mut output = parse(input);
    output.iter_mut().for_each(|x| {
        let spring = x.springs.clone();
        let groups = x.groups.clone();

        for _ in 1..5 {
            x.springs.push(Status::Unknown);
            x.springs.append(&mut spring.clone());
            x.groups.append(&mut groups.clone());
            x.group_count = x.groups.iter().sum();
        }
    });
    let output = output.iter().fold(0, |acc, x| acc + x.get_matched());
    Some(output)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

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
            group_count: 0,
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
        assert_eq!(result, Some(18902));
    }
}
