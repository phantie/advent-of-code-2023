mod part_one {
    use super::*;

    pub fn part_one() -> usize {
        let space = input().parse::<Space>().unwrap();
        let ring = space.ring();
        ring.len() / 2
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 6942);
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> usize {
        let space = input().parse::<Space>().unwrap();
        let ring = space.ring();

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

struct Space(pub Vec<SpaceRow>);

impl Space {
    pub fn ring(&self) -> Vec<Pos> {
        let space = self;
        let start_pos = space.find_start_pos();
        let start_direction = space.start_direction(start_pos);

        let ring = std::iter::repeat(())
            .try_fold(vec![(start_pos, start_direction)], |mut ring, ()| {
                let (pos, direction) = ring.last().unwrap().clone();
                let cell = space.get_cell(direction.from_pos(pos));

                if cell.is_start() {
                    Err(ring)
                } else {
                    ring.push((
                        direction.from_pos(pos),
                        cell.unwrap_node()
                            .opposite_direction(direction.opposite())
                            .unwrap(),
                    ));

                    Ok(ring)
                }
            })
            .unwrap_err()
            .into_iter()
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();

        {
            assert_eq!(ring.len(), 13884);

            let start_pos = ring[0];
            let start = space.get_cell(start_pos);
            assert!(start.is_start());

            let end_pos = *ring.last().unwrap();
            let end = space.get_cell(end_pos);
            assert!(!end.is_start());
        }

        ring
    }

    pub fn get_cell(&self, (i, j): Pos) -> Cell {
        self[i][j]
    }

    pub fn x_dim(&self) -> usize {
        self[0].len()
    }

    pub fn find_start_pos(&self) -> Pos {
        let flat_index = self
            .iter()
            .flatten()
            .enumerate()
            .find_map(|(i, cell)| if cell.is_start() { Some(i) } else { None })
            .unwrap();

        num::integer::div_rem(flat_index, self.x_dim())
    }

    pub fn start_direction(&self, start_pos: Pos) -> Direction {
        use strum::IntoEnumIterator;
        Direction::iter()
            .find_map(|direction| {
                let cell = self.get_cell(direction.from_pos(start_pos));
                if cell.leads_to(direction.opposite()) {
                    Some(direction)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl std::ops::Deref for Space {
    type Target = Vec<SpaceRow>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type SpaceRow = Vec<Cell>;

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

type I = usize;
type J = usize;
type Pos = (I, J);

impl std::str::FromStr for Space {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(Space(
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
                .collect(),
        ))
    }
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

fn main() {
    let part_one = part_one::part_one();
    dbg!(part_one);
    let part_two = part_two::part_two();
    dbg!(part_two);
}
