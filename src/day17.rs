#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Rightward,
    Leftward,
    Downward,
    Upward,
}

struct MinMaxStep {
    min: usize,
    max: usize,
}

impl Default for MinMaxStep {
    fn default() -> Self {
        Self { min: 1, max: 3 }
    }
}

impl Direction {
    fn possible_directions(&self) -> [Direction; 3] {
        use Direction::*;
        match self {
            Rightward => [Rightward, Upward, Downward],
            Leftward => [Leftward, Downward, Upward],
            Downward => [Downward, Leftward, Rightward],
            Upward => [Upward, Rightward, Leftward],
        }
    }

    fn next_pos(&self) -> (i32, i32) {
        match self {
            Direction::Rightward => (0, 1),
            Direction::Leftward => (0, -1),
            Direction::Upward => (-1, 0),
            Direction::Downward => (1, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    direction: Direction,
    coordinate: (usize, usize),
    count_in_same_dir: usize,
}

impl Pos {
    fn new(direction: Direction, coordinate: (usize, usize), count_in_same_dir: usize) -> Pos {
        Pos {
            direction,
            coordinate,
            count_in_same_dir,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct ClumsyCrucible {
    field: Vec<Vec<u32>>,
}

impl ClumsyCrucible {
    fn accept_line(&mut self, line: &str) {
        let line = line
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        self.field.push(line);
    }

    fn heat_at(&self, coordinate: (usize, usize)) -> u32 {
        self.field[coordinate.0][coordinate.1]
    }

    fn next_pos(&self, current: (usize, usize), change: (i32, i32)) -> Option<(usize, usize)> {
        let row = current.0 as isize + change.0 as isize;
        let col = current.1 as isize + change.1 as isize;

        if row < 0
            || col < 0
            || row >= self.field.len() as isize
            || col >= self.field[0].len() as isize
        {
            return None;
        }

        Some((row as usize, col as usize))
    }

    fn calculate_min_heat_loss(&self, min_max: MinMaxStep) -> u32 {
        let starting_pos = Pos::new(Direction::Rightward, (0, 0), 0);
        let mut visited = HashMap::from([(starting_pos.clone(), 0)]);

        let mut queue = VecDeque::new();
        queue.push_back((starting_pos, 0));
        let max_row = self.field.len();
        let max_col = self.field[0].len();
        let mut min_heat = u32::MAX;
        while let Some(current) = queue.pop_front() {
            let (
                Pos {
                    direction,
                    coordinate,
                    count_in_same_dir,
                },
                heat,
            ) = current;

            if coordinate.0 == max_row - 1 && coordinate.1 == max_col - 1 {
                min_heat = min_heat.min(heat);
                continue;
            }

            for next_direction in direction.possible_directions() {
                if next_direction == direction && count_in_same_dir >= min_max.max {
                    continue;
                }

                // starting point does not have any step. So no need to force min check.
                if coordinate != (0, 0)
                    && next_direction != direction
                    && count_in_same_dir < min_max.min
                {
                    continue;
                }

                let count_in_same_dir = if next_direction == direction {
                    count_in_same_dir + 1
                } else {
                    1
                };

                let pos_change = next_direction.next_pos();
                let Some(next_coordinate) = self.next_pos(coordinate, pos_change) else {
                    continue;
                };

                let next_pos = Pos::new(next_direction, next_coordinate, count_in_same_dir);
                let heat = heat + self.heat_at(next_coordinate);
                if let Some(visited_heat) = visited.get(&next_pos) {
                    if *visited_heat <= heat {
                        continue;
                    }
                }

                if heat > min_heat {
                    continue;
                }

                visited.insert(next_pos.clone(), heat);
                queue.push_back((next_pos, heat));
            }
        }

        min_heat
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"#;

        let mut clumsy_crucible = ClumsyCrucible::default();
        for each in input.split("\n") {
            clumsy_crucible.accept_line(each);
        }

        assert_eq!(
            102,
            clumsy_crucible.calculate_min_heat_loss(MinMaxStep::default())
        );
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day17.txt");

        let mut clumsy_crucible = ClumsyCrucible::default();
        for each in file_content.0.lines() {
            clumsy_crucible.accept_line(each);
        }

        println!(
            "Answer1 for day17 is {}",
            clumsy_crucible.calculate_min_heat_loss(MinMaxStep::default())
        );
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"#;

        let mut clumsy_crucible = ClumsyCrucible::default();
        for each in input.split("\n") {
            clumsy_crucible.accept_line(each);
        }

        assert_eq!(
            94,
            clumsy_crucible.calculate_min_heat_loss(MinMaxStep { min: 4, max: 10 })
        );
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day17.txt");

        let mut clumsy_crucible = ClumsyCrucible::default();
        for each in file_content.0.lines() {
            clumsy_crucible.accept_line(each);
        }

        println!(
            "Answer1 for day17 is {}",
            clumsy_crucible.calculate_min_heat_loss(MinMaxStep { min: 4, max: 10 })
        );
    }
}
