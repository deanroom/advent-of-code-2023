advent_of_code::solution!(6);

use nom::{
    character::complete::{self, multispace1, newline, space1},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn get_wins(&self) -> u32 {
        let mut sum = 0;
        for hold in 1..self.time {
            if (self.time - hold) * hold > self.distance {
                sum += 1;
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, races) = parse(input).expect("A valid parse");
    let races = gen_races(races);
    let sum = races.iter().fold(1, |acc, x| acc * x.get_wins());
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, races) = parse(input).expect("A valid parse");
    let races = gen_races(races);
    let time: u32 = races
        .iter()
        .fold(String::new(), |acc, x| acc + &x.time.to_string())
        .parse()
        .expect("A number.");

    let distance = races
        .iter()
        .fold(String::new(), |acc, x| acc + &x.time.to_string())
        .parse()
        .expect("A number.");

    let mut sum = 0;
    for hold in 1..time {
        if (time - hold) * hold > distance {
            sum += 1;
        }
    }
    Some(sum)
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, time) = tag("Time:")
        .precedes(multispace1)
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)?;

    let (input, distance) = newline
        .precedes(tag("Distance:"))
        .precedes(multispace1)
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)?;
    Ok((input, (time, distance)))
}

fn gen_races(input: (Vec<u32>, Vec<u32>)) -> Vec<Race> {
    let races: Vec<Race> = input
        .0
        .iter()
        .zip(input.1.iter())
        .map(|item| Race {
            time: *item.0,
            distance: *item.1,
        })
        .collect();
    races
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (_input, (time, distance)) = parse(input).expect("A valid parse");
        assert_eq!(time, vec![7, 15, 30]);
        assert_eq!(distance, vec![9, 40, 200]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
