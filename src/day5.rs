#![allow(dead_code)]

use itertools::Itertools;
#[derive(Debug)]
struct SourceDestLen {
    src: u128,
    dst: u128,
    len: u128,
    src_end: u128,
}

impl SourceDestLen {
    fn new(src: u128, dst: u128, len: u128) -> Self {
        SourceDestLen {
            src,
            dst,
            len,
            src_end: src + len,
        }
    }
}

#[derive(Debug, Clone)]
struct Range(u128, u128, u128);

#[derive(Debug)]
struct SeedConversion {
    seeds: Vec<Range>,
    conversion_sequence: Vec<String>,
    conversion_distonary: Vec<Vec<SourceDestLen>>,
    new_map_coming: bool,
}

impl SeedConversion {
    fn process_line(&mut self, line: &str, seed_as_pair: bool) {
        let line = line.trim();
        if line.is_empty() {
            return;
        }
        if line.starts_with("seeds") {
            self.conversion_sequence.push("seeds".to_owned());
            let iter = line
                .split(":")
                .last()
                .unwrap()
                .split_ascii_whitespace()
                .filter_map(|num| num.parse::<u128>().ok());
            let mut seeds = Vec::new();
            if seed_as_pair {
                for from_to in &iter.chunks(2) {
                    let mut from_to = from_to.into_iter();
                    let src = from_to.next().unwrap();
                    let len = from_to.next().unwrap();
                    seeds.push(Range(src, src + len, len));
                }
            } else {
                for num in iter {
                    seeds.push(Range(num, num + 1, 1));
                }
            }

            self.seeds = seeds;
            return;
        }

        if line.chars().next().unwrap().is_alphabetic() {
            let next_conversion = line
                .split("-")
                .last()
                .unwrap()
                .split_ascii_whitespace()
                .next()
                .unwrap();
            self.conversion_sequence.push(next_conversion.to_owned());
            return;
        }

        if self.conversion_sequence.len() - self.conversion_distonary.len() > 1 {
            self.conversion_distonary.push(Vec::new());
        }

        let mut mapping = line
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<u128>().ok());

        let (dst, src, len) = (
            mapping.next().unwrap(),
            mapping.next().unwrap(),
            mapping.next().unwrap(),
        );
        let dict = self.conversion_distonary.last_mut().unwrap();
        dict.push(SourceDestLen::new(src, dst, len));
    }

    pub fn get_min_location(&self) -> u128 {
        let mut min_location = None;
        for seed in self.seeds.iter() {
            let conversion = self.get_min_conversion(seed, 0);
            match min_location.as_mut() {
                Some(v) => {
                    if conversion < *v {
                        *v = conversion;
                    }
                }
                None => {
                    min_location = Some(conversion);
                }
            }
        }

        min_location.unwrap()
    }

    pub(self) fn find_overlap(&self, src: &Range, dst: &SourceDestLen) -> Option<(Range, Range)> {
        if dst.src >= src.1 || dst.src + dst.len <= src.0 {
            return None;
        }

        let start = src.0.max(dst.src);
        let end = (src.1).min(dst.src + dst.len);
        let offset = start - dst.src;
        let len = end - start;
        if len == 0 {
            panic!("Going to do bad {:?} {:?}", src, dst);
        }
        Some((
            Range(dst.dst + offset, dst.dst + offset + len, len),
            Range(dst.src + offset, dst.src + offset + len, len),
        ))
    }

    pub(self) fn find_missing_ranges(&self, src: &Range, found_ranges: &Vec<Range>) -> Vec<Range> {
        if found_ranges.is_empty() {
            return vec![src.clone()];
        }

        let mut missing_range = Vec::new();

        if found_ranges[0].0 > src.0 {
            missing_range.push(Range(src.0, found_ranges[0].0, found_ranges[0].0 - src.0));
        }

        for (index, found) in found_ranges.iter().skip(1).enumerate() {
            if found.0 > found_ranges[index].1 {
                missing_range.push(Range(
                    found_ranges[index].1,
                    found.0 + 1,
                    found.0 + 1 - found_ranges[index].1,
                ));
            }
        }

        let last_range = found_ranges.last().unwrap();

        if last_range.1 < src.1 {
            missing_range.push(Range(last_range.1, src.1, src.1 - last_range.1));
        }

        missing_range
    }

    pub fn get_min_conversion(&self, items: &Range, dict_pos: u8) -> u128 {
        if dict_pos >= self.conversion_distonary.len() as u8 {
            return items.0;
        }

        let dict = &self.conversion_distonary[dict_pos as usize];

        let mut sub_ranges = Vec::new();
        let mut matched_ranges = Vec::new();
        for src_dst_len in dict {
            if let Some((sub_range, matched_sub_ranges)) = self.find_overlap(&items, src_dst_len) {
                sub_ranges.push(sub_range);
                matched_ranges.push(matched_sub_ranges);
            }
        }

        matched_ranges.sort_by_key(|range| range.0);
        sub_ranges.append(&mut self.find_missing_ranges(&items, &matched_ranges));
        sub_ranges.sort_by_key(|range| range.0);

        sub_ranges
            .into_iter()
            .map(|range| self.get_min_conversion(&range, dict_pos + 1))
            .min()
            .unwrap()
    }
}

impl Default for SeedConversion {
    fn default() -> Self {
        SeedConversion {
            seeds: Vec::new(),
            conversion_sequence: Vec::new(),
            conversion_distonary: Vec::new(),
            new_map_coming: true,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;

    #[test]
    fn test_first_with_local_data() {
        let input = r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#;

        let mut seed_conversion = SeedConversion::default();
        for each in input.split("\n") {
            let _ = seed_conversion.process_line(each, false);
        }

        assert_eq!(35, seed_conversion.get_min_location());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day5.txt");

        let mut seed_conversion = SeedConversion::default();
        for each in file_content.0.lines() {
            let _ = seed_conversion.process_line(each, false);
        }

        println!("Answer1 for day5 is {}", seed_conversion.get_min_location());
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#;

        let mut seed_conversion = SeedConversion::default();
        for each in input.split("\n") {
            let _ = seed_conversion.process_line(each, true);
        }

        assert_eq!(46, seed_conversion.get_min_location());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day5.txt");

        let mut seed_conversion = SeedConversion::default();
        for each in file_content.0.lines() {
            let _ = seed_conversion.process_line(each, true);
        }

        println!("Answer2 for day5 is {}", seed_conversion.get_min_location());
    }
}
