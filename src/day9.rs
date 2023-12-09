#![allow(dead_code)]

#[derive(Debug)]
struct ValueHistory {
    values: Vec<Vec<i64>>,
}

impl ValueHistory {
    fn new(history: &str) -> ValueHistory {
        let mut single_history = history
            .split_ascii_whitespace()
            .filter_map(|v| v.parse::<i64>().ok())
            .collect::<Vec<_>>();
        let mut values = Vec::new();
        while !single_history.iter().all(|v| *v == 0) {
            let next_entry = single_history
                .iter()
                .skip(1)
                .enumerate()
                // previos value is at pos index as we are skipping by 1.
                .map(|(index, value)| value - single_history[index])
                .collect();
            values.push(single_history);
            single_history = next_entry;
        }
        values.push(single_history);
        ValueHistory { values }
    }

    pub fn find_next_history(&self) -> i64 {
        let mut last = 0_i64;

        for entry in self.values.iter().rev() {
            last += *entry.last().unwrap();
        }

        last
    }

    pub fn find_reverse_history(&self) -> i64 {
        let mut last = 0_i64;

        for entry in self.values.iter().rev() {
            last = *entry.first().unwrap() - last;
        }
        last
    }
}

#[derive(Debug, Default)]
struct HistoryFinder {
    histories: Vec<ValueHistory>,
}

impl HistoryFinder {
    fn add_history(&mut self, history: &str) {
        let value_history = ValueHistory::new(history);
        self.histories.push(value_history);
    }

    fn sum_next_history(&self) -> i64 {
        self.histories
            .iter()
            .map(|history| history.find_next_history())
            .sum()
    }

    fn sum_reverse_hisotory(&self) -> i64 {
        self.histories
            .iter()
            .map(|history| history.find_reverse_history())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#;

        let mut history_finder = HistoryFinder::default();
        for each in input.split("\n") {
            history_finder.add_history(each);
        }

        assert_eq!(114, history_finder.sum_next_history());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day9.txt");

        let mut history_finder = HistoryFinder::default();
        for each in file_content.0.lines() {
            history_finder.add_history(each);
        }

        println!("Answer1 for day9 is {}", history_finder.sum_next_history());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#;

        let mut history_finder = HistoryFinder::default();
        for each in input.split("\n") {
            history_finder.add_history(each);
        }

        assert_eq!(2, history_finder.sum_reverse_hisotory());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day9.txt");

        let mut history_finder = HistoryFinder::default();
        for each in file_content.0.lines() {
            history_finder.add_history(each);
        }

        println!(
            "Answer1 for day9 is {}",
            history_finder.sum_reverse_hisotory()
        );
    }
}
