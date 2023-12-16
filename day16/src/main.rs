fn test_input_2() -> impl Iterator<Item = &'static str> {
    r#".|\
.\/"#
        .lines()
}

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

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
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

fn parse_line(v: &str) -> Row {
    v.chars()
        .map(|c| (c.into(), (false, false, false, false)))
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Space = Vec<Row>;
type Row = Vec<(Cell, PassedFromSides)>;

type X = isize;
type Y = isize;
type Pos = (Y, X);

type PassedFromUp = bool;
type PassedFromDown = bool;
type PassedFromLeft = bool;
type PassedFromRight = bool;

type PassedFromSides = (
    PassedFromUp,
    PassedFromDown,
    PassedFromLeft,
    PassedFromRight,
);

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,               /* . */
    HorizontalSplitter,  /* - */
    VerticalSplitter,    /* | */
    DownwardSlopeMirror, /* \ */
    UpwardSlopeMirror,   /* / */
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
    passed_from_sides: PassedFromSides,
) -> NextDirections {
    use Cell::*;
    use Direction::*;
    use NextDirections::Stop;
    match (direction, encountered_cell, passed_from_sides) {
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

fn pick_space(space: &Space, (y, x): Pos) -> Option<(Cell, PassedFromSides)> {
    space
        .get(y as usize)
        .map(|row| row.get(x as usize))
        .flatten()
        .cloned()
}

fn pick_space_mut(space: &mut Space, (y, x): Pos) -> Option<&mut (Cell, PassedFromSides)> {
    space
        .get_mut(y as usize)
        .map(|row| row.get_mut(x as usize))
        .flatten()
}

fn passed_from_direction(direction: Direction, passed_from_sides: &mut PassedFromSides) {
    use Direction::*;
    match direction {
        Up => passed_from_sides.0 = true,
        Down => passed_from_sides.1 = true,
        Left => passed_from_sides.2 = true,
        Right => passed_from_sides.3 = true,
    }
}

fn tiles_energized(space: Space) -> usize {
    fn explore_path(
        (mut space, mut unique_nodes): (Space, UniqueNodes),
        direction: Direction,
        pos: Pos,
    ) -> (Space, UniqueNodes) {
        let initial_direction = direction;
        let pos = move_to_direction(pos, direction);

        // match pick_space_mut(&mut space, pos) {
        //     None => {}
        //     Some((_, passed_from_sides)) => {
        //         passed_from_direction(initial_direction, passed_from_sides);
        //     }
        // }

        match pick_space(&space, pos) {
            None => (space, unique_nodes),
            Some((cell, passed_from_sides)) => {
                unique_nodes.insert(pos);
                match next_directions(direction, cell, passed_from_sides) {
                    NextDirections::One(direction) => {
                        let (_, passed_from_sides) = pick_space_mut(&mut space, pos).unwrap();
                        passed_from_direction(initial_direction, passed_from_sides);
                        explore_path((space, unique_nodes), direction, pos)
                    }
                    NextDirections::Split((d1, d2)) => {
                        let (_, passed_from_sides) = pick_space_mut(&mut space, pos).unwrap();
                        passed_from_direction(initial_direction, passed_from_sides);
                        explore_path(explore_path((space, unique_nodes), d1, pos), d2, pos)
                    }
                    NextDirections::Stop => (space, unique_nodes),
                }
            }
        }
    }

    type UniqueNodes = std::collections::HashSet<Pos>;
    let unique_nodes = UniqueNodes::new();

    let pos = (0, -1);
    let direction = Direction::Right;
    let (_space, unique_nodes) = explore_path((space, unique_nodes), direction, pos);

    unique_nodes.len()
}

fn main() {
    // let input = test_input();

    // let input = test_input_2();
    // let q = input.map(parse_line).collect::<Space>();

    let q = read_input()
        .map(Result::unwrap)
        .map(|l| parse_line(&l))
        .collect::<Space>();

    // dbg!(q.len());
    // dbg!(q[0].len());

    // dbg!(&q);

    let r = tiles_energized(q);
    dbg!(r);
}

#[test]
fn test_test_input_tiles_energized() {
    let space = test_input().map(parse_line).collect::<Space>();
    assert_eq!(tiles_energized(space), 46);
}

#[test]
fn test_input_tiles_energized() {
    assert_eq!(
        tiles_energized(
            read_input()
                .map(Result::unwrap)
                .map(|l| parse_line(&l))
                .collect::<Space>()
        ),
        8125
    );
}

#[test]
fn test_test_input_2_tiles_energized() {
    let space = test_input_2().map(parse_line).collect::<Space>();
    assert_eq!(tiles_energized(space), 5);
}
