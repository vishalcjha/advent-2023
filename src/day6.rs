#![allow(dead_code)]

#[derive(Debug)]
struct RaceCalculator {
    time: u128,
    distance: u128,
}

impl RaceCalculator {
    pub fn new(time: u128, distance: u128) -> Self {
        RaceCalculator { time, distance }
    }

    pub fn find_winning_ways(&self) -> u128 {
        let Some(right_end) = self.find_right_end(1, self.time) else {
            return 0;
        };
        let Some(left_end) = self.find_left_end(1, self.time) else {
            panic!("left most at worst has to be same as right most i.e. single win");
        };

        1 + right_end - left_end
    }

    pub(self) fn find_left_end(&self, left: u128, right: u128) -> Option<u128> {
        let mut smallest_winning_left = None;
        if left <= right {
            let mid = left + (right - left) / 2;

            if self.will_win_in_time(mid) {
                smallest_winning_left = smallest_winning_left.or(Some(mid));
                if let Some(other_possible) = self.find_left_end(left, mid - 1) {
                    smallest_winning_left =
                        smallest_winning_left.and_then(|current| Some(current.min(other_possible)));
                }
            } else {
                if let Some(other_possible) = self.find_left_end(left, mid - 1) {
                    smallest_winning_left =
                        smallest_winning_left.and_then(|current| Some(current.min(other_possible)));
                } else {
                    smallest_winning_left = self.find_left_end(mid + 1, right)
                }
            }
        }
        smallest_winning_left
    }

    pub(self) fn find_right_end(&self, left: u128, right: u128) -> Option<u128> {
        let mut largest_winning_right: Option<u128> = None;
        if left <= right {
            let mid = left + (right - left) / 2;

            if self.will_win_in_time(mid) {
                largest_winning_right = Some(mid);
                if let Some(other_possible) = self.find_right_end(mid + 1, right) {
                    largest_winning_right =
                        largest_winning_right.and_then(|current| Some(current.max(other_possible)));
                }
            } else {
                if let Some(other_possible) = self.find_right_end(mid + 1, right) {
                    largest_winning_right = Some(other_possible);
                } else {
                    largest_winning_right = self.find_right_end(left, mid - 1);
                }
            }
        }
        largest_winning_right
    }

    pub(self) fn will_win_in_time(&self, charge_time: u128) -> bool {
        let speed = charge_time;
        let remaining = self.time - charge_time;
        self.distance < speed * remaining
    }
}

struct RaceCalculatorBuilder {
    lines: Vec<String>,
    pos: u8,
}

impl RaceCalculatorBuilder {
    fn new() -> Self {
        RaceCalculatorBuilder {
            lines: Vec::new(),
            pos: 0,
        }
    }

    fn add_next_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
        self.pos += 1;
    }

    fn calculate_result(&self, append_numbers: bool) -> u128 {
        let get_numbers = |line: &str| {
            let iter = line
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split_ascii_whitespace();
            if append_numbers {
                let combined_num = iter.collect::<String>();
                println!("Combined num is {:?}", combined_num);
                let combined_num = combined_num.parse::<u128>().unwrap();
                vec![combined_num]
            } else {
                iter.filter_map(|num| num.parse::<u128>().ok())
                    .collect::<Vec<_>>()
            }
        };

        let times = get_numbers(&self.lines[0]);
        let distance = get_numbers(&self.lines[1]);

        let mut total_ways: Option<u128> = None;
        for (time, distance) in times.iter().zip(distance.iter()) {
            let race_calculator = RaceCalculator::new(*time, *distance);
            let winning_ways = race_calculator.find_winning_ways();
            // println!("Winning ways {:?} for {:?}", winning_ways, race_calculator);
            if winning_ways <= 0 {
                continue;
            }

            total_ways = match total_ways.take() {
                Some(previous) => Some(previous * winning_ways),
                None => Some(winning_ways),
            };
        }

        total_ways.unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"Time:      7  15   30
        Distance:  9  40  200"#;

        let mut race_calculator_builder = RaceCalculatorBuilder::new();
        for each in input.split("\n") {
            let _ = race_calculator_builder.add_next_line(each);
        }

        assert_eq!(288, race_calculator_builder.calculate_result(false));
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day6.txt");

        let mut race_calculator_builder = RaceCalculatorBuilder::new();
        for each in file_content.0.lines() {
            let _ = race_calculator_builder.add_next_line(each);
        }

        println!(
            "Answer1 for day6 is {}",
            race_calculator_builder.calculate_result(false)
        );
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"Time:      7  15   30
        Distance:  9  40  200"#;

        let mut race_calculator_builder = RaceCalculatorBuilder::new();
        for each in input.split("\n") {
            let _ = race_calculator_builder.add_next_line(each);
        }

        assert_eq!(71503, race_calculator_builder.calculate_result(true));
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day6.txt");

        let mut race_calculator_builder = RaceCalculatorBuilder::new();
        for each in file_content.0.lines() {
            let _ = race_calculator_builder.add_next_line(each);
        }

        println!(
            "Answer1 for day6 is {}",
            race_calculator_builder.calculate_result(true)
        );
    }
}
