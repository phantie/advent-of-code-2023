#![allow(unused)]

fn main() {
    let (workflows, parts) = parse::parse_input(input());

    dbg!(workflows, parts);
}

fn part_one(workflows: Workflows, parts: Parts) {
    fn calc_part(part: Part, workflows: &Workflows) {}
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

pub type Num = usize;

#[derive(Debug)]
pub enum RatingLabel {
    X,
    M,
    A,
    S,
}

pub type WorkflowName = String;

#[derive(Debug)]
pub struct Workflow {
    name: WorkflowName,
    ruleset: Ruleset,
}

pub type Workflows = std::collections::HashMap<WorkflowName, Workflow>;

#[derive(Debug)]
pub enum Next {
    Accept,
    Reject,
    Send(WorkflowName),
}

#[derive(Debug)]
pub enum Condition {
    Greater(Num),
    Less(Num),
}

#[derive(Debug)]
pub enum Rule {
    Comparative {
        condition: Condition,
        next: Next,
        rating_label: RatingLabel,
    },
    Accept,
    Reject,
    Send(WorkflowName),
}

#[derive(Debug)]
pub struct Ruleset(Vec<Rule>);

#[derive(Debug)]
pub struct Part {
    x: Num,
    m: Num,
    a: Num,
    s: Num,
}

pub type Parts = Vec<Part>;
mod parse {
    // FUUUUUUUUUUUUCK parsing

    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self},
        multi::separated_list1,
        IResult,
    };

    fn parse_rules(input: &str) -> Vec<Rule> {
        let rules = input.split(",").collect::<Vec<_>>();

        let rules = rules.into_iter().map(|rule| {
            let (_, r) = parse_rule(rule).unwrap();
            r
        });

        let rules = rules.collect();

        rules
    }

    fn parse_rule(input: &str) -> IResult<&str, Rule> {
        if input.contains(":") {
            let (input, label) = alt((tag("x"), tag("m"), tag("a"), tag("s")))(input)?;

            let rating_label = match label {
                "x" => RatingLabel::X,
                "m" => RatingLabel::M,
                "a" => RatingLabel::A,
                "s" => RatingLabel::S,
                _ => unreachable!(),
            };

            let (input, sign) = alt((tag(">"), tag("<")))(input)?;
            let (input, num) = complete::u32(input)?;
            let condition = match sign {
                "<" => Condition::Less(num as _),
                ">" => Condition::Greater(num as _),
                _ => unreachable!(),
            };

            let (input, _) = tag(":")(input)?;
            let next = match input {
                "A" => Next::Accept,
                "R" => Next::Reject,
                workflow_name => Next::Send(workflow_name.into()),
            };
            let rule = Rule::Comparative {
                condition,
                next,
                rating_label,
            };

            Ok((input, rule))
        } else {
            let next = match input {
                "A" => Rule::Accept,
                "R" => Rule::Reject,
                workflow_name => Rule::Send(workflow_name.into()),
            };

            Ok((input, next))
        }
    }

    fn parse_workflow(input: &str) -> Workflow {
        let (name, rules) = input.split_once("{").unwrap();

        let rules = parse_rules(&rules[..rules.len() - 1]);

        Workflow {
            name: name.into(),
            ruleset: Ruleset(rules),
        }
    }

    fn parse_workflows(input: &str) -> Workflows {
        input
            .split("\n")
            .map(parse_workflow)
            .into_iter()
            .map(|w| (w.name.clone(), w))
            .collect()
    }

    fn parse_part(input: &str) -> IResult<&str, Part> {
        let (input, _) = tag("{x=")(input)?;
        let (input, x) = complete::u32(input)?;
        let (input, _) = tag(",m=")(input)?;
        let (input, m) = complete::u32(input)?;
        let (input, _) = tag(",a=")(input)?;
        let (input, a) = complete::u32(input)?;
        let (input, _) = tag(",s=")(input)?;
        let (input, s) = complete::u32(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((
            input,
            Part {
                x: x as _,
                m: m as _,
                a: a as _,
                s: s as _,
            },
        ))
    }

    fn parse_parts(input: &str) -> Parts {
        let (_, parts) = separated_list1(tag("\n"), parse_part)(input).unwrap();
        parts
    }

    pub fn parse_input(input: &str) -> (Workflows, Parts) {
        let (workflows, parts) = input.split_once("\n\n").unwrap();

        let workflows = parse_workflows(workflows);
        let parts = parse_parts(parts);

        (workflows, parts)
    }
}
