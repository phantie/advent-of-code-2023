#![allow(unused)]

mod part_one {
    use super::*;

    fn part_one() -> usize {
        let steps = read_input()
            .map(Result::unwrap)
            .map(|l| parse::process_line(&l))
            .collect::<Vec<_>>();

        let (perimeter, ring) = walk_steps(steps);

        let total_area = total_area(&ring);

        total_area + perimeter / 2 + 1
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 62365);
    }
}

fn main() {}

mod part_two {
    use super::*;

    fn part_two() -> usize {
        let steps = read_input()
            .map(Result::unwrap)
            .map(|l| parse::process_line(&l).fix())
            .collect::<Vec<_>>();
        let (perimeter, ring) = walk_steps(steps);
        let total_area = total_area(&ring);
        total_area + perimeter / 2 + 1
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 159485361249806);
    }
}

fn total_area(ring: &Vec<Pos>) -> usize {
    ring.clone()
        .into_iter()
        .chain(std::iter::once(ring[0]))
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        // https://en.wikipedia.org/wiki/Shoelace_formula, see Example
        .map(|window| {
            let ((x_l, y_l), (x_r, y_r)) = (window[0], window[1]);

            (x_l * y_r) - (x_r * y_l)
        })
        .sum::<isize>() as usize
        / 2
}

type X = isize;
type Y = isize;
type Pos = (X, Y);

type Steps = Vec<Step>;
type Perimeter = usize;

fn walk_steps(steps: Steps) -> (Perimeter, Vec<Pos>) {
    let mut contour = vec![(0, 0)];
    let mut perimeter = 0;

    for step in steps {
        perimeter += step.n as usize;
        contour.push(step.direction.move_pos(*contour.last().unwrap(), step.n));
    }

    contour.remove(contour.len() - 1);

    (perimeter, contour)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' | '0' => Direction::Right,
            'L' | '2' => Direction::Left,
            'U' | '3' => Direction::Up,
            'D' | '1' => Direction::Down,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    pub fn move_pos(&self, (x, y): Pos, c: u32) -> Pos {
        let c = c as isize;
        match self {
            Direction::Up => (x, y - c),
            Direction::Down => (x, y + c),
            Direction::Left => (x - c, y),
            Direction::Right => (x + c, y),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Step {
    direction: Direction,
    n: u32,
    color: String,
}

#[allow(unused)]
fn display_ring(ring: &Vec<Pos>) {
    let existing = ring.iter().collect::<std::collections::HashSet<_>>();

    let (_, max_y) = ring.clone().into_iter().max_by_key(|(x, y)| *y).unwrap();
    let (max_x, _) = ring.clone().into_iter().max_by_key(|(x, y)| *x).unwrap();

    for j in (0..max_y + 1) {
        for i in (0..max_x + 1) {
            if existing.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

impl Step {
    pub fn fix(mut self) -> Self {
        self.n = u32::from_str_radix(&self.color[0..5], 16).unwrap();
        self.direction = self.color.chars().last().unwrap().into();
        self
    }
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

mod parse {
    use super::{Direction, Step};
    use nom::{
        bytes::complete::{tag, take_until1},
        character::complete::{self, space1},
        IResult,
    };

    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        let direction = input.chars().into_iter().next().unwrap().into();

        let input = &input[1..];

        Ok((input, direction))
    }

    fn parse_line(input: &str) -> IResult<&str, Step> {
        let (input, direction) = parse_direction(input)?;
        let (input, _) = space1(input)?;
        let (input, n) = complete::u32(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("(#")(input)?;
        let (input, color) = take_until1(")")(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((
            input,
            Step {
                direction,
                n,
                color: color.into(),
            },
        ))
    }

    pub fn process_line(input: &str) -> Step {
        let (_, result) = parse_line(input).unwrap();
        result
    }
}
