advent_of_code::solution!(3);

#[derive(Default)]
struct Number {
    start: u32,
    end: u32,
    value: u32,
}

#[derive(Default)]
struct LineData {
    numbers: Vec<Number>,
    symbols: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_data: Vec<LineData> = input
        .lines()
        .map(|line| {
            let mut data: LineData = LineData::default();
            let mut index = 0;
            let mut it = line.chars();
            while let Some(mut c) = it.next() {
                let mut number = Number::default();
                if c.is_numeric() {
                    number.start = index;
                    // new add line.
                    number.end = index;
                    number.value = c.to_digit(10).expect("Char must be number.");
                    loop {
                        match it.next() {
                            Some(char) => {
                                if !char.is_numeric() {
                                    //TODO make mistake to add number here.
                                    number.end = index;
                                    c = char;
                                    index += 1;
                                    break;
                                }
                                number.value = number.value * 10
                                    + char.to_digit(10).expect("Char must be number.");
                                    // make mistake to calculate wrong end index.
                                number.end = index;
                                index += 1;
                            }
                            None => {
                                index += 1;
                                break;
                            }
                        }
                    }
                }
                if number.value > 0 {
                    // make mistake before to push number too late.
                    data.numbers.push(number);
                }

                if c == '.' {
                } else {
                    data.symbols.push(index);
                }
                index += 1;
            }
            data
        })
        .collect();

    let mut valid_numbers: Vec<u32> = vec![];

    for (index, data) in line_data.iter().enumerate() {
        let mut line_numbers: Vec<u32> = vec![];
        let mut preview_line = &LineData::default();
        let mut next_line = &LineData::default();
        if index > 0 {
            preview_line = &line_data[index - 1];
        }

        if index < line_data.len() - 1 {
            next_line = &line_data[index + 1];
        }

        for number in data.numbers.iter() {
            if data
                .symbols
                .iter()
                .any(|symbol| *symbol + 1 == number.start || *symbol == number.end + 1)
            {
                valid_numbers.push(number.value);
                line_numbers.push(number.value);
                continue;
            }

            if preview_line
                .symbols
                .iter()
                .any(|symbol| *symbol + 1 >= number.start && *symbol <= number.end + 1)
            {
                valid_numbers.push(number.value);
                line_numbers.push(number.value);

                continue;
            }

            if next_line
                .symbols
                .iter()
                .any(|symbol| *symbol + 1 >= number.start && *symbol <= number.end + 1)
            {
                valid_numbers.push(number.value);
                line_numbers.push(number.value);
                continue;
            }
        }
        println!("{:?}", line_numbers);
    }
    Some(valid_numbers.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
