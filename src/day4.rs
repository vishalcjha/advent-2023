#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
struct WinEvaluator {
    winning_total: u32,
    card_win_map: HashMap<u32, u32>,
    max_valid_card_num: u32,
}

#[derive(Debug)]
struct CardWin(u32, u32);

impl WinEvaluator {
    pub fn add_to_total(&mut self, line: &str) -> Result<CardWin, &'static str> {
        let card_win_hand = line.split(":").collect::<Vec<_>>();

        if card_win_hand.len() != 2 {
            return Err("Wrong Format");
        }

        let Some(card) =  card_win_hand[0]
            .split_ascii_whitespace().last()
            .and_then(|card| card.parse::<u32>().ok()) else {
            return Err("could not find card");
        };

        let Some(win_hand) = card_win_hand[1].split(":").last() else {
            return Err("No game value");
        };
        let win_hand = win_hand.split("|").collect::<Vec<_>>();
        if win_hand.len() != 2 {
            return Err("No Hand and wins");
        }
        let winning_hand = win_hand[0]
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let total_winning_hand = win_hand[1]
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .filter_map(|hand| winning_hand.get(&hand))
            .count();
        if total_winning_hand <= 0 {
            return Ok(CardWin(card, 0));
        }
        let new_card_win = 2_u32.pow(total_winning_hand as u32 - 1);
        self.winning_total += new_card_win;

        Ok(CardWin(card, total_winning_hand as u32))
    }

    pub fn add_new_winning_cards(&mut self, line: &str) {
        let Some(card_win) = self.add_to_total(line).ok() else {
            return;
        };

        let current_card = card_win.0;
        let total_current_card = *self.card_win_map.entry(current_card).or_insert(1);
        // this logic only works if added in correct card number. Lower card first and then higher card.
        for i in 1..card_win.1 + 1 {
            self.card_win_map
                .entry(current_card + i)
                .and_modify(|current_count| *current_count += total_current_card)
                // +1 because there is one card by default. Otherwise for winning card, we will be undercounting by 1.
                .or_insert(total_current_card + 1);
        }
        self.max_valid_card_num = current_card;
    }

    pub fn get_total(&self) -> u32 {
        self.winning_total
    }

    pub fn get_total_scratchcard_count(&self) -> u32 {
        let valid_card_win = self
            .card_win_map
            .iter()
            .filter(|kv| kv.0 <= &self.max_valid_card_num)
            .collect::<HashMap<_, _>>();
        valid_card_win.values().map(|v| *v).sum()
    }
}

impl Default for WinEvaluator {
    fn default() -> Self {
        Self {
            winning_total: Default::default(),
            card_win_map: HashMap::new(),
            max_valid_card_num: 0,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let mut win_calculator = WinEvaluator::default();
        for each in input.split("\n") {
            let _ = win_calculator.add_to_total(each);
        }

        assert_eq!(13, win_calculator.get_total());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day4.txt");

        let mut win_calculator = WinEvaluator::default();
        for each in file_content.0.lines() {
            let _ = win_calculator.add_to_total(each);
        }

        println!("Answer1 for day4 is {}", win_calculator.get_total());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let mut win_calculator = WinEvaluator::default();
        for each in input.split("\n") {
            let _ = win_calculator.add_new_winning_cards(each);
        }

        assert_eq!(30, win_calculator.get_total_scratchcard_count());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day4.txt");

        let mut win_calculator = WinEvaluator::default();
        for each in file_content.0.lines() {
            let _ = win_calculator.add_new_winning_cards(each);
        }

        println!(
            "Answer1 for day4 is {}",
            win_calculator.get_total_scratchcard_count()
        );
    }
}
