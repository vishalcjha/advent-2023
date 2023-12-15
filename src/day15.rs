#![allow(dead_code)]

use nom::{bytes::complete::tag, character::streaming::alpha1, sequence::tuple, IResult};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct LabelFocalLength(String, u8);

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Box {
    lenses: Vec<LabelFocalLength>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct LensLibrary {
    boxes: [Box; 256],
}

impl LensLibrary {
    fn new() -> LensLibrary {
        LensLibrary {
            boxes: std::array::from_fn(|_| Box::default()),
        }
    }

    fn label_extractor(token: &str) -> IResult<&str, (&str, &str)> {
        use nom::branch::alt;

        tuple((alpha1, alt((tag("="), tag("-")))))(token)
    }

    fn process_token(&mut self, token: &str) {
        let (rest, (label, action)) = LensLibrary::label_extractor(token).unwrap();
        let lebel_hash = HashCalculator::calculate(label);
        let lens_box = self.boxes.get_mut(lebel_hash as usize).unwrap();
        match action {
            "=" => {
                let focal_length = rest.parse::<u8>().unwrap();
                if let Some(found) = lens_box.lenses.iter_mut().find(|value| value.0 == label) {
                    found.1 = focal_length;
                } else {
                    lens_box
                        .lenses
                        .push(LabelFocalLength(String::from(label), focal_length));
                }
            }
            "-" => {
                lens_box.lenses.retain(|value| value.0 != label);
            }
            unknown @ _ => panic!("Received {unknown} during token processing"),
        }
    }

    fn focus_power(&self) -> usize {
        let mut focal_power = 0;
        for (box_num, each) in self.boxes.iter().enumerate() {
            let box_num = box_num + 1;
            for (slot_num, lens) in each.lenses.iter().enumerate() {
                let slot_num = slot_num + 1;

                focal_power += box_num * slot_num * lens.1 as usize;
            }
        }
        focal_power
    }
}

struct HashCalculator {}

impl HashCalculator {
    fn calculate(each: &str) -> u16 {
        each.chars().fold(0, |mut accum, current| {
            let current = current as u32;
            accum += current as u16;
            accum *= 17;
            accum %= 256;
            accum
        })
    }

    fn hash_sum_calculator(line: &str) -> u32 {
        line.trim().split(",").fold(0, |mut accum, next| {
            accum += HashCalculator::calculate(next) as u32;
            accum
        })
    }
}
#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

        for each in input.split("\n") {
            assert_eq!(1320, HashCalculator::hash_sum_calculator(each));
        }
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day15.txt");

        for each in file_content.0.lines() {
            println!(
                "Answer1 for day15 is {}",
                HashCalculator::hash_sum_calculator(each)
            );
        }
    }

    #[test]
    fn test_small_with_local_data() {
        let input = r#"cm"#;

        for each in input.split("\n") {
            assert_eq!(0, HashCalculator::hash_sum_calculator(each));
        }
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

        let mut library = LensLibrary::new();
        for each in input.trim().split(",") {
            library.process_token(each);
        }

        assert_eq!(145, library.focus_power());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day15.txt");

        let mut library = LensLibrary::new();
        for each in file_content.0.trim().split(",") {
            library.process_token(each);
        }

        println!("Answer1 for day15 is {}", library.focus_power());
    }
}
