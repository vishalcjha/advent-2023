#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    color_count: HashMap<Color, u32>,
}

struct GameLine {
    game_num: u32,
    shows: Vec<HashMap<Color, u32>>,
}

impl GameLine {
    fn new(line: &str) -> Option<GameLine> {
        let game_play: Vec<&str> = line.split(":").collect();
        if game_play.len() != 2 {
            return None;
        }

        let mut shows = Vec::new();
        for each_reveal in game_play[1].split(";") {
            let mut show = HashMap::new();
            for each_color in each_reveal.split(",") {
                let count_color: Vec<_> = each_color.split_ascii_whitespace().collect();
                if count_color.len() != 2 {
                    return None;
                }

                let count = count_color[0].parse::<u32>().unwrap();
                let color: Color = count_color[1].try_into().unwrap();
                show.insert(color, count);
            }
            shows.push(show);
        }

        let game_num: Vec<_> = game_play[0].split_ascii_whitespace().collect();
        if game_num.len() != 2 {
            return None;
        }
        let game_num = game_num[1].parse().unwrap();

        Some(GameLine { game_num, shows })
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new(12, 13, 14)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            err @ _ => Err(format!("Failed to convert value {err}")),
        }
    }
}

impl Game {
    fn new(red_count: u32, green_count: u32, blue_count: u32) -> Self {
        Game {
            color_count: HashMap::from([
                (Color::Red, red_count),
                (Color::Blue, blue_count),
                (Color::Green, green_count),
            ]),
        }
    }

    fn is_game_possible(&self, line: &str) -> Option<u32> {
        let Some(game_line) = GameLine::new(line) else {
            return None;
        };

        for each_show in game_line.shows {
            for each_col in each_show {
                let Some(available) = self.color_count.get(&each_col.0) else {
                    return None;
                };
                if *available < each_col.1 {
                    return None;
                }
            }
        }

        Some(game_line.game_num)
    }
}

fn power_provider(line: &str) -> u32 {
    let Some(game_line) = GameLine::new(line) else {
        return 0;
    };

    let mut max_color_count = HashMap::<Color, u32>::new();
    for each_show in game_line.shows {
        for each_col in each_show {
            max_color_count
                .entry(each_col.0)
                .and_modify(|current_max| {
                    *current_max = *current_max.max(&mut each_col.1.clone());
                })
                .or_insert(each_col.1);
        }
    }

    if max_color_count.is_empty() {
        return 0;
    }
    max_color_count
        .values()
        .fold(1, |accum, current| accum * current)
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;

    #[test]
    fn test_first_with_local_data() {
        let game = Game::default();
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let mut ans = 0;
        for each in input.split("\n") {
            if let Some(game_num) = game.is_game_possible(&each) {
                ans += game_num;
            }
        }

        assert_eq!(ans, 8);
    }

    #[test]
    fn test_first_with_file() {
        let game = Game::default();
        let file_content = FileContent::new("day2.txt");

        let mut ans = 0;
        for each in file_content.0.lines() {
            if let Some(game_num) = game.is_game_possible(&each) {
                ans += game_num;
            }
        }

        println!("Answer1 for day2 is {}", ans);
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let mut ans = 0;
        for each in input.split("\n") {
            ans += power_provider(&each);
        }

        assert_eq!(ans, 2286);
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day2.txt");

        let mut ans = 0;
        for each in file_content.0.lines() {
            ans += power_provider(&each);
        }

        println!("Answer1 for day2 is {}", ans);
    }
}
