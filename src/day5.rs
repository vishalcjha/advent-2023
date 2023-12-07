#![allow(dead_code)]

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

#[derive(Debug)]
struct SeedConversion {
    seeds: Vec<u128>,
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
            self.seeds = line
                .split(":")
                .last()
                .unwrap()
                .split_ascii_whitespace()
                .filter_map(|num| num.parse::<u128>().ok())
                .collect();
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
            let conversion = self
                .get_conversion_by_seed(*seed, "location".to_owned())
                .unwrap();
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

    fn get_conversion_by_seed(&self, seed: u128, dst: String) -> Option<u128> {
        let mut looking_for = seed;
        for (current_dict, current_dst) in self
            .conversion_distonary
            .iter()
            .zip(self.conversion_sequence.iter().skip(1))
        {
            for src_dst_len in current_dict {
                if looking_for >= src_dst_len.src && looking_for < src_dst_len.src_end {
                    let diff = looking_for - src_dst_len.src;
                    looking_for = src_dst_len.dst + diff;
                    break;
                }
            }
            if current_dst == &dst {
                return Some(looking_for);
            }
        }

        None
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

        assert_eq!(35, seed_conversion.get_min_location());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day5.txt");

        let mut seed_conversion = SeedConversion::default();
        for each in file_content.0.lines() {
            let _ = seed_conversion.process_line(each, true);
        }

        println!("Answer1 for day5 is {}", seed_conversion.get_min_location());
    }
}
