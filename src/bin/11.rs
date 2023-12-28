advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Space,
    Star,
}

fn parse(input: &str) -> Vec<Vec<Symbol>> {
    let output = input
        .lines()
        .into_iter()
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

fn expand(input: Vec<Vec<Symbol>>) -> Vec<Vec<Symbol>> {
    let mut ouput: Vec<Vec<Symbol>> = vec![];

    //expand row
    for line in input {
        if !line.iter().any(|x| x == &Symbol::Star) {
            ouput.push(line.clone());
        }
        ouput.push(line);
    }

    //rotate row and column
    let num_rows = ouput.len();
    let num_cols = ouput[0].len();

    let mut matrix: Vec<Vec<Symbol>> = vec![vec![Symbol::Space; num_rows]; num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            matrix[j][i] = ouput[i][j];
        }
    }
    let mut ouput: Vec<Vec<Symbol>> = vec![];

    //expand column by expand row.
    for line in matrix {
        if !line.iter().any(|x| x == &Symbol::Star) {
            ouput.push(line.clone());
        }
        ouput.push(line);
    }

    //rotate back again to keep data clear.
    let num_rows = ouput.len();
    let num_cols = ouput[0].len();

    let mut matrix: Vec<Vec<Symbol>> = vec![vec![Symbol::Space; num_rows]; num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            matrix[j][i] = ouput[i][j];
        }
    }

    matrix
}

fn expand_part2(input: Vec<Vec<Symbol>>, expansion: u32) -> Vec<Vec<Symbol>> {
    let mut ouput: Vec<Vec<Symbol>> = vec![];

    println!("expand row");
    //expand row
    for line in input {
        if !line.iter().any(|x| x == &Symbol::Star) {
            for _ in 0..expansion-1 {
                ouput.push(line.clone());
            }
        }
        ouput.push(line);
    }

    println!("rotate row");
    //rotate row and column
    let num_rows = ouput.len();
    let num_cols = ouput[0].len();

    let mut matrix: Vec<Vec<Symbol>> = vec![vec![Symbol::Space; num_rows]; num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            matrix[j][i] = ouput[i][j];
        }
    }
    let mut ouput: Vec<Vec<Symbol>> = vec![];

    println!("expand column");
    //expand column by expand row.
    for line in matrix {
        if !line.iter().any(|x| x == &Symbol::Star) {
            for _ in 0..expansion-1 {
                ouput.push(line.clone());
            }
        }
        ouput.push(line);
    }

    println!("rotate column");
    //rotate back again to keep data clear.
    let num_rows = ouput.len();
    let num_cols = ouput[0].len();

    let mut matrix: Vec<Vec<Symbol>> = vec![vec![Symbol::Space; num_rows]; num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            matrix[j][i] = ouput[i][j];
        }
    }

    matrix
}

fn find_stars(input: Vec<Vec<Symbol>>) -> Vec<(i32, i32)> {
    println!("find_stars");
    let mut output: Vec<(i32, i32)> = vec![];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == Symbol::Star {
                output.push((i as i32, j as i32));
            }
        }
    }
    output
}

fn get_pairs(input: Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    println!("get_pairs");
    let mut output: Vec<((i32, i32), (i32, i32))> = vec![];
    for ele_i in input.iter() {
        for ele_j in input.iter() {
            if ele_i != ele_j {
                if !output.contains(&(*ele_j, *ele_i)) {
                    output.push((ele_i.clone(), (*ele_j).clone()));
                }
            }
        }
    }
    output
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let input = expand(input);
    let input = find_stars(input);
    let input = get_pairs(input);

    let result: i32 = input
        .iter()
        .map(|point| (point.0 .0 - point.1 .0).abs() + (point.0 .1 - point.1 .1).abs())
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse(input);
    let input = expand_part2(input, 1000000);
    let input = find_stars(input);
    let input = get_pairs(input);

    let result: i32 = input
        .iter()
        .map(|point| (point.0 .0 - point.1 .0).abs() + (point.0 .1 - point.1 .1).abs())
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
        let _ = expand(output);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let input = parse(input);
        let input = expand_part2(input, 10);
        let input = find_stars(input);
        let input = get_pairs(input);

        let result: i32 = input
            .iter()
            .map(|point| (point.0 .0 - point.1 .0).abs() + (point.0 .1 - point.1 .1).abs())
            .sum();
        assert_eq!(result, 1030);
    }
}
