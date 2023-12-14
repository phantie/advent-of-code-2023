fn main() {
    dbg!(part_one::part_one());
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIs)]
enum Cell {
    Empty,
    RoundedRock,
    CubeRock,
}

type Column = Vec<Cell>;
type Row = Vec<Cell>;
type Space = Vec<Row>;

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

fn width(space: &Space) -> usize {
    space[0].len()
}

fn get_column(space: &Space, col: usize) -> Column {
    space.into_iter().map(|row| row[col]).collect()
}

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

fn move_north(col: Column) -> Column {
    use itertools::Itertools;

    let mut new_vec = vec![];

    for (i, group) in col
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

            new_vec.extend(
                (0..round_rock_count)
                    .map(|_| Cell::RoundedRock)
                    .chain((0..this.len() - round_rock_count).map(|_| Cell::Empty)),
            );
        }
    }

    // dbg!(&new_vec);

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
