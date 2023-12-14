mod part_one {
    use super::*;

    pub fn part_one() -> usize {
        space_weight(transpose(
            iter_cols(parse_input()).map(move_north).collect(),
        ))
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 107142);
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> usize {
        fn run_cycle(mut space: Space) -> Space {
            space = iter_cols(space).map(move_north).collect();
            space = transpose(space);

            space = iter_rows(space).map(move_west).collect();

            space = iter_cols(space).map(move_south).collect();
            space = transpose(space);

            space = iter_rows(space).map(move_east).collect();

            space
        }

        let space = parse_input();
        let mut space: Space = space.clone();
        let mut seen = vec![space.clone()];

        loop {
            space = run_cycle(space);

            match seen.iter().position(|_space| _space == &space) {
                None => seen.push(space.clone()),
                Some(cycle_start) => {
                    let cycle_len = seen.len() - cycle_start;

                    let space =
                        seen[cycle_start + (1_000_000_000 - cycle_start) % cycle_len].clone();

                    return space_weight(space);
                }
            }
        }
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 104815);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIs)]
enum Cell {
    Empty,
    RoundedRock,
    CubeRock,
}

type CellSeq = Vec<Cell>;
type Column = CellSeq;
type Row = CellSeq;
type Space = Vec<Row>;

enum MoveTo {
    Start,
    End,
}

fn roll_rocks(cells: CellSeq, move_to: MoveTo) -> CellSeq {
    use itertools::Itertools;

    cells
        .clone()
        .into_iter()
        .group_by(|c| c.is_cube_rock())
        .into_iter()
        .map(|(i, group)| -> Box<dyn Iterator<Item = Cell>> {
            if i {
                Box::new(group)
            } else {
                let group = group.collect::<Vec<_>>();
                let round_rock_count = group.iter().filter(|c| c.is_rounded_rock()).count();

                let round_rocks = (0..round_rock_count).map(|_| Cell::RoundedRock);
                let empty_space = (0..group.len() - round_rock_count).map(|_| Cell::Empty);

                match move_to {
                    MoveTo::Start => Box::new(round_rocks.chain(empty_space)),
                    MoveTo::End => Box::new(empty_space.chain(round_rocks)),
                }
            }
        })
        .flatten()
        .collect()
}

fn space_weight(space: Space) -> usize {
    fn col_weight(col: Column) -> usize {
        col.clone()
            .into_iter()
            .enumerate()
            .filter(|(_, c)| c.is_rounded_rock())
            .map(|(i, _)| col.len() - i)
            .sum::<usize>()
    }

    iter_cols(space).map(col_weight).sum::<usize>()
}

fn iter_cols(space: Space) -> impl Iterator<Item = Vec<Cell>> {
    (0..width(&space)).map(move |i| get_column(&space, i))
}

fn iter_rows(space: Space) -> impl Iterator<Item = Vec<Cell>> {
    (0..height(&space)).map(move |i| get_row(&space, i))
}

fn get_column(space: &Space, col: usize) -> Column {
    space.into_iter().map(|row| row[col]).collect()
}

fn get_row(space: &Space, row: usize) -> Row {
    space[row].clone()
}

fn width(space: &Space) -> usize {
    space[0].len()
}

fn height(space: &Space) -> usize {
    space.len()
}

fn move_north(col: Column) -> Column {
    roll_rocks(col, MoveTo::Start)
}

fn move_west(row: Row) -> Row {
    roll_rocks(row, MoveTo::Start)
}

fn move_south(col: Column) -> Column {
    roll_rocks(col, MoveTo::End)
}

fn move_east(row: Row) -> Row {
    roll_rocks(row, MoveTo::End)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_input() -> Space {
    let input = input();
    // let input = test_input();
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Cell::CubeRock,
                    '.' => Cell::Empty,
                    'O' => Cell::RoundedRock,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

fn main() {
    dbg!(part_one::part_one());
    dbg!(part_two::part_two());
}
