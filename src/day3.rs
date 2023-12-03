#![allow(dead_code)]

struct Engine {
    input: Vec<Vec<char>>,
}

impl Engine {
    fn new() -> Self {
        Engine { input: Vec::new() }
    }

    fn add_new_line(&mut self, line: &str) {
        let mut new_row = Vec::new();
        for ch in line.trim().chars() {
            new_row.push(ch);
        }

        self.input.push(new_row);
    }

    fn is_part_num(&self, row: usize, col: usize) -> bool {
        for rx in [0, 1, -1] {
            for cx in [0, 1, -1] {
                if rx == 0 && cx == 0 {
                    continue;
                }
                let next_row = row as i32 + rx;
                let next_col = col as i32 + cx;
                let Some((next_row, next_col)) = self.is_valid_row(next_row, next_col) else {
                    continue;
                };

                let checked_char = self.input[next_row][next_col];
                if !checked_char.is_ascii_digit() && checked_char != '.' {
                    return true;
                }
            }
        }

        false
    }

    fn is_valid_row(&self, next_row: i32, next_col: i32) -> Option<(usize, usize)> {
        if next_row < 0
            || next_col < 0
            || next_row >= self.input.len() as i32
            || next_col >= self.input[0].len() as i32
        {
            None
        } else {
            Some((next_row as usize, next_col as usize))
        }
    }

    fn get_gear_number(&self, row: usize, col: usize) -> Option<(u32, u32)> {
        let mut near_numbers = Vec::new();
        for rx in [0, 1, -1] {
            for cx in [0, 1, -1] {
                if rx == 0 && cx == 0 {
                    continue;
                }
                let next_row = row as i32 + rx;
                let next_col = col as i32 + cx;
                let Some((next_row, next_col)) = self.is_valid_row(next_row, next_col) else {
                    continue;
                };

                let checked_char = self.input[next_row][next_col];
                if checked_char.is_ascii_digit() {
                    let (next_num, _) = self.get_num_and_index(next_row, next_col);
                    near_numbers.push(next_num);
                    if cx == 0 {
                        break;
                    }
                }
            }
        }

        if near_numbers.len() == 2 {
            Some((near_numbers[0], near_numbers[1]))
        } else {
            None
        }
    }

    fn get_num_and_index(&self, row: usize, col: usize) -> (u32, usize) {
        let mut begin = col;
        let mut end = col;

        let current_row = &self.input[row];
        while begin > 0 {
            if !current_row[begin - 1].is_ascii_digit() {
                break;
            }
            begin -= 1;
        }

        while end < current_row.len() - 1 {
            if !current_row[end + 1].is_ascii_digit() {
                break;
            }

            end += 1;
        }

        let num: u32 = current_row[begin..=end]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        (num, end)
    }

    pub(crate) fn sum_of_all_parts(&self) -> u32 {
        let mut sum = 0;
        for (line_num, line) in self.input.iter().enumerate() {
            let mut i = 0;
            while i < line.len() {
                if line[i].is_ascii_digit() && self.is_part_num(line_num, i) {
                    let (num, next_i) = self.get_num_and_index(line_num, i);
                    sum += num;
                    i = next_i;
                }
                i += 1;
            }
        }
        sum
    }

    pub(crate) fn sum_all_gears(&self) -> u32 {
        let mut sum = 0;
        for (line_num, line) in self.input.iter().enumerate() {
            let mut i = 0;
            while i < line.len() {
                if line[i] == '*' {
                    if let Some((first, second)) = self.get_gear_number(line_num, i) {
                        sum += first * second;
                    }
                }
                i += 1;
            }
        }
        sum
    }
}
mod test {

    use crate::{day3::Engine, file_input_iterator::FileContent};

    #[test]
    fn test_first_with_local_data() {
        let input = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#;

        let mut engine = Engine::new();
        for each in input.split("\n") {
            engine.add_new_line(each);
        }

        assert_eq!(4361, engine.sum_of_all_parts());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day3.txt");

        let mut engine = Engine::new();
        for each in file_content.0.lines() {
            engine.add_new_line(each);
        }

        println!("Answer1 for day3 is {}", engine.sum_of_all_parts());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#;

        let mut engine = Engine::new();
        for each in input.split("\n") {
            engine.add_new_line(each);
        }

        assert_eq!(467835, engine.sum_all_gears());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day3.txt");

        let mut engine = Engine::new();
        for each in file_content.0.lines() {
            engine.add_new_line(each);
        }

        println!("Answer2 for day3 is {}", engine.sum_all_gears());
    }
}
