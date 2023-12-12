#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

lazy_static! {
    static ref DIRECTION_MAP: HashMap<Pipe, Vec<(i8, i8)>> = {
        use Pipe::*;
        let mut m = HashMap::new();
        m.insert(Vertical, vec![(-1, 0), (1, 0)]);
        m.insert(Horizontal, vec![(0, -1), (0, 1)]);

        m.insert(NorthEast, vec![(-1, 0), (0, 1)]);
        m.insert(NorthWest, vec![(-1, 0), (0, -1)]);

        m.insert(SouthEast, vec![(1, 0), (0, 1)]);
        m.insert(SouthWest, vec![(1, 0), (0, -1)]);

        m.insert(Ground, Vec::new());

        m
    };
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPoint,
}

impl Pipe {
    fn new(in_char: char) -> Pipe {
        match in_char {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::StartingPoint,
            unknown @ _ => panic!("Unknown pipe {}", unknown),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coordinate(usize, usize);

#[derive(Debug, Default)]
struct Maze {
    pipes: Vec<Vec<Pipe>>,
    starting_point: Option<(usize, usize)>,
}

impl Maze {
    fn accept_line(&mut self, line: &str) {
        let line = line.trim();

        if let Some(starting_point) = line.find('S') {
            self.starting_point = Some((self.pipes.len(), starting_point));
        }

        let new_line = line.chars().map(|ch| Pipe::new(ch)).collect();
        self.pipes.push(new_line);
    }

    fn is_valid(&self, row: i32, col: i32) -> bool {
        row >= 0
            && col >= 0
            && (row as usize) < self.pipes.len()
            && (col as usize) < self.pipes[0].len()
    }

    fn reachable_coordinates(&self, row: usize, col: usize) -> Vec<Rc<Coordinate>> {
        let pipe = &self.pipes[row][col];

        let mut visitable = Vec::new();
        let Some(directions) =  DIRECTION_MAP.get(pipe) else {
            return visitable;
        };

        for (rc, cc) in directions.iter() {
            let next_row = row as i32 + *rc as i32;
            let next_col = col as i32 + *cc as i32;

            if !self.is_valid(next_row, next_col) {
                continue;
            }

            visitable.push(Rc::new(Coordinate(next_row as usize, next_col as usize)));
        }

        visitable
    }

    fn find_max_distance(&self) -> (u32, HashSet<Rc<Coordinate>>) {
        let mut visited = HashSet::<Rc<Coordinate>>::new();

        let mut coordinates = VecDeque::<Rc<Coordinate>>::new();
        let Some(starting_point) = self.starting_point else {
            return (0, visited);
        };

        let starting_point = Rc::new(Coordinate(starting_point.0, starting_point.1));
        visited.insert(starting_point.clone());
        for change in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = starting_point.0 as i32 + change.0;
            let next_col = starting_point.1 as i32 + change.1;
            if !self.is_valid(next_row, next_col) {
                continue;
            }

            for visitable in self.reachable_coordinates(next_row as usize, next_col as usize) {
                if visitable == starting_point {
                    let initial_coordinate =
                        Rc::new(Coordinate(next_row as usize, next_col as usize));
                    coordinates.push_back(initial_coordinate.clone());
                    visited.insert(initial_coordinate);
                }
            }
        }

        coordinates.push_back(starting_point.clone());
        visited.insert(starting_point.clone());

        let mut max_distance = 0;
        while !coordinates.is_empty() {
            max_distance += 1;
            let mut new_coordinates = VecDeque::new();
            while let Some(next) = coordinates.pop_back() {
                for visitable in self.reachable_coordinates(next.0, next.1) {
                    if visited.insert(visitable.clone()) {
                        new_coordinates.push_back(visitable);
                    }
                }
            }

            coordinates = new_coordinates;
        }

        (max_distance, visited)
    }

    fn find_enclosed_ground_tile_count(&self) -> u32 {
        let main_loop = self.find_max_distance().1;
        let visited = RefCell::new(HashSet::<Coordinate>::new());

        let is_edge = |coordinate: &Coordinate| -> bool {
            coordinate.0 == 0
                || coordinate.0 == self.pipes.len() - 1
                || coordinate.1 == 0
                || coordinate.1 == self.pipes[0].len() - 1
        };

        // To be inside closed pipe, number of intersection with path has to be odd.
        let is_closed_loop = |coordinate: Coordinate| -> bool {
            for change in vec![(1, 0), (0, 1)] {
                let mut coordinate = coordinate.clone();
                let mut cross_count = 0;
                // help not to count when we walking one pipe after another.i.e. when in closed area.
                let mut out = true;
                while self.is_valid(coordinate.0 as i32, coordinate.1 as i32) {
                    coordinate = Coordinate(coordinate.0 + change.0, coordinate.1 + change.1);
                    if main_loop.contains(&coordinate) {
                        if out {
                            cross_count += 1;
                            out = false;
                        }
                    } else {
                        out = true;
                    }
                }

                //println!("Cross count for {:?} is {:?}", coordinate, cross_count);
                if cross_count % 2 == 1 {
                    return true;
                }
            }
            false
        };

        let visit_all_connected_not_in_path = |coordinate: Coordinate| -> Option<u32> {
            let mut queue = VecDeque::new();
            queue.push_back(coordinate.clone());
            let mut found_edge = false;
            let mut count = 0;
            while let Some(next) = queue.pop_front() {
                count += 1;
                if is_edge(&next) {
                    found_edge = true;
                }
                for change in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let next_row = next.0 as i32 + change.0;
                    let next_col = next.1 as i32 + change.1;

                    if self.is_valid(next_row, next_col) {
                        let next = Coordinate(next_row as usize, next_col as usize);
                        if main_loop.contains(&next) {
                            continue;
                        }
                        if visited.borrow_mut().insert(next.clone()) {
                            queue.push_back(next.clone());
                        }
                    }
                }
            }

            if found_edge || !is_closed_loop(coordinate) {
                None
            } else {
                Some(count)
            }
        };
        let mut enclosed_tile_count = 0;

        for (r_index, row) in self.pipes.iter().enumerate() {
            for (c_index, _) in row.iter().enumerate() {
                if !main_loop.contains(&Coordinate(r_index, c_index))
                    && visited.borrow_mut().insert(Coordinate(r_index, c_index))
                {
                    if let Some(enclosed) =
                        visit_all_connected_not_in_path(Coordinate(r_index, c_index))
                    {
                        enclosed_tile_count += enclosed;
                    }
                }
            }
        }

        enclosed_tile_count
    }
}
#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#".....
        .S-7.
        .|.|.
        .L-J.
        ....."#;

