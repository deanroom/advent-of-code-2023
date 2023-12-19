advent_of_code::solution!(7);

use nom::bytes::complete::take_while1;
use nom::character::complete::{self, alphanumeric1, digit1, line_ending, space1};
use nom::multi::{many1, many_till};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
// enum Rank {
//     FiveOfAKind,
//     FourOfAKing,
//     FullHouse,
//     ThreeOfAKind,
//     TwoPair,
//     OnePair,
//     HighCard,
//     None,
// }

struct CamelCard {
    //TODO string or str
    hand: String,
    bid: u32,
    groups: Vec<String>,
    power: u8,
    rank: u32,
}

impl<'a> CamelCard {
    fn new(hand: String, bid: u32) -> CamelCard {
        let mut card = CamelCard {
            hand,
            bid,
            groups: vec![],
            power: 0,
            rank: 0,
        };
        card.order_cards();
        card.group();
        card.power();
        card
    }

    fn order_cards(&mut self) {
        let mut chars: Vec<char> = self.hand.chars().collect();
        chars.sort_unstable_by(|a, b| self.get_order(a).cmp(&self.get_order(b)).reverse());
        self.hand = chars.into_iter().collect();
    }
   
    fn get_order(&self, c: &char) -> u8 {
        match c {
            'A' => 14,
            'Q' => 13,
            'K' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).expect("should be number") as u8,
        }
    }
    fn group(&mut self) {
        let mut it = self.hand.chars();
        while let Some(next) = it.next() {
            let mut group = String::new();
            group.push(next);

            while let Some(c) = it.next() {
                if c == group.chars().last().expect("a char") {
                    group.push(c);
                } else {
                    self.groups.push(group);
                    group = String::new();
                    group.push(c)
                }
            }
            if it.next() == None {
                self.groups.push(group)
            }
        }
        self.groups
            .sort_unstable_by(|a, b| a.len().cmp(&b.len()).reverse());
    }
    fn power(&mut self) {
        let max_len = self
            .groups
            .iter()
            .max_by_key(|x| x.len())
            .expect("group")
            .len();

        self.power = match max_len {
            5 => 7,
            4 => 6,
            3 => {
                if self.groups.len() == 2 {
                    5
                } else {
                    4
                }
            }
            2 => {
                if self.groups.len() == 3 {
                    3
                } else {
                    2
                }
            }
            1 => 1,
            _ => panic!("WRONG."),
        };
    }
}

fn parse_line(input: &str) -> IResult<&str, CamelCard> {
    let (input, (hand, _, bid, _)) =
        tuple((alphanumeric1, space1, complete::u32, line_ending))(input)?;

    Ok((input, CamelCard::new(hand.to_string(), bid)))
}
fn parse(input: &str) -> IResult<&str, Vec<CamelCard>> {
    let (input, cards) = many1(parse_line)(input)?;
    Ok((input, cards))
}
pub fn part_one(input: &str) -> Option<u32> {
    let input = &advent_of_code::template::read_file("examples", DAY);
    let (_, cards) = parse(&input).expect("Should parse successfully.");
    let mut cards = cards;
    cards.sort_unstable_by_key(|a| a.power);

    for (index, ele) in cards.iter_mut().enumerate() {
        ele.rank = index as u32 + 1;
    }
    let result = cards.iter().map(|x| x.bid * x.rank).sum::<u32>();

    println!("{:?}", cards);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (_, cards) = parse(&input).expect("Should parse successfully.");
        let mut cards = cards;
        cards.sort_unstable_by_key(|a| a.power);

        for (index, ele) in cards.iter_mut().enumerate() {
            ele.rank = index as u32 + 1;
        }

        println!("{:?}", cards);
        assert_eq!(cards.len(), 5);
    }

    #[test]
    fn test_sort() {
        let mut card = CamelCard::new("A1K2J".to_string(), 10);
        card.order_cards();
        assert_eq!(card.hand, "AKJ21");
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
