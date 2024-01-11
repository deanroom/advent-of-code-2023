use std::{collections::btree_map::Range, slice::Iter, time::Instant};

advent_of_code::solution!(12);

#[derive(Debug)]
struct Springs {
    springs: Vec<i8>,
    groups: Vec<u32>,
    group_count: u32,
}

impl Springs {
    fn get_composites(&self) -> u32 {
        let mut path = Vec::new();
        let mut output: u32 = 0;
        self.composite(&self.springs[..], &mut path, &mut output);
        output
    }
    fn composite(&self, springs: &[i8], path: &mut Vec<i8>, output: &mut u32) {
        if springs.is_empty() {
            return;
        }
        let range = match springs[0] {
            0 => 0..1,
            1 => 1..2,
            -1 => 0..2,
            _ => panic!("wrong number."),
        };

        for x in range {
            path.push(x);
            let tmp_group = convert_to_group(&path[..]);

            if springs.len() > 1 {
                if self.compare_group(&tmp_group) {
                    self.composite(&springs[1..], path, output)
                }
            } else {
                println!("{}==={}", tmp_group.len(), path.len());
                if self.groups.len() == tmp_group.len() {
                    if self.groups == tmp_group {
                        *output += 1;
                        // println!("{}", output);
                    }
                }
            }

            path.pop();
        }
    }
    fn compare_group(&self, other_group: &[u32]) -> bool {
        if other_group.len() > self.groups.len() {
            return false;
        }

        if self.groups.len() > 1 && other_group.len() > 1 {
            if other_group.iter().max() > self.groups.iter().max() {
                return false;
            }

            if other_group.iter().sum::<u32>() > self.groups.iter().sum() {
                return false;
            }

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

fn convert_to_group(springs: &[i8]) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut it: Iter<'_, i8> = springs.iter();
    while let Some(status) = it.next() {
        if *status != 1 {
            continue;
        }

        let mut damaged = 1;
        damaged += eat_damage(&mut it);
        result.push(damaged);
    }
    result
}
fn eat_damage(it: &mut Iter<'_, i8>) -> u32 {
    let mut result = 0;
    if let Some(status) = it.next() {
        if *status == 1 {
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
                        '.' => 0,
                        '#' => 1,
                        '?' => -1,
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
    let output = output.iter().fold(0, |acc, x| acc + x.get_composites());
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut output = parse(input);
    output.iter_mut().for_each(|x| {
        let spring = x.springs.clone();
        let groups = x.groups.clone();

        for _ in 1..5 {
            x.springs.push(-1);
            x.springs.append(&mut spring.clone());
            x.groups.append(&mut groups.clone());
            x.group_count = x.groups.iter().sum();
        }
    });

    let output = output.iter().fold(0, |acc, x| acc + x.get_composites());
    Some(output)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    // #[test]
    // fn test_convert_group() {
    //     let result = convert_to_group(
    //         &vec![
    //             Status::Damaged,
    //             Status::Operational,
    //             Status::Damaged,
    //             Status::Operational,
    //             Status::Damaged,
    //             Status::Damaged,
    //             Status::Damaged,
    //         ][..],
    //     );
    //     assert_eq!(result, vec![1, 1, 3]);
    // }
    #[test]
    fn test_range() {
        for i in 0..1 {
            println!("{}", i);
        }
        println!("end");

        for i in 1..2 {
            println!("{}", i);
        }
        println!("end");

        for i in 0..2 {
            println!("{}", i);
        }
        println!("end");
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
