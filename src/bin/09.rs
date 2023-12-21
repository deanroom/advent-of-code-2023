advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i32>> {
    let output: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let line_number: Vec<i32> = line
                .split(' ')
                .map(|x| x.parse().expect("must be number."))
                .collect();
            line_number
        })
        .collect();
    output
}

fn fill_number(input: &[i32]) -> i32 {
    let mut last = *input.last().expect("must be a number.");
    let output: Vec<i32> = input
        .iter()
        .enumerate()
        .filter(|x| x.0 > 0)
        .map(|(index, num)| *num - input[index - 1])
        .collect();
    if output.iter().any(|x| *x != 0) {
        last += fill_number(&output)
    }
    last
}
fn fill_number_two(input: &[i32]) -> i32 {
    let mut first: i32 = *input.first().expect("must be a number.");
    let output: Vec<i32> = input
        .iter()
        .enumerate()
        .filter(|x| x.0 > 0)
        .map(|(index, num)| *num - input[index - 1])
        .collect();
    if output.iter().any(|x| *x != 0) {
        first += -fill_number_two(&output);
    }
    first
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let result = input.iter().fold(0, |acc, x| {
        let output = fill_number(x);
        acc + output
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse(input);
    let result = input.iter().fold(0, |acc, x| {
        let output = fill_number_two(x);
        acc + output
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_number() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let input = parse(input);
        input.iter().for_each(|x| {
            let output = fill_number(x);
            println!("{}", output);
        });
    }

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let output = parse(input);
        println!("{:?}", output);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
