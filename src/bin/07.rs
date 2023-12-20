advent_of_code::solution!(7);

use std::cmp::Ordering;


use nom::character::complete::{self, alphanumeric1, space1};

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

            for c in it.by_ref() {
                if c == group.chars().last().expect("a char") {
                    group.push(c);
                } else {
                    self.groups.push(group);
                    group = String::new();
                    group.push(c)
                }
            }
            if it.next().is_none() {
                self.groups.push(group)
            }
        }
        self.groups
            .sort_unstable_by(|a, b| a.len().cmp(&b.len()).reverse());
        self.hand = self.groups.join("");
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
    let (input, (hand, _, bid)) = tuple((alphanumeric1, space1, complete::u32))(input)?;

    Ok((input, CamelCard::new(hand.to_string(), bid)))
}
fn parse(input: &str) -> Vec<CamelCard> {
    let cards = input
        .lines()
        .map(|_line| {
            let (_input, card) = parse_line(_line).expect("Valid parse line.");
            card
        })
        .collect();
    cards
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut cards = parse(input);
    cards.sort_unstable_by_key(|a| a.power);

    cards.sort_by(|a, b| {
        let zip: Vec<(char, char)> = a
            .hand
            .chars()
            .zip(b.hand.chars())
            .collect();
        let mut order = Ordering::Equal;
        if a.power > b.power {
            order = Ordering::Greater
        } else if a.power < b.power {
            order = Ordering::Less
        } else {
            for (x, y) in zip {
                if a.get_order(&x) > b.get_order(&y) {
                    order = Ordering::Greater;
                    break;
                } else if a.get_order(&x) < b.get_order(&y) {
                    order = Ordering::Less;
                    break;
                }
            }
        }
        order
    });

    for (index, ele) in cards.iter_mut().enumerate() {
        ele.rank = index as u32 + 1;
    }

    let result = cards.iter().map(|x| x.bid * x.rank).sum::<u32>();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut cards = parse(&input);
        cards.sort_unstable_by_key(|a| a.power);

        for (index, ele) in cards.iter_mut().enumerate() {
            ele.rank = index as u32 + 1;
        }

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
