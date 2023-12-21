#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space0},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    Off,
    On,
}

impl State {
    fn other(&self) -> State {
        match self {
            State::Off => State::On,
            State::On => State::Off,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Machine(String);

#[derive(Debug, PartialEq, Eq)]
enum Module {
    Broadcast(Vec<Machine>),
    Conjunction(String, Vec<Machine>, RefCell<HashMap<Machine, Pulse>>),
    FlipFlop(String, State, Vec<Machine>),
}

impl Module {
    fn nomify_machine(input: &str) -> IResult<&str, Vec<Machine>> {
        let (_, machines) = separated_list1(tag(", "), alpha1)(input)?;
        let machines = machines
            .into_iter()
            .map(|it| Machine(String::from(it)))
            .collect();

        Ok(("", machines))
    }

    fn nomify_arrow_machine(input: &str) -> IResult<&str, Vec<Machine>> {
        let (_, (_, _, machines)) = tuple((space0, tag("-> "), Module::nomify_machine))(input)?;
        Ok(("", machines))
    }

    fn nomify_broadcast(input: &str) -> IResult<&str, Module> {
        let (_, machines) = tuple((tag("broadcaster"), Module::nomify_arrow_machine))(input)?;
        Ok(("", Module::Broadcast(machines.1)))
    }

    fn nomify_flip(input: &str) -> IResult<&str, Module> {
        let (_, (_, name, machines)) =
            tuple((tag("%"), alpha1, Module::nomify_arrow_machine))(input)?;
        Ok((
            "",
            Module::FlipFlop(String::from(name), State::Off, machines),
        ))
    }
    fn nomify_conjunction(input: &str) -> IResult<&str, Module> {
        let (_, (_, name, machines)) =
            tuple((tag("&"), alpha1, Module::nomify_arrow_machine))(input)?;
        Ok((
            "",
            Module::Conjunction(String::from(name), machines, RefCell::new(HashMap::new())),
        ))
    }

    fn nomify(input: &str) -> IResult<&str, Module> {
        let (_, module) = nom::branch::alt((
            Module::nomify_broadcast,
            Module::nomify_flip,
            Module::nomify_conjunction,
        ))(input)?;

        Ok(("", module))
    }
}

impl Module {
    fn pulse_action(&mut self, from: &Machine, pulse: &Pulse) -> Option<Pulse> {
        match self {
            Module::Broadcast(_) => panic!(),
            Module::Conjunction(_, _, providers) => {
                {
                    let mut current_state = providers.borrow_mut();
                    let current_state = current_state.get_mut(from).unwrap();
                    *current_state = pulse.clone();
                }

                match providers
                    .borrow()
                    .values()
                    .any(|value| *value == Pulse::Low)
                {
                    true => Some(Pulse::High),
                    false => Some(Pulse::Low),
                }
            }
            Module::FlipFlop(_, current, _) => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    let pulse = match current {
                        State::Off => Pulse::High,
                        State::On => Pulse::Low,
                    };
                    *current = current.other();
                    Some(pulse)
                }
            },
        }
    }

    fn get_neighbor(&self) -> &Vec<Machine> {
        match self {
            Module::Broadcast(_) => panic!(),
            Module::Conjunction(_, ref machines, _) => machines,
            Module::FlipFlop(_, _, ref machines) => machines,
        }
    }
}

impl FromStr for Module {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::nomify(s).map(|x| x.1).map_err(|_| "Parsing error")
    }
}

#[derive(Debug, Default)]
struct Pulser {
    broadcast_to: Vec<Machine>,
    modules: HashMap<String, Module>,
}

impl Pulser {
    fn accept_line(&mut self, line: &str) {
        let module = Module::from_str(line.trim()).unwrap();
        match module {
            Module::Broadcast(machines) => {
                self.broadcast_to = machines;
            }
            Module::Conjunction(name, machines, incoming_state) => {
                self.modules.insert(
                    name.clone(),
                    Module::Conjunction(name, machines, incoming_state),
                );
            }
            Module::FlipFlop(name, pulse, machines) => {
                self.modules
                    .insert(name.clone(), Module::FlipFlop(name, pulse, machines));
            }
        }
    }

