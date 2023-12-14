mod part_one {
    use super::*;

    pub fn part_one() -> usize {
        let space = parse_input();
        (0..width(&space))
            .map(|i| calc_weight(move_north(get_column(&space, i))))
            .sum::<usize>()
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
        let _space = parse_input();

        let mut space: Space = _space.clone();

        fn run_cycle(mut space: Space) -> Space {
            space = (0..width(&space))
                .map(|i| move_north(get_column(&space, i)))
                .collect();
            space = transpose(space);

            space = (0..height(&space))
                .map(|i| move_west(get_row(&space, i)))
                .collect();

            space = (0..width(&space))
                .map(|i| move_south(get_column(&space, i)))
                .collect();
            space = transpose(space);

            space = (0..height(&space))
                .map(|i| move_east(get_row(&space, i)))
                .collect();

            space
        }

        let mut seen = vec![_space.clone()];

        loop {
            space = run_cycle(space);

            match seen.iter().position(|_space| _space == &space) {
                None => seen.push(space.clone()),
                Some(cycle_start) => {
                    let cycle_len = seen.len() - cycle_start;

                    let space =
                        seen[cycle_start + (1_000_000_000 - cycle_start) % cycle_len].clone();

                    return (0..width(&space))
                        .map(|i| calc_weight(get_column(&space, i)))
                        .sum::<usize>();
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

fn width(space: &Space) -> usize {
    space[0].len()
}

fn height(space: &Space) -> usize {
    space.len()
}

fn get_column(space: &Space, col: usize) -> Column {
    space.into_iter().map(|row| row[col]).collect()
}

fn get_row(space: &Space, row: usize) -> Row {
    space[row].clone()
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

enum MoveTo {
    Start,
    End,
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

fn roll_rocks(cells: CellSeq, move_to: MoveTo) -> Column {
    use itertools::Itertools;

    let mut new_vec = vec![];

    for (i, group) in cells
        .clone()
        .into_iter()
        .enumerate()
        .group_by(|(_, c)| c.is_cube_rock())
        .into_iter()
    {
        if i {
            new_vec.extend(group.map(|(_, c)| c));
        } else {
            let this = group.collect::<Vec<_>>();
            let round_rock_count = this.iter().filter(|(_, c)| c.is_rounded_rock()).count();

            let round_rocks = (0..round_rock_count).map(|_| Cell::RoundedRock);
            let empty_space = (0..this.len() - round_rock_count).map(|_| Cell::Empty);

            match move_to {
                MoveTo::Start => new_vec.extend(round_rocks.chain(empty_space)),
                MoveTo::End => new_vec.extend(empty_space.chain(round_rocks)),
            }
        }
    }

    new_vec
}

fn calc_weight(col: Column) -> usize {
    col.clone()
        .into_iter()
        .enumerate()
        .filter(|(_, c)| c.is_rounded_rock())
        .map(|(i, _)| col.len() - i)
        .sum::<usize>()
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

fn test_input() -> &'static str {
    "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
}

fn main() {
    dbg!(part_one::part_one());
    dbg!(part_two::part_two());
}
