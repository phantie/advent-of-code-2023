fn main() {
    let steps = read_input()
        .map(Result::unwrap)
        .map(|l| parse::process_line(&l))
        .collect::<Vec<_>>();

    let ring = walk_steps(steps);
    // let ring = walk_steps2(steps);
    dbg!(ring.len());
    dbg!(ring.first());
    dbg!(ring.last());
    // dbg!(&ring);

    // let (_, max_y) = ring.clone().into_iter().max_by_key(|(x, y)| *y).unwrap();
    // dbg!(max_y);

    let total_area = ring
        .clone()
        .into_iter()
        .chain(std::iter::once(ring[0]))
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        // https://en.wikipedia.org/wiki/Shoelace_formula, see Example
        .map(|window| {
            let ((x_l, y_l), (x_r, y_r)) = (window[0], window[1]);

            (x_l * y_r) as i64 - (x_r * y_l) as i64
        })
        .sum::<i64>() as usize
        / 2;

    // derived from https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b / 2 - 1
    // i = A - b / 2 + 1
    let inner = total_area - ring.len() / 2 + 1;
    dbg!(inner);
    // let r = total_area;
    dbg!(ring.len());

    dbg!(ring.len() + inner);
}

mod part_one {
    use super::*;

    fn part_one() -> usize {
        let steps = read_input()
            .map(Result::unwrap)
            .map(|l| parse::process_line(&l))
            .collect::<Vec<_>>();

        let ring = walk_steps(steps);

        let total_area = ring
            .clone()
            .into_iter()
            .chain(std::iter::once(ring[0]))
            .collect::<Vec<_>>()
            .as_slice()
            .windows(2)
            // https://en.wikipedia.org/wiki/Shoelace_formula, see Example
            .map(|window| {
                let ((x_l, y_l), (x_r, y_r)) = (window[0], window[1]);
                (x_l * y_r) as i64 - (x_r * y_l) as i64
            })
            .sum::<i64>() as usize
            / 2;

        // derived from https://en.wikipedia.org/wiki/Pick%27s_theorem
        // A = i + b / 2 - 1
        // i = A - b / 2 + 1
        let inner = total_area - ring.len() / 2 + 1;

        // I have no idea why this time I have to do it differently
        // formula derived from finding pattern in smaller input
        ring.len() + inner
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 62365);
    }
}

type X = isize;
type Y = isize;
type Pos = (X, Y);

fn walk_steps(steps: Steps) -> Vec<Pos> {
    // let mut pos = (-1, 0);

    let mut contour = vec![(0, 0)];

    for step in steps {
        for _ in 0..step.n {
            contour.push(step.direction.move_pos(*contour.last().unwrap(), 1));
        }
    }

    // contour.remove(0);
    contour.remove(contour.len() - 1);

    // dbg!(contour);

    contour
}

fn walk_steps2(steps: Steps) -> Vec<Pos> {
    let mut contour = vec![(0, 0)];

    for step in steps {
        contour.push(step.direction.move_pos(*contour.last().unwrap(), step.n));
    }

    contour.remove(contour.len() - 1);

    contour
}

type Steps = Vec<Step>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        let direction = match input.chars().into_iter().next().unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unreachable!(),
        };

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
