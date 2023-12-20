advent_of_code::solution!(7);

use nom::character::complete::{self, alphanumeric1, space1};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum Rank {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    None = 0,
}
struct CamelCard {
    //TODO string or str
    hand: String,
    origin_hand: String,
    bid: u32,
    groups: Vec<String>,
    power: Rank,
    power2: Rank,
    score_hand: (u8, u8, u8, u8, u8),
    rank: u32,
}

impl CamelCard {
    fn new(hand: String, bid: u32) -> CamelCard {
        let mut card = CamelCard {
            hand: String::from(&hand),
            bid,
            groups: vec![],
            power: Rank::None,
            power2: Rank::None,
            rank: 0,
            origin_hand: hand,
            score_hand: (0, 0, 0, 0, 0),
        };
        card.order_cards();
        let mut score = card.origin_hand.chars().map(|c| card.get_order(&c));
        card.score_hand = (
            score.next().unwrap(),
            score.next().unwrap(),
            score.next().unwrap(),
            score.next().unwrap(),
            score.next().unwrap(),
        );
        card.group();
        card.power();
        card.power2();
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
            'K' => 13,
            'Q' => 12,
            //part1 use j to 11
            'J' => 1,
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
            5 => Rank::FiveOfAKind,
            4 => Rank::FourOfAKind,
            3 => {
                if self.groups.len() == 2 {
                    Rank::FullHouse
                } else {
                    Rank::ThreeOfAKind
                }
            }
            2 => {
                if self.groups.len() == 3 {
                    Rank::TwoPair
                } else {
                    Rank::OnePair
                }
            }
            1 => Rank::HighCard,
            _ => panic!("WRONG."),
        };
    }
    fn power2(&mut self) {
        let max_len = self
            .groups
            .iter()
            .max_by_key(|x| x.len())
            .expect("group")
            .len();
        let joker = "J";
        self.power2 = match max_len {
            5 => Rank::FiveOfAKind,
            4 => {
                if self.hand.contains(joker) {
                    Rank::FiveOfAKind
                } else {
                    Rank::FourOfAKind
                }
            }
            3 => {
                if self.groups.len() == 2 {
                    if self.hand.contains(joker) {
                        Rank::FiveOfAKind
                    } else {
                        Rank::FullHouse
                    }
                } else if self.hand.contains(joker) {
                    Rank::FourOfAKind
                } else {
                    Rank::ThreeOfAKind
                }
            }
            2 => {
                if self.groups.len() == 3 {
                    if self.groups[0].contains(joker) || self.groups[1].contains(joker) {
                        Rank::FourOfAKind
                    } else if self.groups[2].contains(joker) {
                        Rank::FullHouse
                    } else {
                        Rank::TwoPair
                    }
                } else if self.hand.contains(joker) {
                    Rank::ThreeOfAKind
                } else {
                    Rank::OnePair
                }
            }
            1 => {
                if self.hand.contains(joker) {
                    Rank::OnePair
                } else {
                    Rank::HighCard
                }
            }
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
    cards.sort_unstable_by_key(|x| (x.power as u8, x.score_hand));

    for (index, ele) in cards.iter_mut().enumerate() {
        ele.rank = index as u32 + 1;
    }
    // cards.iter().for_each(|card| {
    //     println!(
    //         "{},{:?},{},{}",
    //         card.origin_hand, card.power, card.bid, card.rank
    //     )
    // });

    let result = cards.iter().map(|x| x.bid * x.rank).sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = parse(input);
    cards.sort_unstable_by_key(|x| (x.power2 as u8, x.score_hand));

    for (index, ele) in cards.iter_mut().enumerate() {
        ele.rank = index as u32 + 1;
    }
    // cards.iter().for_each(|card| {
    //     println!(
    //         "{},{:?},{},{}",
    //         card.origin_hand, card.power2, card.bid, card.rank
    //     )
    // });

    let result = cards.iter().map(|x| x.bid * x.rank).sum::<u32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut cards = parse(input);
        cards.sort_unstable_by_key(|a| a.power as u8);

        for (index, ele) in cards.iter_mut().enumerate() {
            ele.rank = index as u32 + 1;
        }

        assert_eq!(cards.len(), 5);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