        let mut maze = Maze::default();
        for each in input.split("\n") {
            maze.accept_line(each);
        }

        assert_eq!(4, maze.find_max_distance().0);
    }

    #[test]
    fn test_first_second_with_local_data() {
        let input = r#"..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ..."#;

        let mut maze = Maze::default();
        for each in input.split("\n") {
            maze.accept_line(each);
        }

        assert_eq!(8, maze.find_max_distance().0);
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day10.txt");

        let mut maze = Maze::default();
        for each in file_content.0.lines() {
            maze.accept_line(each);
        }

        println!("Answer1 for day10 is {}", maze.find_max_distance().0);
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"#;

        let mut maze = Maze::default();
        for each in input.split("\n") {
            maze.accept_line(each);
        }

        assert_eq!(10, maze.find_enclosed_ground_tile_count());
    }

    #[test]
    fn test_second_variant_1_with_local_data() {
        let input = r#"...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ..........."#;

        let mut maze = Maze::default();
        for each in input.split("\n") {
            maze.accept_line(each);
        }

        assert_eq!(4, maze.find_enclosed_ground_tile_count());
    }

    #[test]
    fn test_second_variant_2_with_local_data() {
        let input = r#".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ..."#;

        let mut maze = Maze::default();
        for each in input.split("\n") {
            maze.accept_line(each);
        }

        assert_eq!(8, maze.find_enclosed_ground_tile_count());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day10.txt");

        let mut maze = Maze::default();
        for each in file_content.0.lines() {
            maze.accept_line(each);
        }

        println!(
            "Answer2 for day10 is {}",
            maze.find_enclosed_ground_tile_count()
        );
    }
}
