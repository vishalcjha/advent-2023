#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    FiveOfAKind = 60,
    FourOfAKind = 50,
    FullHouse = 40,
    ThreeOfAKind = 30,
    TwoPair = 20,
    OnePair = 10,
    HighCard = 0,
}

impl CardType {
    pub(self) fn new(card: &str) -> CardType {
        let card_count =
            card.chars()
                .fold(HashMap::<char, u8>::new(), |mut char_count, current| {
                    char_count
                        .entry(current)
                        .and_modify(|current| *current += 1)
                        .or_insert(1);
                    char_count
                });
        if card_count.len() == 1 {
            return CardType::FiveOfAKind;
        }

        if *card_count.values().max().unwrap() == 4 {
            return CardType::FourOfAKind;
        }

        if *card_count.values().max().unwrap() == 3 {
            if card_count.len() == 2 {
                return CardType::FullHouse;
            }

            return CardType::ThreeOfAKind;
        }

        if *card_count.values().max().unwrap() == 2 {
            if card_count.len() == 3 {
                return CardType::TwoPair;
            }

            return CardType::OnePair;
        }

        CardType::HighCard
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Card<const B: bool> {
    value: String,
    c_type: CardType,
}

impl<const B: bool> Card<B> {
    const USE_JOKER: bool = B;
}

impl Card<true> {
    fn new(value: impl Into<String>) -> Card<true> {
        let value: String = value.into();
        let mut c_type = CardType::new(&value);

        if value.contains("J") {
            let mut distinct_chars = value.chars().fold(HashSet::new(), |mut accum, current| {
                accum.insert(current);
                accum
            });

            distinct_chars.remove(&'J');

            for other in distinct_chars {
                let new_value = value.clone();
                let new_value = new_value.replace(&'J'.to_string(), &other.to_string());
                let new_card_type = CardType::new(&new_value);
                if new_card_type > c_type {
                    c_type = new_card_type;
                }
            }
        }

        Card { value, c_type }
    }
}

impl Card<false> {
    fn new(value: impl Into<String>) -> Card<false> {
        let value: String = value.into();
        let c_type = CardType::new(&value);

        Card { value, c_type }
    }
}

impl PartialOrd for Card<true> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Card<false> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card<true> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        lazy_static! {
            static ref CHAR_ORDER: HashMap<char, u32> = {
                let mut m = HashMap::new();
                m.insert('A', 0);
                m.insert('K', 1);
                m.insert('Q', 2);
                m.insert('T', 4);
                m.insert('9', 5);
                m.insert('8', 6);
                m.insert('7', 7);
                m.insert('6', 8);
                m.insert('5', 9);
                m.insert('4', 10);
                m.insert('3', 11);
                m.insert('2', 12);
                m.insert('J', 13);
                m
            };
        }
        match self.c_type.cmp(&other.c_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        for (first, second) in self.value.chars().zip(other.value.chars()) {
            let first = *CHAR_ORDER.get(&first).unwrap();
            let second = *CHAR_ORDER.get(&second).unwrap();

            if first != second {
                return Reverse(first).cmp(&Reverse(second));
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl Ord for Card<false> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        lazy_static! {
            static ref CHAR_ORDER: HashMap<char, u32> = {
                let mut m = HashMap::new();
                m.insert('A', 0);
                m.insert('K', 1);
                m.insert('Q', 2);
                m.insert('J', 3);
                m.insert('T', 4);
                m.insert('9', 5);
                m.insert('8', 6);
                m.insert('7', 7);
                m.insert('6', 8);
                m.insert('5', 9);
                m.insert('4', 10);
                m.insert('3', 11);
                m.insert('2', 12);
                m
            };
        }
        match self.c_type.cmp(&other.c_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        for (first, second) in self.value.chars().zip(other.value.chars()) {
            let first = *CHAR_ORDER.get(&first).unwrap();
            let second = *CHAR_ORDER.get(&second).unwrap();

            if first != second {
                return Reverse(first).cmp(&Reverse(second));
            }
        }
        std::cmp::Ordering::Equal
    }
}

#[derive(Debug)]
struct Game<const B: bool> {
    cards: Vec<(Card<B>, u32)>,
}

impl Game<true> {
    pub fn add_new_card(&mut self, line: &str) {
        let mut iter = line.split_ascii_whitespace().take(2);
        let card = iter.next().unwrap();
        let bet = iter.next().unwrap().parse::<u32>().unwrap();
        let card = Card::<true>::new(card);
        self.cards.push((card, bet));
    }
}

impl Game<false> {
    pub fn add_new_card(&mut self, line: &str, _use_joken: bool) {
        let mut iter = line.split_ascii_whitespace().take(2);
        let card = iter.next().unwrap();
        let bet = iter.next().unwrap().parse::<u32>().unwrap();
        let card = Card::<false>::new(card);
        self.cards.push((card, bet));
    }
}

impl Game<true> {
    pub fn new() -> Game<true> {
        Game {
            cards: Vec::<(Card<true>, u32)>::new(),
        }
    }
}

impl Game<false> {
    pub fn new() -> Game<false> {
        Game {
            cards: Vec::<(Card<false>, u32)>::new(),
        }
    }
}

impl Game<true> {
    pub fn find_win_point(&mut self) -> u32 {
        self.cards
            .sort_by(|(first, _), (second, _)| first.cmp(&second));

        let mut win_point = 0;
        for (index, card_bet) in self.cards.iter().enumerate() {
            win_point += card_bet.1 * (index as u32 + 1);
        }

        win_point
    }
}

impl Game<false> {
    pub fn find_win_point(&mut self) -> u32 {
        self.cards
            .sort_by(|(first, _), (second, _)| first.cmp(&second));

        let mut win_point = 0;
        for (index, card_bet) in self.cards.iter().enumerate() {
            win_point += card_bet.1 * (index as u32 + 1);
        }

        win_point
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"#;

        let mut game = Game::<false>::new();
        for each in input.split("\n") {
            game.add_new_card(each, false);
        }

        assert_eq!(6440, game.find_win_point());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day7.txt");

        let mut game = Game::<false>::new();
        for each in file_content.0.lines() {
            game.add_new_card(each, false);
        }

        println!("Answer1 for day7 is {}", game.find_win_point());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"#;

        let mut game = Game::<true>::new();
        for each in input.split("\n") {
            game.add_new_card(each);
        }

        assert_eq!(5905, game.find_win_point());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day7.txt");

        let mut game = Game::<true>::new();
        for each in file_content.0.lines() {
            game.add_new_card(each);
        }

        println!("Answer1 for day7 is {}", game.find_win_point());
    }
}