    // only taking mut as this will make sure no one is modifying anything.
    fn init_incoming_for_conjunction(&mut self) {
        let empty_machines = vec![];
        for (name, module) in self.modules.iter() {
            let machines = match module {
                Module::Broadcast(_) => &empty_machines,
                Module::Conjunction(_, ref machines, _) => machines,
                Module::FlipFlop(_, _, ref machines) => machines,
            };

            for machine in machines {
                let Some(module)  = self.modules.get(&machine.0) else{
                    continue;
                };

                match module {
                    Module::Broadcast(_) | Module::FlipFlop(_, _, _) => {}
                    Module::Conjunction(_, _, incoming) => {
                        incoming
                            .borrow_mut()
                            .insert(Machine(String::from(name)), Pulse::Low);
                    }
                }
            }
        }
    }

    fn rum_pulses(&mut self, give_min_loop_count: bool) -> usize {
        let mut low_count = 0;
        let mut high_count = 0;
        let mut loop_count = 0;
        loop {
            if !give_min_loop_count && loop_count == 1000 {
                break;
            }

            //from - to - pulse
            let mut pulse_queue = VecDeque::new();
            let broadcast_machine = Machine(String::from("broadcaster"));
            for machine in self.broadcast_to.iter() {
                pulse_queue.push_back((broadcast_machine.clone(), machine.clone(), Pulse::Low));
            }
            low_count += 1; // initial button for broadcast

            while let Some(pulse) = pulse_queue.pop_front() {
                println!("{:?}", pulse);
                let (from, to, pulse) = pulse;
                if give_min_loop_count && to.0 == "rx" && pulse == Pulse::Low {
                    return loop_count + 1;
                }

                match pulse {
                    Pulse::High => high_count += 1,
                    Pulse::Low => low_count += 1,
                };

                self.modules.entry(to.0.clone()).and_modify(|to_module| {
                    if let Some(new_pluse) = to_module.pulse_action(&from, &pulse) {
                        for neighbor in to_module.get_neighbor() {
                            pulse_queue.push_back((
                                to.clone(),
                                neighbor.clone(),
                                new_pluse.clone(),
                            ));
                        }
                    }
                });
            }
            loop_count += 1;
        }

        high_count * low_count
    }
}

#[cfg(test)]
mod test {
    use crate::file_input_iterator::FileContent;

    use super::*;
    #[test]
    fn test_first_with_local_data() {
        let input = r#"broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a"#;

        let mut pulser = Pulser::default();
        for each in input.split("\n") {
            pulser.accept_line(each);
        }
        pulser.init_incoming_for_conjunction();

        assert_eq!(32000000, pulser.rum_pulses(false));
    }

    #[test]
    fn test_first_with_local_data_second() {
        let input = r#"broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"#;

        let mut pulser = Pulser::default();
        for each in input.split("\n") {
            pulser.accept_line(each);
        }
        pulser.init_incoming_for_conjunction();

        assert_eq!(11687500, pulser.rum_pulses(false));
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day20.txt");

        let mut pulser = Pulser::default();
        for each in file_content.0.lines() {
            pulser.accept_line(each);
        }
        pulser.init_incoming_for_conjunction();

        println!("Answer1 for day20 is {}", pulser.rum_pulses(false));
    }

    #[test]
    fn test_second_with_file() {
        let file_content = FileContent::new("day20.txt");

        let mut pulser = Pulser::default();
        for each in file_content.0.lines() {
            pulser.accept_line(each);
        }
        pulser.init_incoming_for_conjunction();

        println!("Answer1 for day20 is {}", pulser.rum_pulses(true));
    }
}
