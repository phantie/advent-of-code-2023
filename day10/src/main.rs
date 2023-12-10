#![allow(unused)]

type SpaceRow = Vec<FieldNode>;
type Space = Vec<SpaceRow>;

type I = usize;
type J = usize;

type Pos = (I, J);

fn main() {
    let part_one = part_one::part_one();
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
pub enum FieldNode {
    Node(Node),
    Start,
    Ground,
}

impl FieldNode {
    pub fn leads_to(&self, direction: Direction) -> bool {
        match self {
            FieldNode::Start => false,
            FieldNode::Ground => false,
            FieldNode::Node(node) => node.leads_to(direction),
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
                    '.' => FieldNode::Ground,
                    'S' => FieldNode::Start,
                    '|' => FieldNode::Node(Node(South, North)),
                    '-' => FieldNode::Node(Node(West, East)),
                    'L' => FieldNode::Node(Node(North, East)),
                    'J' => FieldNode::Node(Node(North, West)),
                    '7' => FieldNode::Node(Node(South, West)),
                    'F' => FieldNode::Node(Node(South, East)),
                    _ => unreachable!(),
                })
                .collect::<SpaceRow>()
        })
        .collect::<Space>()
}

fn pad_space(space: Space) -> Space {
    let row_len = space[0].len();

    let empty_row = (0..row_len + 2)
        .map(|_| FieldNode::Ground)
        .collect::<Vec<_>>();

    std::iter::once(empty_row.clone())
        .chain(space.into_iter().map(|line| {
            std::iter::once(FieldNode::Ground)
                .chain(line.into_iter())
                .chain(std::iter::once(FieldNode::Ground))
                .collect()
        }))
        .chain(std::iter::once(empty_row.clone()))
        .collect()
}

type RelativeDirection = Direction;

fn get_adjacent_indeces(pos: Pos) -> Vec<(Pos, RelativeDirection)> {
    vec![
        (Direction::East.opposite().from_pos(pos), Direction::East),
        (Direction::West.opposite().from_pos(pos), Direction::West),
        (Direction::South.opposite().from_pos(pos), Direction::South),
        (Direction::North.opposite().from_pos(pos), Direction::North),
    ]
}

fn get_field_node(space: &Space, (i, j): Pos) -> FieldNode {
    space[i][j]
}

mod part_one {
    use super::*;
    use strum::IntoEnumIterator;

    pub fn part_one() -> u32 {
        let space = pad_space(parse(input()));

        let flat_index = space
            .iter()
            .flatten()
            .enumerate()
            .find_map(|(i, node)| if node.is_start() { Some(i) } else { None })
            .unwrap();

        let row_len = space[0].len();

        let (div, rem) = num::integer::div_rem(flat_index, row_len);

        let pos @ (i, j) = (div, rem);

        let start = &space[i][j];

        assert!(start.is_start());

        let (node, mut direction, mut pos) = Direction::iter()
            .find_map(|direction| {
                let cell = get_field_node(&space, direction.from_pos(pos));
                if cell.leads_to(direction.opposite()) {
                    Some((cell.unwrap_node(), direction, pos))
                } else {
                    None
                }
            })
            .unwrap();

        let mut counter = 0;
        loop {
            let node = get_field_node(&space, direction.from_pos(pos));
            counter += 1;

            if node.is_start() {
                return counter / 2;
            }

            pos = direction.from_pos(pos);
            direction = node
                .unwrap_node()
                .opposite_direction(direction.opposite())
                .unwrap();
        }
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 6942);
    }
}
