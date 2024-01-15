use std::{borrow::Borrow, convert, slice::Iter, collections::VecDeque};

use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Springs {
    springs: Vec<i8>,
    groups: Vec<u32>,
    group_count: u32,
}

impl Springs {
    fn get_composites(&self) -> u32 {
        // 我们通过以下方式寻找所有的组合-路径：
        // 0，定义每一个可以表示【好】、【坏】、【好或者坏】的所有元素构成一个数组，每种取值为一个状态，每个元素可以有一种或者两种状态，
        // 1. 对于每一个元素，其自身最多有两种状态，通过选择所有元素的第一个状态，可以建立一条路径
        // 3. 从这个数组末尾开始，如果其是一种状态，那么我们最开始建立的路径就已经覆盖，我们可以处理倒数第二个元素；
        //   如果这个元素是两种状态，那么我们就需要使用第一个路径当前元素之前元素构成路径+当前元素的第二种状态作为第二条路径.
        // . 这样我们就有了第二条路径
        // 4. 接下来我们处理倒数第二个元素，我们仍然是根据其有两种还是一种状态来处理，如果是一种状态，我们之前谈到的第一条路径和第二条路径其实已经包含了
        //  如果两种状态，那么我们就用当前状态加上前面两条路径分别作为新的两条路径。
        // 5. 继续以上第四步，依次类推，最终直到处理完第一个元素后，我们就完成了所有路径。
        let full_path: Vec<Vec<u8>> = self
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
        let path = full_path
            .iter()
            .map(|x| x.first().expect("number").to_string())
            .collect::<String>();
        let path = u128::from_str_radix(&path,2).expect("msg");
            

        let mut stack_path: VecDeque<u128> =VecDeque::new();
        stack_path.push_back(path);
        let mut loop_index = full_path.len() - 1;
        loop {
            if full_path[loop_index].len() == 1 {
                if loop_index == 0 {
                    break;
                }
                loop_index -= 1;
                continue;
            }
            let previous_stack = stack_path.iter();
            let mut new_path: u128;
            let mut new_stack = VecDeque::new();;
            for path in previous_stack {
                // new_path = vec![];
                // for (index, node) in path.iter().enumerate() {
                //     if index == loop_index {
                //         new_path.push(&1)
                //     } else {
                //         new_path.push(&node);
                //     }
                // }
                new_path = *path;
                new_stack.push_back(new_path)
            }
            println!(
                "Path Loop Index: {:?}/{},with stack count: {}",
                loop_index,
                full_path.len(),
                new_stack.len()
            );
            stack_path.append(&mut new_stack);

            if loop_index == 0 {
                break;
            }
            loop_index -= 1;
        }
        // let result = stack_path
        //     .iter()
        //     .filter(|x| convert_to_group(&x) == self.groups)
        //     .count();
        // stack_path
        //     .iter()
        //     .enumerate()
        //     .for_each(|x| println!("composite: {:?}, group: {:?}", x, convert_to_group(x.1)));
        0
    }
}

fn convert_to_group(springs: &[&i8]) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut it: Iter<'_, &i8> = springs.iter();
    while let Some(status) = it.next() {
        if **status != 1 {
            continue;
        }

        let mut damaged = 1;
        damaged += eat_damage(&mut it);
        result.push(damaged);
    }
    result
}
fn eat_damage(it: &mut Iter<'_, &i8>) -> u32 {
    let mut result = 0;
    if let Some(status) = it.next() {
        if **status == 1 {
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

    let output = output
        .iter()
        .enumerate()
        .fold(0, |acc, x| acc + x.1.get_composites());
    Some(output)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_part_simple() {
        let result = part_one("?#? 1,1");
    }

    use super::*;
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
