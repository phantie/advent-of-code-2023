#![allow(unused)]

#[cfg(test)]
mod part_one {
    use super::*;

    fn part_one() -> usize {
        tiles_energized(
            read_input()
                .map(Result::unwrap)
                .map(|l| parse_line(&l))
                .collect::<Space>(),
            (0, -1),
            Direction::Right,
        )
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 8125);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;

    fn part_two() -> usize {
        let space = read_input()
            .map(Result::unwrap)
            .map(|l| parse_line(&l))
            .collect::<Space>();

        generate_starting_positions_directions(&space)
            .into_iter()
            .map(|(pos, direction)| tiles_energized(space.clone(), pos, direction))
            .max()
            .unwrap()
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 8489);
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Space = Vec<Row>;
type Row = Vec<(Cell, VisitedFromDirection)>;

type X = isize;
type Y = isize;
type Pos = (Y, X);

type VisitedFromUp = bool;
type VisitedFromDown = bool;
type VisitedFromLeft = bool;
type VisitedFromRight = bool;

type VisitedFromDirection = (
    VisitedFromUp,
    VisitedFromDown,
    VisitedFromLeft,
    VisitedFromRight,
);

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,               /* . */
    HorizontalSplitter,  /* - */
    VerticalSplitter,    /* | */
    DownwardSlopeMirror, /* \ */
    UpwardSlopeMirror,   /* / */
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            '\\' => Self::DownwardSlopeMirror,
            '/' => Self::UpwardSlopeMirror,
            _ => unreachable!(),
        }
    }
}

enum NextDirections {
    One(Direction),
    Split((Direction, Direction)),
    Stop,
}

impl From<Direction> for NextDirections {
    fn from(value: Direction) -> Self {
        Self::One(value)
    }
}

impl From<(Direction, Direction)> for NextDirections {
    fn from(value: (Direction, Direction)) -> Self {
        Self::Split(value)
    }
}

fn move_to_direction((y, x): Pos, direction: Direction) -> Pos {
    use Direction::*;
    match direction {
        Up => (y - 1, x),
        Down => (y + 1, x),
        Right => (y, x + 1),
        Left => (y, x - 1),
    }
}

fn next_directions(
    direction: Direction,
    encountered_cell: Cell,
    visited_from_direction: VisitedFromDirection,
) -> NextDirections {
    use Cell::*;
    use Direction::*;
    use NextDirections::Stop;
    match (direction, encountered_cell, visited_from_direction) {
        (_, Empty, _) => direction.into(),

        (Left, _, (_, _, true, _)) => Stop,
        (Right, _, (_, _, _, true)) => Stop,
        (Up, _, (true, _, _, _)) => Stop,
        (Down, _, (_, true, _, _)) => Stop,

        (Right, HorizontalSplitter, (_, _, _, false)) => direction.into(),
        (Left, HorizontalSplitter, (_, _, false, _)) => direction.into(),
        (Up, HorizontalSplitter, (false, _, _, _)) => (Left, Right).into(),
        (Down, HorizontalSplitter, (_, false, _, _)) => (Left, Right).into(),

        (Up, VerticalSplitter, (false, _, _, _)) => direction.into(),
        (Down, VerticalSplitter, (_, false, _, _)) => direction.into(),
        (Left, VerticalSplitter, (_, _, false, _)) => (Up, Down).into(),
        (Right, VerticalSplitter, (_, _, _, false)) => (Up, Down).into(),

        /* \ */
        (Left, DownwardSlopeMirror, (_, _, false, _)) => Up.into(),
        (Right, DownwardSlopeMirror, (_, _, _, false)) => Down.into(),
        (Up, DownwardSlopeMirror, (false, _, _, _)) => Left.into(),
        (Down, DownwardSlopeMirror, (_, false, _, _)) => Right.into(),

        /* / */
        (Left, UpwardSlopeMirror, (_, _, false, _)) => Down.into(),
        (Right, UpwardSlopeMirror, (_, _, _, false)) => Up.into(),
        (Up, UpwardSlopeMirror, (false, _, _, _)) => Right.into(),
        (Down, UpwardSlopeMirror, (_, false, _, _)) => Left.into(),
    }
}

