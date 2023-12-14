#![allow(dead_code)]

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pattern {
    row_based: Vec<String>,
    col_based: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MirrorPoint {
    Row(u32),
    Col(u32),
}

impl Pattern {
    fn accept_line(&mut self, line: impl Into<String>) {
        self.row_based.push(line.into());
    }

    fn finalize(&mut self) {
        let mut iters = self
            .row_based
            .iter()
            .map(|it| it.chars())
            .collect::<Vec<_>>();

        for _ in 0..self.row_based[0].len() {
            let col = iters.iter_mut().map(|it| it.next().unwrap()).collect();
            self.col_based.push(col);
        }
    }

    fn find_mirror_for(pattern: &Vec<String>, with_smudge: bool) -> Option<u32> {
        for i in 1..pattern.len() {
            let (first, second) = pattern.split_at(i);
            let mut mismached_count = 0;
            for (first, second) in first.iter().rev().zip(second.iter()) {
                if mismached_count > 1 {
                    break;
                }
                mismached_count += first
                    .chars()
                    .zip(second.chars())
                    .filter(|(first, second)| *first != *second)
                    .count()
            }

            if (!with_smudge && mismached_count == 0) || (with_smudge && mismached_count == 1) {
                return Some(i as u32);
            }
        }

        None
    }

    fn find_mirror(&self, with_smudge: bool) -> MirrorPoint {
        if let Some(place) = Pattern::find_mirror_for(&self.col_based, with_smudge) {
            return MirrorPoint::Col(place);
        }

        let Some(place) = Pattern::find_mirror_for(&self.row_based, with_smudge) else {
            panic!("No pattern found for {:?}", self);
        };

        MirrorPoint::Row(place)
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MirrorFinder {
    patterns: Vec<Pattern>,
    current: Option<Pattern>,
}

impl MirrorFinder {
    fn accept_line(&mut self, line: impl Into<String>) {
        let line: String = line.into();
        let line = line.trim();

        if line.is_empty() {
            if let Some(mut pattern) = self.current.take() {
                pattern.finalize();
                self.patterns.push(pattern);
            }
        } else {
            if self.current == None {
                self.current = Some(Pattern::default());
            }
            self.current.as_mut().unwrap().accept_line(line);
        }
    }

    fn finalize(&mut self) {
        if let Some(mut pattern) = self.current.take() {
            pattern.finalize();
            self.patterns.push(pattern);
        }
    }

    fn sumarize_mirros(&self, with_smudge: bool) -> u32 {
        self.patterns
            .iter()
            .map(|pattern| match pattern.find_mirror(with_smudge) {
                MirrorPoint::Row(pos) => {
                    println!("Got row for {pos}");
                    pos * 100
                }
                MirrorPoint::Col(pos) => {
                    println!("Got col for {pos}");
                    pos
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"#;

        let mut mirror_finder = MirrorFinder::default();
        for each in input.split("\n") {
            mirror_finder.accept_line(each);
        }
        mirror_finder.finalize();

        assert_eq!(405, mirror_finder.sumarize_mirros(false));
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day13.txt");

        let mut mirror_finder = MirrorFinder::default();
        for each in file_content.0.lines() {
            mirror_finder.accept_line(each);
        }
        mirror_finder.finalize();

        println!(
            "Answer1 for day13 is {}",
            mirror_finder.sumarize_mirros(false)
        );
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"#;

        let mut mirror_finder = MirrorFinder::default();
        for each in input.split("\n") {
            mirror_finder.accept_line(each);
        }
        mirror_finder.finalize();

        assert_eq!(400, mirror_finder.sumarize_mirros(true));
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day13.txt");

        let mut mirror_finder = MirrorFinder::default();
        for each in file_content.0.lines() {
            mirror_finder.accept_line(each);
        }
        mirror_finder.finalize();

        println!(
            "Answer2 for day13 is {}",
            mirror_finder.sumarize_mirros(true)
        );
    }
}
