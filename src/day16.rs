#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Element {
    Empty,
    Mirror(char),
    Splitter(char),
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Mirror(mirror) => write!(f, "{}", mirror),
            Self::Splitter(split) => write!(f, "{}", split),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Rightward,
    Leftward,
    Upward,
    Downward,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DirectionedElement(Direction, Element);

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct LavaGround {
    layout: Vec<Vec<Element>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LightPos(Direction, (usize, usize));

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NewDirection {
    Single(Direction),
    Double([Direction; 2]),
}

impl LavaGround {
    fn accept_line(&mut self, line: &str) {
        let new_row = line.trim().chars().map(Element::new).collect();
        self.layout.push(new_row);
    }

    fn next_pos(&self, direction: &Direction, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (dx, dy) = direction.next_pos();
        let next_pos = (pos.0 as isize + dx as isize, pos.1 as isize + dy as isize);
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 >= self.layout.len() as isize
            || next_pos.1 >= self.layout[0].len() as isize
        {
            return None;
        }

        Some((next_pos.0 as usize, next_pos.1 as usize))
    }

    fn count_energized_tiles(&self, start: LightPos) -> u32 {
        let mut light_at_queue = VecDeque::new();
        let mut lighted_set = HashSet::<LightPos>::new();
        light_at_queue.push_front(start.clone());
        lighted_set.insert(start);

        // neglect new pos if light has already came to that pos in given direction.
        let mut add_new_light_pos =
            |light_pos: &mut VecDeque<LightPos>, direction: &Direction, pos: (usize, usize)| {
                if let Some((dx, dy)) = self.next_pos(direction, pos) {
                    if lighted_set.insert(LightPos(direction.clone(), (dx, dy))) {
                        light_pos.push_back(LightPos(direction.clone(), (dx, dy)));
                    }
                }
            };

        while let Some(LightPos(direction, pos)) = light_at_queue.pop_front() {
            // println!("Got light at [pos {:?}] [going {:?}]", pos, direction);
            let element = &self.layout[pos.0][pos.1];
            let new_direction = element.get_new_direction(&direction);
            match new_direction {
                NewDirection::Single(new_direction) => {
                    add_new_light_pos(&mut light_at_queue, &new_direction, pos)
                }
                NewDirection::Double(new_directions) => {
                    new_directions.iter().for_each(|new_direction| {
                        add_new_light_pos(&mut light_at_queue, &new_direction, pos);
                    })
                }
            }
        }

        lighted_set
            .into_iter()
            .map(|e| e.1)
            .collect::<HashSet<_>>()
            .len() as u32
    }

    fn find_max_count_with_all_possibile_configuration(&self) -> u32 {
        use Direction::*;
        let row_count = self.layout.len();
        let col_count = self.layout[0].len();
        let mut max_conf_count = 0;
        for i in 0..row_count {
            let possible_new_count = self.count_energized_tiles(LightPos(Rightward, (i, 0)));
            max_conf_count = max_conf_count.max(possible_new_count);
            let possible_new_count =
                self.count_energized_tiles(LightPos(Leftward, (i, col_count - 1)));
            max_conf_count = max_conf_count.max(possible_new_count);
        }

        for i in 0..col_count {
            let possible_new_count = self.count_energized_tiles(LightPos(Downward, (0, i)));
            max_conf_count = max_conf_count.max(possible_new_count);
            let possible_new_count =
                self.count_energized_tiles(LightPos(Upward, (row_count - 1, i)));
            max_conf_count = max_conf_count.max(possible_new_count);
        }
        max_conf_count
    }
}

impl Debug for LavaGround {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for each in self.layout.iter() {
            writeln!(f, "{:?}", each)?;
        }

        Ok(())
    }
}

impl Element {
    fn new(ch: char) -> Element {
        use Element::*;
        match ch {
            '.' => Empty,
            '|' | '-' => Splitter(ch),
            '\\' | '/' => Mirror(ch),
            unknown @ _ => panic!("Received unknown element  {}", unknown),
        }
    }

    fn get_new_direction(&self, direction: &Direction) -> NewDirection {
        use NewDirection::*;
        match self {
            Element::Empty => Single(direction.clone()),
            Element::Mirror(tilt) => {
                let new_direction = direction.ninty_digree_angle_for_backslash_mirror();
                Single(match tilt {
                    '/' => new_direction,
                    '\\' => new_direction.opposite(),
                    _ => panic!("Unimplemented tilt"),
                })
            }
            Element::Splitter(split) => {
                use Direction::*;
                if (*direction == Upward || *direction == Downward) && *split == '|' {
                    Single(direction.clone())
                } else if (*direction == Rightward || *direction == Leftward) && *split == '-' {
                    Single(direction.clone())
                } else if *split == '|' {
                    Double([Upward, Downward])
                } else if *split == '-' {
                    Double([Leftward, Rightward])
                } else {
                    panic!("Unimplemented split")
                }
            }
        }
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Rightward => Direction::Leftward,
            Direction::Leftward => Direction::Rightward,
            Direction::Upward => Direction::Downward,
            Direction::Downward => Direction::Upward,
        }
    }

    fn ninty_digree_angle_for_backslash_mirror(&self) -> Direction {
        match self {
            Direction::Rightward => Direction::Upward,
            Direction::Leftward => Direction::Downward,
            Direction::Upward => Direction::Rightward,
            Direction::Downward => Direction::Leftward,
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

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."#;

        let mut lave_ground = LavaGround::default();
        for each in input.split("\n") {
            lave_ground.accept_line(each);
        }

        assert_eq!(
            46,
            lave_ground.count_energized_tiles(LightPos(Direction::Rightward, (0, 0)))
        );
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day16.txt");

        let mut lave_ground = LavaGround::default();
        for each in file_content.0.lines() {
            lave_ground.accept_line(each);
        }

        println!(
            "Answer1 for day16 is {}",
            lave_ground.count_energized_tiles(LightPos(Direction::Rightward, (0, 0)))
        );
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."#;

        let mut lave_ground = LavaGround::default();
        for each in input.split("\n") {
            lave_ground.accept_line(each);
        }

        assert_eq!(
            51,
            lave_ground.find_max_count_with_all_possibile_configuration()
        );
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day16.txt");

        let mut lave_ground = LavaGround::default();
        for each in file_content.0.lines() {
            lave_ground.accept_line(each);
        }

        println!(
            "Answer2 for day16 is {}",
            lave_ground.find_max_count_with_all_possibile_configuration()
        );
    }
}
