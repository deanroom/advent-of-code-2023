use cached::proc_macro::cached;
use std::time::Instant;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Springs {
    springs: Vec<i8>,
    groups: Vec<u32>,
}

impl Springs {
    fn get_composites(&self) -> u64 {
        let mut full_path: Vec<Vec<u8>> = self
            .springs
            .iter()
            .map(|x| {
                match x {
                    0 => 0..1,
                    1 => 1..2,
                    -1 => 0..2,
                    _ => panic!("wrong number."),
                }
                .collect()
            })
            .collect();
        full_path.push(vec![0]);
        get_matched(full_path, self.groups.clone(), 0)
    }
}
#[cached]
fn get_matched(num: Vec<Vec<u8>>, group: Vec<u32>, num_group: u32) -> u64 {
    let mut num_matched = 0;
    if num.len() == 0 {
        if group.len() == 0 && num_group == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    for ele in &num[0] {
        if *ele == 1 {
            num_matched += get_matched(num[1..].to_vec(), group[..].to_vec(), num_group + 1)
        } else {
            if num_group > 0 {
                if group.len() > 0 && group[0] == num_group {
                    num_matched += get_matched(num[1..].to_vec(), group[1..].to_vec(), 0)
                }
            } else {
                num_matched += get_matched(num[1..].to_vec(), group[..].to_vec(), 0)
            }
        }
    }
    num_matched
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
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let output = parse(input);
    let output = output.iter().fold(0, |acc, x| acc + x.get_composites());
    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut output = parse(input);
    output.iter_mut().for_each(|x| {
        let spring = x.springs.clone();
        let groups = x.groups.clone();

        for _ in 1..5 {
            x.springs.push(-1);
            x.springs.append(&mut spring.clone());
            x.groups.append(&mut groups.clone());
        }
    });
    let output = output.iter().enumerate().fold(0, |acc, x| {
        let start = Instant::now();

        let result = acc + x.1.get_composites();
        println!(
            "Process {}/{},cost time: {:?}",
            x.0,
            output.len(),
            start.elapsed()
        );
        result
    });
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_simple() {
        let result = part_one("?#? 1,1");
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
