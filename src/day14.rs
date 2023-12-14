#![allow(dead_code)]

use std::{collections::HashMap, fmt::Debug};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Object {
    Rock,
    Cube,
    Space,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Object {
    fn from_char(ch: char) -> Object {
        use Object::*;
        match ch {
            'O' => Rock,
            '#' => Cube,
            '.' => Space,
            unknown @ _ => panic!("Received {unknown}"),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Rock => "O",
                Object::Cube => "#",
                Object::Space => ".",
            },
        )
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Reflector {
    space: Vec<Vec<Object>>,
}

impl Reflector {
    fn accept_line(&mut self, line: &str) {
        let line = line.trim();
        let new_row = line.chars().map(Object::from_char).collect();
        self.space.push(new_row);
    }

    fn tilt_east(space: &mut Vec<Vec<Object>>) {
        let col_count = space[0].len();
        let row_count = space.len();
        for row in 0..row_count {
            for col in (0..col_count - 1).rev() {
                if space[row][col] == Object::Rock {
                    let mut col = col;
                    while col < col_count - 1 {
                        col += 1;
                        if space[row][col] != Object::Space {
                            break;
                        }

                        space[row][col - 1] = Object::Space;
                        space[row][col] = Object::Rock;
                    }
                }
            }
        }
    }

    fn tilt_west(space: &mut Vec<Vec<Object>>) {
        let col_count = space[0].len();
        let row_count = space.len();
        for row in 0..row_count {
            for col in 1..col_count {
                if space[row][col] == Object::Rock {
                    let mut col = col;
                    while col >= 1 {
                        col -= 1;
                        if space[row][col] != Object::Space {
                            break;
                        }

                        space[row][col + 1] = Object::Space;
                        space[row][col] = Object::Rock;
                    }
                }
            }
        }
    }

    fn tilt_north(space: &mut Vec<Vec<Object>>) {
        let col_count = space[0].len();
        let row_count = space.len();
        for col in 0..col_count {
            for row in 1..row_count {
                if space[row][col] == Object::Rock {
                    let mut row = row;
                    while row >= 1 {
                        row -= 1;
                        if space[row][col] != Object::Space {
                            break;
                        }

                        space[row + 1][col] = Object::Space;
                        space[row][col] = Object::Rock;
                    }
                }
            }
        }
    }

    fn tilt_south(space: &mut Vec<Vec<Object>>) {
        let row_count = space.len();
        let col_count = space[0].len();
        for col in 0..col_count {
            for row in (0..row_count - 1).rev() {
                if space[row][col] == Object::Rock {
                    let mut row = row;
                    while row < row_count - 1 {
                        row += 1;
                        if space[row][col] != Object::Space {
                            break;
                        }

                        space[row - 1][col] = Object::Space;
                        space[row][col] = Object::Rock;
                    }
                }
            }
        }
    }

    fn tilt(&mut self, (directions, count): (Vec<Direction>, usize)) {
        let mut visited = HashMap::<Reflector, usize>::new();
        let mut space = self.clone();
        let mut i = 0;
        let mut do_match = true;
        while i < count {
            if do_match {
                if let Some(x) = visited.get(&space) {
                    do_match = false;
                    let repeat_gap = i - x;
                    let remaining_steps = count - i;
                    let multiplier = remaining_steps / repeat_gap;
                    i += multiplier * repeat_gap;
                    continue;
                }

                visited.insert(space.clone(), i);
            }

            for direction in directions.iter() {
                use Direction::*;
                match direction {
                    North => Reflector::tilt_north(&mut space.space),
                    South => Reflector::tilt_south(&mut space.space),
                    East => Reflector::tilt_east(&mut space.space),
                    West => Reflector::tilt_west(&mut space.space),
                }
            }

            i += 1;
        }

        self.space = space.space;
    }

    fn find_load(&mut self) -> usize {
        let row_count = self.space.len();
        let mut load = 0;
        for (index, row) in self.space.iter().enumerate() {
            let row_rock_count = row.iter().filter(|it| **it == Object::Rock).count();
            load += row_rock_count * (row_count - index);
        }

        load
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    use Direction::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."#;

        let mut reflector = Reflector::default();
        for each in input.split("\n") {
            reflector.accept_line(each);
        }

        reflector.tilt((vec![North], 1));

        assert_eq!(136, reflector.find_load());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day14.txt");

        let mut reflector = Reflector::default();
        for each in file_content.0.lines() {
            reflector.accept_line(each);
        }

        reflector.tilt((vec![North], 1));

        println!("Answer1 for day14 is {}", reflector.find_load());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."#;

        let mut reflector = Reflector::default();
        for each in input.split("\n") {
            reflector.accept_line(each);
        }

        reflector.tilt((vec![North, West, South, East], 1_000_000_000));

        assert_eq!(64, reflector.find_load());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day14.txt");

        let mut reflector = Reflector::default();
        for each in file_content.0.lines() {
            reflector.accept_line(each);
        }

        reflector.tilt((vec![North, West, South, East], 1_000_000_000));

        println!("Answer1 for day14 is {}", reflector.find_load());
    }
}
