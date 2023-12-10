#![allow(unused)]

type SpaceRow = Vec<Cell>;
type Space = Vec<SpaceRow>;

type I = usize;
type J = usize;

type Pos = (I, J);

fn main() {
    let part_one = part_one::part_one();
    let part_two = part_two::part_two();
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn from_pos(self, (i, j): Pos) -> Pos {
        match self {
            Self::North => (i - 1, j),
            Self::South => (i + 1, j),
            Self::East => (i, j + 1),
            Self::West => (i, j - 1),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node(pub Direction, pub Direction);

impl Node {
    pub fn leads_to(&self, direction: Direction) -> bool {
        self.0 == direction || self.1 == direction
    }

    pub fn opposite_direction(&self, direction: Direction) -> Option<Direction> {
        if self.0 == direction {
            Some(self.1)
        } else if self.1 == direction {
            Some(self.0)
        } else {
            None
        }
    }
}

#[derive(Debug, strum::EnumIs, Clone, Copy)]
pub enum Cell {
    Node(Node),
    Start,
    Ground,
}

impl Cell {
    pub fn leads_to(&self, direction: Direction) -> bool {
        match self {
            Cell::Start => false,
            Cell::Ground => false,
            Cell::Node(node) => node.leads_to(direction),
        }
    }

    pub fn unwrap_node(self) -> Node {
        match self {
            Self::Ground | Self::Start => unreachable!(),
            Self::Node(node) => node,
        }
    }
}

fn parse(input: &str) -> Space {
    use Direction::*;
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Cell::Ground,
                    'S' => Cell::Start,
                    '|' => Cell::Node(Node(South, North)),
                    '-' => Cell::Node(Node(West, East)),
                    'L' => Cell::Node(Node(North, East)),
                    'J' => Cell::Node(Node(North, West)),
                    '7' => Cell::Node(Node(South, West)),
                    'F' => Cell::Node(Node(South, East)),
                    _ => unreachable!(),
                })
                .collect::<SpaceRow>()
        })
        .collect::<Space>()
}

fn pad_space(space: Space) -> Space {
    let row_len = space[0].len();

    let empty_row = (0..row_len + 2).map(|_| Cell::Ground).collect::<Vec<_>>();

    std::iter::once(empty_row.clone())
        .chain(space.into_iter().map(|line| {
            std::iter::once(Cell::Ground)
                .chain(line.into_iter())
                .chain(std::iter::once(Cell::Ground))
                .collect()
        }))
        .chain(std::iter::once(empty_row.clone()))
        .collect()
}

type RelativeDirection = Direction;

fn get_cell(space: &Space, (i, j): Pos) -> Cell {
    space[i][j]
}

mod part_one {
    use super::*;
    use strum::IntoEnumIterator;

    pub fn part_one() -> usize {
        let space = pad_space(parse(input()));

        let start_pos = {
            let flat_index = space
                .iter()
                .flatten()
                .enumerate()
                .find_map(|(i, cell)| if cell.is_start() { Some(i) } else { None })
                .unwrap();

            let row_len = space[0].len();

            num::integer::div_rem(flat_index, row_len)
        };

        let start = get_cell(&space, start_pos);

        assert!(start.is_start());

        let (node, direction, pos) = Direction::iter()
            .find_map(|direction| {
                let cell = get_cell(&space, direction.from_pos(start_pos));
                if cell.leads_to(direction.opposite()) {
                    Some((cell.unwrap_node(), direction, start_pos))
                } else {
                    None
                }
            })
            .unwrap();

        std::iter::repeat(())
            .enumerate()
            .try_fold((pos, direction), |(pos, direction), (i, ())| {
                let cell = get_cell(&space, direction.from_pos(pos));

                if cell.is_start() {
                    Err(i / 2 + 1)
                } else {
                    Ok((
                        direction.from_pos(pos),
                        cell.unwrap_node()
                            .opposite_direction(direction.opposite())
                            .unwrap(),
                    ))
                }
            })
            .unwrap_err()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 6942);
    }
}

mod part_two {
    use super::*;
    use strum::IntoEnumIterator;

    pub fn part_two() -> usize {
        let space = pad_space(parse(input()));

        let start_pos = {
            let flat_index = space
                .iter()
                .flatten()
                .enumerate()
                .find_map(|(i, cell)| if cell.is_start() { Some(i) } else { None })
                .unwrap();

            let row_len = space[0].len();

            num::integer::div_rem(flat_index, row_len)
        };

        let start = get_cell(&space, start_pos);

        assert!(start.is_start());

        let (_node, direction, _pos) = Direction::iter()
            .find_map(|direction| {
                let cell = get_cell(&space, direction.from_pos(start_pos));
                if cell.leads_to(direction.opposite()) {
                    Some((cell.unwrap_node(), direction, start_pos))
                } else {
                    None
                }
            })
            .unwrap();

        let mut ring = vec![(start, start_pos, direction)];

        loop {
            let (node, pos, direction) = ring.last().unwrap().clone();

            let cell = get_cell(&space, direction.from_pos(pos));

            if cell.is_start() {
                break;
            } else {
                ring.push((
                    cell,
                    direction.from_pos(pos),
                    cell.unwrap_node()
                        .opposite_direction(direction.opposite())
                        .unwrap(),
                ));
            }
        }

        {
            assert_eq!(ring.len(), 13884);

            let (start, _, _) = ring[0];
            assert!(start.is_start());

            let (end, _, _) = ring.last().unwrap();
            assert!(!end.is_start());
        }

        let total_area = ring
            .clone()
            .into_iter()
            .chain(std::iter::once(ring[0]))
            .map(|(_, pos, _)| pos)
            .collect::<Vec<_>>()
            .as_slice()
            .windows(2)
            // https://en.wikipedia.org/wiki/Shoelace_formula, see Example
            .map(|window| {
                let ((x_l, y_l), (x_r, y_r)) = (window[0], window[1]);

                // adjust y coord
                let y_l = space.len() - y_l;
                let y_r = space.len() - y_r;

                (x_l * y_r) as i64 - (x_r * y_l) as i64
            })
            .sum::<i64>() as usize
            / 2;

        // derived from https://en.wikipedia.org/wiki/Pick%27s_theorem
        // A = i + b / 2 - 1
        // i = A - b / 2 + 1
        total_area - ring.len() / 2 + 1
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 297);
    }
}
