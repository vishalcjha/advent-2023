#![allow(dead_code)]

use std::{collections::HashMap, time::SystemTime};

use anyhow::anyhow;
use nom::{bytes::complete::is_not, bytes::complete::tag, sequence::delimited, IResult};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(dir: char) -> Direction {
        if dir == 'L' {
            return Direction::Left;
        }

        assert!(dir == 'R');
        Direction::Right
    }
}

#[derive(Debug)]
struct PathFinder {
    path_guide: Vec<Direction>,
    map: HashMap<String, Vec<String>>,
}

impl Default for PathFinder {
    fn default() -> Self {
        PathFinder {
            path_guide: Vec::new(),
            map: HashMap::new(),
        }
    }
}

impl PathFinder {
    pub(self) fn parse_delimiter(input: &str) -> IResult<&str, &str> {
        delimited(tag("("), is_not(")"), tag(")"))(input)
    }

    pub fn accept_line(&mut self, line: &str) -> anyhow::Result<()> {
        if line.trim().is_empty() {
            return Ok(());
        }

        if self.path_guide.is_empty() {
            for each in line.chars() {
                self.path_guide.push(Direction::new(each));
            }
            return Ok(());
        }

        let mut first_split = line.split("=");
        let source = first_split
            .next()
            .ok_or(anyhow!("Error in = split"))?
            .trim();
        let rest = first_split
            .next()
            .ok_or(anyhow!("Could not find child"))?
            .trim();

        let children = PathFinder::parse_delimiter(rest).unwrap().1;
        let entry = self.map.entry(source.to_string()).or_insert(Vec::new());
        for child in children.split(",") {
            entry.push(child.trim().to_string());
        }

        Ok(())
    }

    pub fn find_hop_count(&self) -> u32 {
        let mut current = "AAA";
        let mut hop_count = 0;
        loop {
            for direction in self.path_guide.iter() {
                let path_list = self.map.get(current).unwrap();
                current = match direction {
                    Direction::Left => &path_list[0],
                    Direction::Right => &path_list[1],
                };

                hop_count += 1;
                if current == "ZZZ" {
                    return hop_count;
                }
            }
        }
    }

    pub fn find_multi_hop_count_me_dumb(&self) -> usize {
        let mut current_pos = self
            .map
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect::<Vec<_>>();

        // for given key, when start with loop, which pos result in Z points and what is end point.
        let mut interesting_points = HashMap::<&String, (Vec<usize>, &String)>::new();
        for starting_point in self.map.keys() {
            let mut current = starting_point;
            let mut points_with_z = Vec::new();
            for (index, direction) in self.path_guide.iter().enumerate() {
                let pos = match direction {
                    Direction::Left => 0,
                    Direction::Right => 1,
                };
                let possible_places = self.map.get(current.as_str()).unwrap();
                current = possible_places.get(pos).unwrap();
                if current.ends_with("Z") {
                    points_with_z.push(index);
                }
            }
            interesting_points.insert(&starting_point, (points_with_z, current));
        }

        let mut loop_count = 0;
        loop {
            current_pos.sort();
            println!(
                "Running loop for {} with start {:?} {:?}",
                loop_count,
                current_pos,
                SystemTime::now().elapsed().ok().unwrap()
            );
            let mut z_point_pos_count = HashMap::new();
            for start in current_pos.iter_mut() {
                let (z_points, end) = interesting_points.get(start).unwrap();
                *start = *end;
                for z_point in z_points.into_iter() {
                    z_point_pos_count
                        .entry(*z_point)
                        .and_modify(|current| *current += 1)
                        .or_insert(1);
                }
            }
            let mut found_pos: Option<usize> = None;
            for z_point in z_point_pos_count {
                if z_point.1 == current_pos.len() {
                    match found_pos.as_mut() {
                        Some(existing) => {
                            if *existing > z_point.0 {
                                *existing = z_point.0;
                            }
                        }
                        None => found_pos = Some(z_point.0),
                    }
                }
            }

            if let Some(matched) = found_pos {
                return loop_count * self.path_guide.len() + matched + 1;
            }
            loop_count += 1;
        }
    }

    // after 90 mins i looked up internet for solution. I am still not sure why it works.
    pub fn find_multi_hop_count_after_hint(&self) -> u64 {
        let starting_pos = self
            .map
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect::<Vec<_>>();

        let mut z_positions = Vec::new();
        for each in starting_pos.iter() {
            let mut count = 0_u64;
            let mut current = *each;
            let mut found = false;
            while !found {
                for direction in self.path_guide.iter() {
                    count += 1;
                    let index = match direction {
                        Direction::Left => 0,
                        Direction::Right => 1,
                    };
                    current = self.map.get(current).unwrap().get(index).unwrap();
                    if current.ends_with("Z") {
                        z_positions.push(count);
                        found = true;
                        break;
                    }
                }
            }
        }
        assert!(z_positions.len() == starting_pos.len());
        if z_positions.len() == 1 {
            return z_positions.pop().unwrap();
        }

        let mut lcm = num_integer::lcm(z_positions[0], z_positions[1]);

        for num in z_positions.iter().skip(2) {
            lcm = num_integer::lcm(lcm, *num);
        }

        lcm
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"#;

        let mut path_finder = PathFinder::default();
        for each in input.split("\n") {
            let _ = path_finder.accept_line(each);
        }

        assert_eq!(6, path_finder.find_hop_count());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day8.txt");

        let mut path_finder = PathFinder::default();
        for each in file_content.0.lines() {
            let _ = path_finder.accept_line(each);
        }

        println!("Answer1 for day8 is {}", path_finder.find_hop_count());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"#;

        let mut path_finder = PathFinder::default();
        for each in input.split("\n") {
            let _ = path_finder.accept_line(each);
        }

        assert_eq!(6, path_finder.find_multi_hop_count_after_hint());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day8.txt");

        let mut path_finder = PathFinder::default();
        for each in file_content.0.lines() {
            let _ = path_finder.accept_line(each);
        }

        println!(
            "Answer2 for day8 is {}",
            path_finder.find_multi_hop_count_after_hint()
        );
    }
}
