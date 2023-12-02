#![allow(dead_code)]

use std::collections::HashMap;

trait DigitFinder {
    fn get_value(&self, input: &str) -> Vec<u32>;
}

struct SimpleDigitFinder {}
impl DigitFinder for SimpleDigitFinder {
    fn get_value(&self, input: &str) -> Vec<u32> {
        input
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .fold(Vec::new(), |mut accum, current| {
                if accum.len() >= 2 {
                    accum.pop();
                }
                accum.push(current.to_digit(10).unwrap());
                accum
            })
    }
}

struct WordDigitDigitFinder<'a> {
    pub dictonary: HashMap<&'a str, i32>,
}

impl<'a> WordDigitDigitFinder<'a> {
    fn new() -> Self {
        let dictonary = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);
        WordDigitDigitFinder { dictonary }
    }
}

impl<'a> DigitFinder for WordDigitDigitFinder<'a> {
    fn get_value(&self, input: &str) -> Vec<u32> {
        let mut pos_value = Vec::new();
        for key in self.dictonary.keys() {
            let mut found: Vec<_> = input.match_indices(key).collect();
            pos_value.append(&mut found);
        }
        pos_value.sort_by_key(|k| k.0);
        match pos_value.len() {
            0 => vec![],
            1 => pos_value,
            2 => pos_value,
            len @ _ => vec![pos_value[0], pos_value[len - 1]],
        }
        .iter()
        .map(|value| *self.dictonary.get(value.1).unwrap() as u32)
        .collect()
    }
}

fn get_calibration_value(input: &str, digit_finder: &impl DigitFinder) -> i32 {
    let first_and_last = digit_finder.get_value(input);

    match first_and_last.len() {
        0 => 0,
        1 => format!("{:?}{:?}", first_and_last[0], first_and_last[0])
            .parse()
            .unwrap(),
        2 => format!("{:?}{:?}", first_and_last[0], first_and_last[1])
            .parse()
            .unwrap(),
        unexpected @ _ => panic!("found a bug to fix with {unexpected}"),
    }
}

pub fn get_simple_collaboration<'a>(input: impl IntoIterator<Item = &'a str>) -> i32 {
    input
        .into_iter()
        .map(|line| get_calibration_value(line, &SimpleDigitFinder {}))
        .sum::<i32>()
}

pub fn get_word_digit_collaboration<'a>(input: impl IntoIterator<Item = &'a str>) -> i32 {
    input
        .into_iter()
        .map(|line| get_calibration_value(line, &WordDigitDigitFinder::new()))
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn get_result_from_sample_one() {
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#;
        assert_eq!(142, get_simple_collaboration(input.split("\n")));
    }

    #[test]
    fn run_for_file() {
        use std::fs::read_to_string;
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/input/day1.txt");

        println!(
            "And the answer to first day is {:?}",
            get_simple_collaboration(read_to_string(format!("{}", d.display())).unwrap().lines())
        );
    }

    #[test]
    fn get_result_from_sample_two() {
        let input = r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(281, get_word_digit_collaboration(input.split("\n")));
    }

    #[test]
    fn run_for_file_day1_2() {
        use std::fs::read_to_string;
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/input/day1.txt");

        println!(
            "And the answer to first day is {:?}",
            get_word_digit_collaboration(
                read_to_string(format!("{}", d.display())).unwrap().lines()
            )
        );
    }
}
