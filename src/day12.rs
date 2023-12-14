#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl State {
    fn new(state: char) -> State {
        use State::*;
        match state {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            unknown @ _ => panic!("Unknown passed {:?}", unknown),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd)]
struct StateDamange(Vec<State>, Vec<usize>);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd)]
struct HotSpring {
    springs: Vec<StateDamange>,
}

impl HotSpring {
    fn accept_line(&mut self, line: &str) {
        let mut splitted = line.trim().split_ascii_whitespace();
        let state = splitted
            .next()
            .unwrap()
            .chars()
            .map(|ch| State::new(ch))
            .collect();
        let damage = splitted
            .next()
            .unwrap()
            .split(",")
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect();
        self.springs.push(StateDamange(state, damage));
    }

    fn expand(self) -> HotSpring {
        let mut springs = Vec::new();

        for spring in self.springs {
            let old_state = spring.0;
            let old_damage = spring.1;
            let mut state: Vec<State> = Vec::with_capacity(old_state.len() * 5 + 5);
            let mut damage = Vec::with_capacity(old_damage.len() * 5);

            for i in 0..5 {
                state.append(&mut old_state.clone());
                if i != 4 {
                    state.push(State::Unknown);
                }

                damage.append(&mut old_damage.clone());
            }

            springs.push(StateDamange(state, damage));
        }

        HotSpring { springs }
    }

    fn count_possible_configuration(
        state_damage: &StateDamange,
        state_pos: usize,
        damage_pos: usize,
        memoize: &mut HashMap<(usize, usize), Option<usize>>,
    ) -> Option<usize> {
        if let Some(ans) = memoize.get(&(state_pos, damage_pos)) {
            return *ans;
        }

        let StateDamange(state, damage) = state_damage;
        let is_damage_count_possible = |mut count: usize, starting_at: usize| {
            for st in state[starting_at..].iter() {
                if count == 0 {
                    if *st == State::Damaged {
                        return false;
                    }

                    return true;
                }
                if *st == State::Operational {
                    return false;
                }
                count -= 1;
            }
            count == 0
        };

        let Some(&current_damage_count) = damage.get(damage_pos) else {
            if state_pos < state.len() && state[state_pos..].iter().any(|st| *st == State::Damaged) {
                return None;
            }
            return Some(1);
        };

        if state_pos >= state.len() {
            return None;
        }

        let mut sum = 0;
        if is_damage_count_possible(current_damage_count, state_pos) {
            match HotSpring::count_possible_configuration(
                state_damage,
                state_pos + current_damage_count + 1,
                damage_pos + 1,
                memoize,
            ) {
                Some(count) => sum += count,
                None => {}
            };
        }

        if state[state_pos] != State::Damaged {
            match HotSpring::count_possible_configuration(
                state_damage,
                state_pos + 1,
                damage_pos,
                memoize,
            ) {
                Some(count) => sum += count,
                None => {}
            };
        }

        let ans = if sum == 0 { None } else { Some(sum) };
        memoize.insert((state_pos, damage_pos), ans.clone());
        ans
    }

    fn sum_possible_configuration(&self) -> usize {
        let mut sum = 0;
        for state_damage in self.springs.iter() {
            let mut memoize = HashMap::<(usize, usize), Option<usize>>::new();
            if let Some(count) =
                HotSpring::count_possible_configuration(state_damage, 0, 0, &mut memoize)
            {
                sum += count;
            } else {
                panic!("Got nothing");
            }
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"#;

        let mut hot_spring = HotSpring::default();
        for each in input.split("\n") {
            hot_spring.accept_line(each);
        }

        assert_eq!(21, hot_spring.sum_possible_configuration());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day12.txt");

        let mut hot_spring = HotSpring::default();
        for each in file_content.0.lines() {
            hot_spring.accept_line(each);
        }

        println!(
            "Answer1 for day12 is {}",
            hot_spring.sum_possible_configuration()
        );
    }

    #[test]
    fn test_second_with_local_data() {
        let input = r#"???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"#;

        let mut hot_spring = HotSpring::default();
        for each in input.split("\n") {
            hot_spring.accept_line(each);
        }
        hot_spring = hot_spring.expand();

        assert_eq!(525152, hot_spring.sum_possible_configuration());
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day12.txt");

        let mut hot_spring = HotSpring::default();
        for each in file_content.0.lines() {
            hot_spring.accept_line(each);
        }
        hot_spring = hot_spring.expand();

        println!(
            "Answer2 for day12 is {}",
            hot_spring.sum_possible_configuration()
        );
    }
}
