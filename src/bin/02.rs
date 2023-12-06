

advent_of_code::solution!(2);

#[derive(Debug, Default)]
struct Game {
    id: u32,
    round: Vec<Round>,
}

#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn default() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let beg = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let output: u32 = input
        .lines()
        .map(|line| {
            let game = process_line(line);
            if game.round.iter().any(|game_try| {
                game_try.red > beg.red || game_try.green > beg.green || game_try.blue > beg.blue
            }) {
                0
            } else {
                game.id
            }
        })
        .sum();
    Some(output)
}

fn process_line(line: &str) -> Game {
    let mut token: Vec<&str> = line.split(&[':', ';']).collect();
    let mut it = token.iter_mut();
    let mut game = Game::default();
    game.id = it
        .next()
        .expect("There must game segment")
        .replace("Game", "")
        .trim()
        .parse()
        .expect("It must be game id.");

    game.round = it
        .map(|strgame| {
            let bolls: Vec<&str> = strgame.split(',').collect();
            let mut game_try: Round = Round::default();
            let _: Vec<_> = bolls
                .iter()
                .map(|boll| {
                    if boll.contains("red") {
                        game_try.red += boll
                            .replace("red", "")
                            .trim()
                            .parse::<u32>()
                            .expect("red boll number.");
                    } else if boll.contains("blue") {
                        game_try.blue += boll
                            .replace("blue", "")
                            .trim()
                            .parse::<u32>()
                            .expect("blue boll number.");
                    } else if boll.contains("green") {
                        game_try.green += boll
                            .replace("green", "")
                            .trim()
                            .parse::<u32>()
                            .expect("green boll number.");
                    }
                })
                .collect();
            game_try
        })
        .collect();
    game
}
pub fn part_two(input: &str) -> Option<u32> {
    let _beg = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let output: u32 = input
        .lines()
        .map(|line| {
            let game = process_line(line);
            let red = game.round.iter().max_by_key(|x| x.red).unwrap().red;
            let blue = game.round.iter().max_by_key(|x| x.blue).unwrap().blue;
            let green = game.round.iter().max_by_key(|x| x.green).unwrap().green;
            red * blue * green
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
