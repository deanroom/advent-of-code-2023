advent_of_code::solution!(7);

use nom::character::complete::{self, alphanumeric1, digit1, line_ending, space1};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
enum Rank {
    FiveOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
#[derive(Debug)]
struct CamelCard {
    //TODO string or str
    hand: String,
    bid: u32,
}

impl<'a> CamelCard {
    fn new(hand: &str, bid: u32) -> CamelCard {
        let mut card = CamelCard {
            hand: String::from(hand),
            bid,
        };
        card.order_cards();
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

    fn rank(&self) -> Rank {
        Rank::HighCard
    }
}
fn parse_line(input: &str) -> IResult<&str, CamelCard> {
    let (input, (hand, space, bid, _)) =
        tuple((alphanumeric1, space1, complete::u32, line_ending))(input)?;
    Ok((input, CamelCard::new(hand, bid)))
}
fn parse(input: &str) -> IResult<&str, Vec<CamelCard>> {
    let (input, cards) = many1(parse_line)(input)?;
    Ok((input, cards))
}
pub fn part_one(input: &str) -> Option<u32> {
    None
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
        println!("{:?}", cards);
        assert_eq!(cards.len(), 4);
    }

    #[test]
    fn test_sort() {
        let mut card = CamelCard::new("A1K2J",10);
        card.order_cards();
        assert_eq!(card.hand, "AKJ21");
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
