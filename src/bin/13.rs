use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    Some(match_mirror(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(match_mirror(input, 1))
}

fn find_mirror(a: &[Vec<bool>], mismatch: usize) -> Option<usize> {
    for i in 1..a.len() {
        let n = i.min(a.len() - i);
        let left = &a[..i].iter().rev().take(n).collect_vec();
        let right = &a[i..].iter().take(n).collect_vec();
        if left
            .iter()
            .zip(right)
            .map(|(x, y)| {
                let count: usize = x
                    .iter()
                    .zip(*y)
                    .map(|(a, b)| if *a ^ *b { 1 } else { 0 })
                    .sum::<usize>();
                count
            })
            .sum::<usize>()
            == mismatch
        {
            return Some(i);
        }
    }
    None
}

pub fn match_mirror(input: &str, mismatch: usize) -> u32 {
    let mut total = 0;
    input.split("\n\n").for_each(|x| {
        let matrix = x
            .lines()
            .map(|x| {
                x.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid input"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        if let Some(output) = find_mirror(&matrix, mismatch) {
            total += output * 100;
        } else {
            let transposed_matrix: Vec<Vec<bool>> = (0..matrix[0].len())
                .map(|col| (0..matrix.len()).map(|row| matrix[row][col]).collect())
                .collect();

            if let Some(output) = find_mirror(&transposed_matrix, mismatch) {
                total += output;
            } else {
                print!("No output found");
            }
        }
    });
    total as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