fn visit_from_direction(direction: Direction, visited_from_direction: &mut VisitedFromDirection) {
    use Direction::*;
    match direction {
        Up => visited_from_direction.0 = true,
        Down => visited_from_direction.1 = true,
        Left => visited_from_direction.2 = true,
        Right => visited_from_direction.3 = true,
    }
}

fn tiles_energized(space: Space, pos: Pos, direction: Direction) -> usize {
    fn explore_path(
        (mut space, mut unique_nodes): (Space, UniqueNodes),
        direction: Direction,
        pos: Pos,
    ) -> (Space, UniqueNodes) {
        let initial_direction = direction;
        let pos = move_to_direction(pos, direction);

        match pick_space(&space, pos) {
            None => (space, unique_nodes),
            Some((cell, visited_from_direction)) => {
                unique_nodes.insert(pos);
                match next_directions(direction, cell, visited_from_direction) {
                    NextDirections::One(direction) => {
                        let (_, visited_from_direction) = pick_space_mut(&mut space, pos).unwrap();
                        visit_from_direction(initial_direction, visited_from_direction);
                        explore_path((space, unique_nodes), direction, pos)
                    }
                    NextDirections::Split((d1, d2)) => {
                        let (_, visited_from_direction) = pick_space_mut(&mut space, pos).unwrap();
                        visit_from_direction(initial_direction, visited_from_direction);
                        explore_path(explore_path((space, unique_nodes), d1, pos), d2, pos)
                    }
                    NextDirections::Stop => (space, unique_nodes),
                }
            }
        }
    }

    type UniqueNodes = std::collections::HashSet<Pos>;
    let unique_nodes = UniqueNodes::new();

    let (_space, unique_nodes) = explore_path((space, unique_nodes), direction, pos);

    unique_nodes.len()
}

fn generate_starting_positions_directions(space: &Space) -> Vec<(Pos, Direction)> {
    let width = width(space) as isize;
    let height = height(space) as isize;

    use Direction::*;

    let top = (0..width).map(|i| ((-1, i), Down));
    let bottom = (0..width).map(|i| ((height, i), Up));

    let left = (0..height).map(|j| ((j, -1), Right));
    let right = (0..height).map(|j| ((j, width), Left));

    top.clone()
        .chain(bottom.clone())
        .chain(left.clone())
        .chain(right.clone())
        .collect()
}

fn pick_space(space: &Space, (y, x): Pos) -> Option<(Cell, VisitedFromDirection)> {
    space
        .get(y as usize)
        .map(|row| row.get(x as usize))
        .flatten()
        .cloned()
}

fn pick_space_mut(space: &mut Space, (y, x): Pos) -> Option<&mut (Cell, VisitedFromDirection)> {
    space
        .get_mut(y as usize)
        .map(|row| row.get_mut(x as usize))
        .flatten()
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_input_tiles_energized() {
        let space = test_input().map(parse_line).collect::<Space>();
        assert_eq!(tiles_energized(space, (0, -1), Direction::Right), 46);
    }

    #[test]
    fn test_test_input_2_tiles_energized() {
        let space = test_input_2().map(parse_line).collect::<Space>();
        assert_eq!(tiles_energized(space, (0, -1), Direction::Right), 5);
    }

    #[cfg(test)]
    fn test_input_2() -> impl Iterator<Item = &'static str> {
        r#".|\
.\/"#
            .lines()
    }

    #[cfg(test)]
    fn test_input() -> impl Iterator<Item = &'static str> {
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
            .lines()
    }
}

fn width(space: &Space) -> usize {
    space[0].len()
}

fn height(space: &Space) -> usize {
    space.len()
}

fn parse_line(v: &str) -> Row {
    v.chars()
        .map(|c| (c.into(), (false, false, false, false)))
        .collect()
}
