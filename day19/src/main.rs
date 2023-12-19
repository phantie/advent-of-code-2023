#![allow(unused)]

fn main() {
    let (workflows, parts) = parse::parse_input(input());

    dbg!(&workflows, &parts);

    let r = part_one(workflows, parts);
    dbg!(r);
}

// 333263
fn part_one(workflows: Workflows, parts: Parts) -> Num {
    fn part_accepted(part: Part, workflows: &Workflows) -> bool {
        let mut workflow = workflows.get("in").unwrap();

        loop {
            match workflow.process_part(part) {
                Next::Accept => return true,
                Next::Reject => return false,
                Next::Send(name) => workflow = workflows.get(&name).unwrap(),
            }
        }
    }

    parts
        .into_iter()
        .filter(|part| part_accepted(*part, &workflows))
        .map(|Part { x, m, a, s }| x + m + a + s)
        .sum::<Num>()
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

pub type Num = usize;

#[derive(Debug, Clone, Copy)]
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

impl Workflow {
    fn process_part(&self, part: Part) -> Next {
        self.ruleset.process_part(part)
    }
}

pub type Workflows = std::collections::HashMap<WorkflowName, Workflow>;

#[derive(Debug, Clone)]
pub enum Next {
    Accept,
    Reject,
    Send(WorkflowName),
}

#[derive(Debug, Clone, Copy)]
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

impl Rule {
    pub fn process_part(&self, part: Part) -> Option<Next> {
        match self {
            Self::Accept => Some(Next::Accept),
            Self::Reject => Some(Next::Reject),
            Self::Send(name) => Some(Next::Send(name.clone())),
            Self::Comparative {
                condition,
                next,
                rating_label,
            } => {
                let value = part.label(*rating_label);

                let applies = match *condition {
                    Condition::Greater(than) => value > than,
                    Condition::Less(than) => value < than,
                };

                if applies {
                    Some(next.clone())
                } else {
                    // the rule does not apply, I don't know what to do,
                    // move to the next
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Ruleset(Vec<Rule>);

impl std::ops::Deref for Ruleset {
    type Target = Vec<Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ruleset {
    pub fn process_part(&self, part: Part) -> Next {
        for rule in self.iter() {
            match rule.process_part(part) {
                None => continue,
                Some(next) => return next,
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Part {
    x: Num,
    m: Num,
    a: Num,
    s: Num,
}

impl Part {
    pub fn label(self, label: RatingLabel) -> Num {
        let Part { x, m, a, s } = self;
        use RatingLabel::*;
        match label {
            X => x,
            M => m,
            A => a,
            S => s,
        }
    }
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
