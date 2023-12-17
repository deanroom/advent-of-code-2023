use std::vec;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|line: &str| -> u32 {
            let data = line.split([':', '|']).collect::<Vec<&str>>();

            let winning_numbers = data[1]
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<Vec<u32>>();

            let game_numbers = data[2]
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<Vec<u32>>();

            process(&game_numbers, &winning_numbers)
        })
        .sum();
    Some(output)
}

fn process(game_numbers: &[u32], winning_numbers: &[u32]) -> u32 {
    let win: u32 = game_numbers
        .iter()
        .filter_map(|num| {
            if winning_numbers.iter().any(|x| *x == *num) {
                Some(1)
            } else {
                None
            }
        })
        .sum();
    if win == 0 {
        0
    } else {
        1 << (win - 1)
    }
}

fn process_win(game_numbers: &[u32], winning_numbers: &[u32]) -> u32 {
    let win: u32 = game_numbers
        .iter()
        .filter_map(|num| {
            if winning_numbers.iter().any(|x| *x == *num) {
                Some(1)
            } else {
                None
            }
        })
        .sum();
    win
}
#[derive(Debug, Default)]
struct Card {
    id: u32,
    win: u32,
}
#[derive(Debug, Default)]
struct CardNew {
    id: u32,
    win: u32,
    children: Vec<CardNew>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut original_cards: Vec<Card> = vec![];
    for (index, line) in input.lines().enumerate() {
        let data: Vec<&str> = line.split([':', '|']).collect::<Vec<&str>>();
        let winning_numbers = data[1]
            .split(' ')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let game_numbers = data[2]
            .split(' ')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let card = Card {
            id: (index as u32) + 1,
            win: process_win(&game_numbers, &winning_numbers),
        };
        original_cards.push(card);
    }
    // println!("{:?}", original_cards);
    let mut it = original_cards.iter();
    let mut number = 0;
    for node in it {
        let mut root = CardNew::default();
        root.id = node.id;
        root.win = node.win;
        scratch(&mut root, &original_cards);
        number = number + count_number(&root);
    }
    // println!("{:?}", root);
    Some(number)
}

fn scratch<'a>(root: &mut CardNew, origin_cards: &Vec<Card>) {
    let mut copied_index = root.id + root.win;
    if copied_index >= origin_cards.len() as u32 {
        copied_index = (origin_cards.len() - 1) as u32;
    }

    if copied_index <= root.id {
        return;
    }

    for index in root.id..copied_index {
        let copied_card = origin_cards.get(index as usize).unwrap();
        let mut child = CardNew::default();
        child.id = copied_card.id;
        child.win = copied_card.win;
        if child.win > 0 {
            scratch(&mut child, &origin_cards)
        }
        root.children.push(child);
    }
}
fn count_number(root: &CardNew) -> u32 {
    let mut count = 1;
    for node in root.children.iter() {
        count = count + count_number(node);
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two_count_number() {
        let result = count_number(&CardNew {
            id: 1,
            win: 8,
            children: vec![CardNew {
                id: 2,
                win: 2,
                children: vec![CardNew {
                    id: 3,
                    win: 2,
                    children: vec![
                        CardNew {
                            id: 4,
                            win: 1,
                            children: vec![CardNew {
                                id: 5,
                                win: 0,
                                children: vec![],
                            }],
                        },
                        CardNew {
                            id: 5,
                            win: 0,
                            children: vec![],
                        },
                    ],
                }],
            }],
        });
        assert_eq!(result, 6);
    }
}
