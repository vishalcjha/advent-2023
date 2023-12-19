#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{anychar, digit1, multispace1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(ch: char) -> Direction {
        use Direction::*;
        match ch {
            'L' => Left,
            'R' => Right,
            'U' => Up,
            'D' => Down,
            _ => panic!("Unknown direction {}", ch),
        }
    }

    fn pos_change(&self, step: u32) -> (i32, i32) {
        match self {
            Direction::Left => (0, -(step as i32)),
            Direction::Right => (0, step as i32),
            Direction::Up => (-(step as i32), 0),
            Direction::Down => (step as i32, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate(usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Patch {
    Vertical((Coordinate, Coordinate, String)),
    Horizontal((Coordinate, Coordinate, String)),
}

impl Patch {
    fn from_direction(
        direction: Direction,
        from: Coordinate,
        to: Coordinate,
        color: String,
    ) -> Patch {
        use Patch::*;
        match direction {
            Direction::Left | Direction::Right => Horizontal((from, to, color)),
            Direction::Up | Direction::Down => Vertical((from, to, color)),
        }
    }
}

#[derive(Debug, Default)]
struct LagoonMakerBuilder {
    patches: Vec<Patch>,
    max_row: usize,
    max_col: usize,

    current_pos: (usize, usize),
    // a hack to know what is offset
    signed_pos: (isize, isize),
    min_row: isize,
    min_col: isize,
}

impl LagoonMakerBuilder {
    fn nomify_line(line: &str) -> IResult<&str, (Direction, u32, &str)> {
        let (left, direction) = anychar(line)?;
        let (left, (_, steps)) = tuple((multispace1, digit1))(left)?;
        let (_, (_, color)) =
            tuple((multispace1, delimited(tag("("), is_not(")"), tag(")"))))(left)?;
        let resp = (
            Direction::from_char(direction),
            steps.parse::<u32>().unwrap(),
            color,
        );
        Ok(("", resp))
    }

    fn decode_color_to_step_and_direction(color: &str) -> (Direction, u32) {
        use Direction::*;
        let step = u32::from_str_radix(&color[1..6], 16).unwrap();
        let direction = u8::from_str_radix(&color[6..], 16).unwrap();
        let direction = match direction {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => panic!("Got somthing wrong {direction}"),
        };
        (direction, step)
    }

    fn accept_line(&mut self, line: &str, use_color: bool) {
        let (mut direction, mut steps, color) =
            LagoonMakerBuilder::nomify_line(line.trim()).unwrap().1;

        if use_color {
            (direction, steps) = LagoonMakerBuilder::decode_color_to_step_and_direction(color);
        }

        let (dx, dy) = direction.pos_change(steps);
        let (x, y) = self.current_pos;
        self.current_pos = (
            (x as isize + dx as isize) as usize,
            (y as isize + dy as isize) as usize,
        );
        let (x1, y1) = self.current_pos;
        let patch = Patch::from_direction(
            direction,
            Coordinate(x, y),
            Coordinate(x1, y1),
            color.to_string(),
        );

        self.max_row = self.max_row.max(x1);
        self.max_col = self.max_col.max(y1);

        self.patches.push(patch);
    }

    fn set_min(&mut self, line: &str, use_color: bool) {
        let (mut direction, mut steps, color) =
            LagoonMakerBuilder::nomify_line(line.trim()).unwrap().1;
        if use_color {
            (direction, steps) = LagoonMakerBuilder::decode_color_to_step_and_direction(color);
        }
        let (dx, dy) = direction.pos_change(steps);
        let (x, y) = self.signed_pos;
        self.signed_pos = (x + dx as isize, y + dy as isize);

        self.min_row = self.min_row.min(self.signed_pos.0);
        self.min_col = self.min_col.min(self.signed_pos.1);
    }
}

struct Lagoon {
    grid: Vec<Vec<char>>,
}

impl Debug for Lagoon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            writeln!(f, "{:?}", row)?;
        }

        Ok(())
    }
}

impl Lagoon {
    fn from_patches(patches: Vec<Patch>, max_row: usize, max_col: usize) -> Lagoon {
        let mut grid = (0..=max_row)
            .map(|_| vec!['.'; max_col + 1])
            .collect::<Vec<_>>();
        for patch in patches {
            match patch {
                Patch::Vertical((from, to, _)) => {
                    let (from, to) = (from.clone().min(to.clone()), from.max(to));
                    for i in from.0..=to.0 {
                        grid[i][from.1] = '#';
                    }
                }
                Patch::Horizontal((from, to, _)) => {
                    let (from, to) = (from.clone().min(to.clone()), from.max(to));
                    for i in from.1..=to.1 {
                        grid[from.0][i] = '#';
                    }
                }
            }
        }
        Lagoon { grid }
    }

    fn is_edge(&self, coordinate: &Coordinate) -> bool {
        coordinate.0 == 0
            || coordinate.0 == self.grid.len() - 1
            || coordinate.1 == 0
            || coordinate.1 == self.grid[0].len() - 1
    }

    fn is_enclosed(&self, row: usize, col: usize) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start_cordinate = Coordinate(row, col);

        if self.is_edge(&start_cordinate) {
            return false;
        }

        queue.push_front(start_cordinate.clone());
        visited.insert(start_cordinate);
        let changes = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(next) = queue.pop_front() {
            for (dx, dy) in changes.iter() {
                let x = next.0 as isize + *dx as isize;
                let y = next.1 as isize + *dy as isize;
                if x < 0
                    || y < 0
                    || x >= self.grid.len() as isize
                    || y >= self.grid[0].len() as isize
                {
                    continue;
                }

                let next_coordinate = Coordinate(x as usize, y as usize);
                if self.is_edge(&next_coordinate)
                    && self.grid[next_coordinate.0][next_coordinate.1] == '.'
                {
                    return false;
                }

                if self.grid[next_coordinate.0][next_coordinate.1] == '#' {
                    continue;
                }

                if visited.insert(next_coordinate.clone()) {
                    queue.push_back(next_coordinate);
                }
            }
        }

        true
    }

    fn fill_with(&mut self, row: usize, col: usize, what: char) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start_cordinate = Coordinate(row, col);
        queue.push_front(start_cordinate.clone());
        visited.insert(start_cordinate);
        let changes = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(next) = queue.pop_front() {
            self.grid[next.0][next.1] = what;
            for (dx, dy) in changes.iter() {
                let x = next.0 as isize + *dx as isize;
                let y = next.1 as isize + *dy as isize;
                if x < 0
                    || y < 0
                    || x >= self.grid.len() as isize
                    || y >= self.grid[0].len() as isize
                {
                    continue;
                }

                let next_coordinate = Coordinate(x as usize, y as usize);
                if self.grid[next_coordinate.0][next_coordinate.1] == '#' {
                    continue;
                }

                if visited.insert(next_coordinate.clone()) {
                    queue.push_back(next_coordinate);
                }
            }
        }
    }

    fn fill_enclosed_space(&mut self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == '.' {
                    let what = match self.is_enclosed(i, j) {
                        true => '#',
                        false => '-',
                    };
                    println!("At [{i} {j}] fill with {what}");
                    self.fill_with(i, j, what);
                }
            }
        }
    }

    fn count_non_empty(&self) -> usize {
        let mut count = 0;
        for row in self.grid.iter() {
            for col in row {
                if *col == '#' {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#;

        let mut builder = LagoonMakerBuilder::default();
        for each in input.split("\n") {
            builder.set_min(each, false);
        }

        let offset_coordinate = Coordinate(
            (0 - builder.min_row) as usize,
            (0 - builder.min_col) as usize,
        );

        builder.current_pos = (
            builder.current_pos.0 + offset_coordinate.0,
            builder.current_pos.1 + offset_coordinate.1,
        );

        for each in input.split("\n") {
            builder.accept_line(each, false);
        }

        let mut lagoon = Lagoon::from_patches(builder.patches, builder.max_row, builder.max_col);
        lagoon.fill_enclosed_space();

        assert_eq!(62, lagoon.count_non_empty());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day18.txt");

        let mut builder = LagoonMakerBuilder::default();
        for each in file_content.0.lines() {
            builder.set_min(each, false);
        }

        let offset_coordinate = Coordinate(
            (0 - builder.min_row) as usize,
            (0 - builder.min_col) as usize,
        );

        builder.current_pos = (
            builder.current_pos.0 + offset_coordinate.0,
            builder.current_pos.1 + offset_coordinate.1,
        );

        for each in file_content.0.lines() {
            builder.accept_line(each, false);
        }

        let mut lagoon = Lagoon::from_patches(builder.patches, builder.max_row, builder.max_col);
        lagoon.fill_enclosed_space();
        println!("Answer1 for day18 is {}", lagoon.count_non_empty());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#;

        let mut builder = LagoonMakerBuilder::default();
        for each in input.split("\n") {
            builder.set_min(each, true);
        }

        let offset_coordinate = Coordinate(
            (0 - builder.min_row) as usize,
            (0 - builder.min_col) as usize,
        );

        println!("Offset is {:?}", offset_coordinate);

        builder.current_pos = (
            builder.current_pos.0 + offset_coordinate.0,
            builder.current_pos.1 + offset_coordinate.1,
        );

        for each in input.split("\n") {
            builder.accept_line(each, true);
        }

        let mut lagoon = Lagoon::from_patches(builder.patches, builder.max_row, builder.max_col);
        lagoon.fill_enclosed_space();

        assert_eq!(952408144115, lagoon.count_non_empty());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day18.txt");

        let mut builder = LagoonMakerBuilder::default();
        for each in file_content.0.lines() {
            builder.set_min(each, true);
        }

        let offset_coordinate = Coordinate(
            (0 - builder.min_row) as usize,
            (0 - builder.min_col) as usize,
        );

        builder.current_pos = (
            builder.current_pos.0 + offset_coordinate.0,
            builder.current_pos.1 + offset_coordinate.1,
        );

        for each in file_content.0.lines() {
            builder.accept_line(each, true);
        }

        let mut lagoon = Lagoon::from_patches(builder.patches, builder.max_row, builder.max_col);
        lagoon.fill_enclosed_space();
        println!("Answer2 for day18 is {}", lagoon.count_non_empty());
    }
}
