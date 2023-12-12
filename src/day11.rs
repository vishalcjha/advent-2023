#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn new(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Space {
    EMPTY,
    GALAXY,
    ADDED(Pad),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Pad {
    ONE,
    TEN,
    HUNDRED,
    MILLION,
}

impl Into<usize> for Pad {
    fn into(self) -> usize {
        match self {
            Pad::ONE => 1,
            Pad::MILLION => 1000000,
            Pad::TEN => 10,
            Pad::HUNDRED => 100,
        }
    }
}

impl Space {
    fn new(space: char) -> Space {
        use Space::*;
        match space {
            '.' => EMPTY,
            '#' => GALAXY,
            unknown @ _ => panic!("Received invalid input for space {:?}", unknown),
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::EMPTY | Self::ADDED(_) => '*',
                Space::GALAXY => '#',
            }
        )
    }
}

#[derive(Debug, Default)]
struct Cosmos {
    space: Vec<Vec<Space>>,
}

impl Cosmos {
    fn accept_new_line(&mut self, line: &str) {
        let new_space = line.trim().chars().map(|each| Space::new(each)).collect();
        self.space.push(new_space);
    }

    fn find_all_empty_columns(&self) -> Vec<usize> {
        let mut empty_cols = Vec::new();
        for i in 0..self.space[0].len() {
            let mut empty = true;
            for j in 0..self.space.len() {
                if *&self.space[j][i] == Space::GALAXY {
                    empty = false;
                    break;
                }
            }

            if empty {
                empty_cols.push(i);
            }
        }

        empty_cols
    }

    fn fina_all_empty_rows(&self) -> Vec<usize> {
        let mut empty_rows = Vec::new();
        for (index, row) in self.space.iter().enumerate() {
            if row.iter().all(|space| *space == Space::EMPTY) {
                empty_rows.push(index);
            }
        }

        empty_rows
    }

    fn expand(self, pad: Pad) -> Cosmos {
        let empty_space_columns = self.find_all_empty_columns();
        let empty_space_rows = self.fina_all_empty_rows();
        let total_col = self.space[0].len() + empty_space_columns.len();
        let mut space = Vec::<Vec<Space>>::new();
        for (index, row) in self.space.into_iter().enumerate() {
            if empty_space_rows.contains(&index) {
                space.push((0..total_col).into_iter().map(|_| Space::EMPTY).collect());
                space.push(
                    (0..total_col)
                        .into_iter()
                        .map(|_| Space::ADDED(pad.clone()))
                        .collect(),
                );
                continue;
            }

            let mut new_row = Vec::new();
            for (sub_index, space) in row.into_iter().enumerate() {
                new_row.push(space);
                if empty_space_columns.contains(&sub_index) {
                    new_row.push(Space::ADDED(pad.clone()));
                }
            }
            space.push(new_row);
        }

        Cosmos { space }
    }

    fn neighbors(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        for (rc, cc) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            //for (rc, cc) in vec![(0, 1), (1, 0)] {
            if (cc == -1 && coordinate.col == 0) || (rc == -1 && coordinate.row == 0) {
                continue;
            }

            let row = (coordinate.row as i64 + rc) as usize;
            let col = (coordinate.col as i64 + cc) as usize;

            if row < self.space.len() && col < self.space[0].len() {
                neighbors.push(Coordinate::new(row, col));
            }
        }

        neighbors
    }

    fn find_shortest_path_distance_matrix(&self) -> Vec<Vec<Option<usize>>> {
        todo!()
    }

    fn find_shortest_path_to_all_galaxy_from(&self, coordinate: Coordinate) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((coordinate.clone(), 0));
        let mut visited = HashSet::new();
        visited.insert(coordinate.clone());
        let mut sum = 0;

        while let Some((cord, dis)) = queue.pop_front() {
            let space = &self.space[cord.row][cord.col];
            let to_add: usize = match space {
                Space::EMPTY => 1,
                Space::GALAXY => 1,
                Space::ADDED(pad) => pad.clone().into(),
            };

            for neighbor in self.neighbors(&cord) {
                let possible_cord = neighbor;
                if visited.insert(possible_cord.clone()) {
                    if self.space[possible_cord.row][possible_cord.col] == Space::GALAXY
                    // prevent from over count
                        && coordinate < possible_cord
                    {
                        println!(
                            "B/w [{:?} - {:?}] => {:?} ",
                            coordinate,
                            possible_cord,
                            dis + to_add
                        );
                        sum += dis + to_add;
                    }
                    queue.push_back((possible_cord, dis + to_add));
                }
            }
        }

        sum
    }

    fn find_shortest_path_sum_between_galaxies(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.space.len() {
            for col in 0..self.space[0].len() {
                if *&self.space[row][col] == Space::GALAXY {
                    sum += self.find_shortest_path_to_all_galaxy_from(Coordinate::new(row, col));
                }
            }
        }

        sum
    }
}

impl Display for Cosmos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.space.iter() {
            for each in row.iter() {
                write!(f, "{}", each)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#;

        let mut consmos = Cosmos::default();
        for each in input.split("\n") {
            consmos.accept_new_line(each);
        }

        consmos = consmos.expand(Pad::ONE);

        assert_eq!(374, consmos.find_shortest_path_sum_between_galaxies());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day11.txt");

        let mut consmos = Cosmos::default();
        for each in file_content.0.lines() {
            consmos.accept_new_line(each);
        }
        consmos = consmos.expand(Pad::ONE);

        println!(
            "Answer1 for day11 is {}",
            consmos.find_shortest_path_sum_between_galaxies()
        );
    }

    #[test]
    fn test_second_ten_with_local_data() {
        let input = r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#;

        let mut consmos = Cosmos::default();
        for each in input.split("\n") {
            consmos.accept_new_line(each);
        }

        consmos = consmos.expand(Pad::TEN);

        assert_eq!(1030, consmos.find_shortest_path_sum_between_galaxies());
    }

    #[test]
    fn test_second_hundred_with_local_data() {
        let input = r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#;

        let mut consmos = Cosmos::default();
        for each in input.split("\n") {
            consmos.accept_new_line(each);
        }

        consmos = consmos.expand(Pad::HUNDRED);

        assert_eq!(8410, consmos.find_shortest_path_sum_between_galaxies());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day11.txt");

        let mut consmos = Cosmos::default();
        for each in file_content.0.lines() {
            consmos.accept_new_line(each);
        }
        consmos = consmos.expand(Pad::MILLION);

        println!(
            "Answer1 for day11 is {}",
            consmos.find_shortest_path_sum_between_galaxies()
        );
    }
}
