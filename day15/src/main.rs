#![allow(unused)]

mod part_one {
    use super::*;

    pub fn part_one() -> u32 {
        parse_input(input()).map(hash).sum::<u32>()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 504036);
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> usize {
        focusing_power_of_lense_configuration(parse_steps(input()).collect())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 295719);
    }
}

fn focusing_power_of_lense_configuration(steps: Vec<Step>) -> usize {
    steps
        .into_iter()
        .fold(
            (0..256).map(|_| vec![]).collect::<Vec<_>>(),
            |mut boxes: Vec<Vec<Lense>>, step| {
                match step.operation {
                    Operation::Equals { focal_len } => {
                        match boxes[step.hash]
                            .iter()
                            .position(|_step| _step.label == step.label)
                        {
                            None => boxes[step.hash].push(Lense {
                                label: step.label,
                                focal_len,
                            }),
                            Some(i) => boxes[step.hash][i].focal_len = focal_len,
                        }
                    }
                    Operation::Dash => {
                        match boxes[step.hash]
                            .iter()
                            .position(|_step| _step.label == step.label)
                        {
                            None => {}
                            Some(i) => {
                                boxes[step.hash].remove(i);
                            }
                        }
                    }
                }

                boxes
            },
        )
        .into_iter()
        .enumerate()
        .map(|(box_i, a_box)| {
            a_box
                .into_iter()
                .enumerate()
                .map(move |(lense_i, lense)| (box_i + 1) * (lense_i + 1) * lense.focal_len)
        })
        .flatten()
        .sum::<usize>()
}

type FocalLen = usize;

#[derive(Debug, Clone)]
struct Step {
    pub label: String,
    pub operation: Operation,
    pub hash: usize,
}

impl Step {
    pub fn new(label: String, operation: Operation) -> Self {
        let hash = hash(&label) as usize;

        Self {
            label,
            operation,
            hash,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Dash,
    Equals { focal_len: FocalLen },
}

#[derive(Clone, Debug)]
struct Lense {
    pub label: String,
    pub focal_len: FocalLen,
}

fn hash(v: &str) -> u32 {
    v.chars().map(char_to_ascii_code).fold(0, |acc, x| {
        let (_div, rem) = num::integer::div_rem((acc + x) * 17, 256);
        rem
    })
}

fn char_to_ascii_code(c: char) -> u32 {
    c as u32
}

fn parse_input(i: &str) -> impl Iterator<Item = &str> {
    i.split(",")
}

fn parse_steps(i: &str) -> impl Iterator<Item = Step> + '_ {
    i.split(",").map(|v| v.parse().unwrap())
}

impl std::str::FromStr for Step {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, operation) = if s.contains('-') {
            let label = &s[0..s.len() - 1];
            let operation = Operation::Dash;
            (label, operation)
        } else {
            let (label, lense_focal_length) = s.split_once('=').unwrap();
            let operation = Operation::Equals {
                focal_len: lense_focal_length.parse().unwrap(),
            };
            (label, operation)
        };

        Ok(Self::new(label.into(), operation))
    }
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_example() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_sequence_example() {
        let r = parse_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
            .map(hash)
            .sum::<u32>();
        assert_eq!(r, 1320);
    }

    #[test]
    fn test_focusing_power_of_lense_configuration() {
        let steps =
            parse_steps("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").collect::<Vec<_>>();

        assert_eq!(focusing_power_of_lense_configuration(steps), 145);
    }
}
