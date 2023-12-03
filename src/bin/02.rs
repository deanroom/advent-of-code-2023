use std::fmt::Display;

advent_of_code::solution!(2);

#[derive(Debug)]
struct GameTry {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameTry {
    fn default() -> GameTry {
        GameTry {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    tries: Vec<GameTry>,
}
impl Game {
    fn default() -> Game {
        Game {
            id: 0,
            tries: Vec::<GameTry>::new(),
        }
    }
}

impl Display for GameTry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " Red: {},Green: {},Blue: {}",
            self.red, self.green, self.blue
        )
    }
}

// impl Display for Game {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "game id:{}, tries:{}", self.id, self.tries)
//     }
// }

pub fn part_one(input: &str) -> Option<u32> {
    let beg = GameTry {
        red: 12,
        green: 13,
        blue: 14,
    };

    let output: u32 = input
        .lines()
        .map(|line| {
            let mut token: Vec<&str> = line.split(&[':', ';']).collect();
            let mut it = token.iter_mut();
            let game_id: u32 = it
                .next()
                .expect("There must game segment")
                .replace("Game", "")
                .trim()
                .parse()
                .expect("It must be game id.");
            let mut game = Game::default();
            game.id = game_id;

            let _: Vec<_> = it
                .map(|strgame| {
                    let bolls: Vec<&str> = strgame.split(',').collect();
                    let mut game_try: GameTry = GameTry::default();
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
                    game.tries.push(game_try);
                })
                .collect();

            if game.tries.iter().any(|game_try| {
                game_try.red > beg.red || game_try.green > beg.green || game_try.blue > beg.blue
            }) {
                0
            } else {
                game_id
            }
        })
        .sum();
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
