#![allow(dead_code)]

use std::collections::HashMap;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, digit1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl Destination {
    fn new(destination: &str) -> Destination {
        use Destination::*;
        match destination {
            "A" => Accept,
            "R" => Reject,
            _ => Workflow(String::from(destination)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct ConditionalRule {
    what: String,
    operator: String,
    compared_with: u32,
    destination: Destination,
}

impl ConditionalRule {
    fn parse(line: &str) -> IResult<&str, ConditionalRule> {
        let (_, (first, last)) = separated_pair(is_not(":"), tag(":"), alpha1)(line)?;
        let (_, (what, operator, comparated_with)) =
            tuple((alpha1, nom::branch::alt((tag(">"), tag("<"))), digit1))(first)?;

        Ok((
            "",
            ConditionalRule {
                what: String::from(what),
                operator: String::from(operator),
                compared_with: comparated_with.parse().unwrap(),
                destination: Destination::new(last),
            },
        ))
    }

    fn get_destination_for_part(&self, part: &Part) -> Option<Destination> {
        let part_value = part.get_section(&self.what);
        let matched = match self.operator.as_str() {
            ">" => part_value > self.compared_with,
            "<" => part_value < self.compared_with,
            _ => panic!("Unknown condition met {}", self.operator),
        };

        if matched {
            Some(self.destination.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct UnconditionalRule {
    destination: Destination,
}

impl UnconditionalRule {
    fn parse(line: &str) -> IResult<&str, UnconditionalRule> {
        let destination = Destination::new(line);

        Ok(("", UnconditionalRule { destination }))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Rule {
    Conditional(ConditionalRule),
    Unconditional(UnconditionalRule),
}

impl Rule {
    fn nomify(rule: &str) -> IResult<&str, Rule> {
        if let Ok(conditional) = ConditionalRule::parse(rule) {
            return Ok((conditional.0, Rule::Conditional(conditional.1)));
        }

        let (_, unconditional) = UnconditionalRule::parse(rule)?;

        Ok(("", Rule::Unconditional(unconditional)))
    }

    fn new(rule: &str) -> Rule {
        Rule::nomify(rule).unwrap().1
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn nomify(line: &str) -> IResult<&str, Workflow> {
        let (_, (name, rule)) = tuple((
            is_not("{"),
            nom::sequence::delimited(tag("{"), is_not("}"), tag("}")),
        ))(line)?;
        let (_, rules) = separated_list1(tag(","), is_not(","))(rule)?;

        let rules = rules.into_iter().map(Rule::new).collect();

        Ok((
            "",
            Workflow {
                name: String::from(name),
                rules,
            },
        ))
    }

    fn new(line: &str) -> Workflow {
        Workflow::nomify(line.trim()).unwrap().1
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse_part(line: &str) -> IResult<&str, Part> {
        let (left, (_, x_val, _, m_val, _, a_val, _, s_val, _)) = tuple((
            tag("{x="),
            digit1,
            tag(",m="),
            digit1,
            tag(",a="),
            digit1,
            tag(",s="),
            digit1,
            tag("}"),
        ))(line)?;
        Ok((
            left,
            (Part {
                x: x_val.parse::<u32>().unwrap(),
                m: m_val.parse().unwrap(),
                a: a_val.parse().unwrap(),
                s: s_val.parse().unwrap(),
            }),
        ))
    }

    fn new(line: &str) -> Part {
        Part::parse_part(line.trim()).unwrap().1
    }

    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn get_section(&self, what: &str) -> u32 {
        match what {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Unkwon part section {what}"),
        }
    }
}

#[derive(Debug, Default)]
struct Aplenty {
    parts: Vec<Part>,
    workflows: HashMap<String, Workflow>,
}

impl Aplenty {
    fn add_parts(&mut self, line: &str) {
        let part = Part::new(line);
        self.parts.push(part);
    }

    fn add_workflow(&mut self, line: &str) {
        let workflow = Workflow::new(line);
        self.workflows.insert(workflow.name.clone(), workflow);
    }

    fn is_accepted(&self, workflow: &Workflow, part: &Part) -> bool {
        for section in workflow.rules.iter() {
            let destination = match section {
                Rule::Conditional(ref conditional) => {
                    match conditional.get_destination_for_part(part) {
                        Some(other) => other,
                        None => Destination::Workflow(workflow.name.clone()),
                    }
                }
                Rule::Unconditional(ref unconditional) => unconditional.destination.clone(),
            };

            match &destination {
                Destination::Accept => return true,
                Destination::Reject => return false,
                Destination::Workflow(to) => {
                    if *to == workflow.name {
                        continue;
                    }

                    return self.is_accepted(self.workflows.get(to).unwrap(), part);
                }
            };
        }

        return false;
    }

    fn sum_accepted_rating_number(&self) -> u32 {
        let mut sum = 0;
        let in_work_flow = self.workflows.get("in").unwrap();
        for part in self.parts.iter() {
            if self.is_accepted(in_work_flow, part) {
                sum += part.rating();
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
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"#;

        let mut is_workflow = true;
        let mut aplenty = Aplenty::default();
        for each in input.split("\n") {
            if each.trim().is_empty() {
                is_workflow = false;
                continue;
            }

            if is_workflow {
                aplenty.add_workflow(each.trim());
            } else {
                aplenty.add_parts(each.trim());
            }
        }

        assert_eq!(19114, aplenty.sum_accepted_rating_number());
    }

    #[test]
    fn test_first_with_file() {
        let file_content = FileContent::new("day19.txt");

        let mut is_workflow = true;
        let mut aplenty = Aplenty::default();
        for each in file_content.0.lines() {
            if each.trim().is_empty() {
                is_workflow = false;
                continue;
            }

            if is_workflow {
                aplenty.add_workflow(each.trim());
            } else {
                aplenty.add_parts(each.trim());
            }
        }

        println!(
            "Answer1 for day19 is {}",
            aplenty.sum_accepted_rating_number()
        );
    }
}
