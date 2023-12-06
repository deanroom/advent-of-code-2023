advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let output: u32 = input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter_map(|character| character.to_digit(10));

            let first = numbers.next().unwrap();
            match numbers.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum();
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    let output: u32 = input
        .lines()
        .map(|line| {
            let mut numbers = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let char = numbers
                    .iter()
                    .find(|number| reduced_line.starts_with(number.0));
                let result = match char {
                    Some(number) => number.1,
                    None => reduced_line.chars().next().unwrap(),
                };
                result.to_digit(10)
            });

            let first: u32 = numbers.next().unwrap();
            match numbers.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum();
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result.expect("There must be numbers"), 281);
    }
}
