mod part_one {
    use super::*;

    #[allow(unused)]
    fn part_one() -> usize {
        parse_input().into_iter().map(calc_group).sum::<usize>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 30705);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Ash,
    Rock,
}

fn main() {
    let r = parse_input().into_iter().map(calc_group).sum::<usize>();
    dbg!(r);
}

#[test]
fn test_calc_test_group_1() {
    let test_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    let group = parse_group(test_input);

    let col_indeces = generate_initital_column_indeces(&group);

    let r = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_column_reflection(&group, (l, r)));

    assert_eq!(r, Some(5));
}

#[test]
fn test_calc_test_group_2() {
    let test_input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    let group = parse_group(test_input);

    let row_indeces = generate_initial_row_indeces(&group);

    let r = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_row_reflection(&group, (l, r)));

    assert_eq!(r, Some(4));
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

type Row = Vec<Cell>;
type Group = Vec<Row>;
type Input = Vec<Group>;

fn generate_initital_column_indeces(group: &Group) -> Vec<(usize, usize)> {
    generate_initial_indeces(width(group))
}

fn generate_initial_row_indeces(group: &Group) -> Vec<(usize, usize)> {
    generate_initial_indeces(height(group))
}

fn generate_initial_indeces(count: usize) -> Vec<(usize, usize)> {
    (0..count)
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        .map(|window| {
            let (l, r) = (window[0], window[1]);
            (l, r)
        })
        .collect()
}

fn move_away_indeces(
    (a, b): (usize, usize),
    c: usize,
    upper_limit: usize,
) -> (Option<usize>, Option<usize>) {
    (
        if c > a { None } else { Some(a - c) },
        if c + b >= upper_limit {
            None
        } else {
            Some(b + c)
        },
    )
}

fn width(group: &Group) -> usize {
    group[0].len()
}

fn height(group: &Group) -> usize {
    group.len()
}

fn get_column(group: &Group, col: usize) -> Vec<Cell> {
    group.into_iter().map(|row| row[col]).collect()
}

fn get_row(group: &Group, row: usize) -> Vec<Cell> {
    group[row].clone()
}

fn check_column_reflection(group: &Group, (l, r): (usize, usize)) -> bool {
    let l_col = get_column(group, l);
    let r_col = get_column(group, r);
    l_col == r_col
}

fn check_row_reflection(group: &Group, (u, l): (usize, usize)) -> bool {
    let u_row = get_row(group, u);
    let l_row = get_row(group, l);
    u_row == l_row
}

fn check_full_column_reflection(
    group: &Group,
    (initital_l, initital_r): (usize, usize),
) -> Option<usize> {
    let (mut l, mut r) = (Some(initital_l), Some(initital_r));

    loop {
        match (l, r) {
            (None, None) => unreachable!(),
            (Some(_l), Some(_r)) => {
                if !check_column_reflection(group, (_l, _r)) {
                    return None;
                }

                (l, r) = move_away_indeces((_l, _r), 1, width(group));
            }
            (Some(_l), None) => {
                return Some(initital_l + 1);
            }
            (None, Some(_r)) => {
                return Some(initital_l + 1);
            }
        }
    }
}

fn check_full_row_reflection(
    group: &Group,
    (initital_u, initital_l): (usize, usize),
) -> Option<usize> {
    let (mut u, mut l) = (Some(initital_u), Some(initital_l));

    loop {
        match (u, l) {
            (None, None) => unreachable!(),
            (Some(_u), Some(_l)) => {
                if !check_row_reflection(group, (_u, _l)) {
                    return None;
                }

                (u, l) = move_away_indeces((_u, _l), 1, height(group));
            }
            (Some(_), None) => {
                return Some(initital_u + 1);
            }
            (None, Some(_)) => {
                return Some(initital_u + 1);
            }
        }
    }
}

fn calc_group(group: Group) -> usize {
    let col_indeces = generate_initital_column_indeces(&group);

    let col_reflection = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_column_reflection(&group, (l, r)));

    // dbg!(col_reflection);

    let row_indeces = generate_initial_row_indeces(&group);

    let row_reflection = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_row_reflection(&group, (l, r)));

    // dbg!(row_reflection);

    col_reflection.unwrap_or(0) + row_reflection.unwrap_or(0) * 100
}

fn parse_group(value: &str) -> Group {
    value
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Cell::Rock,
                    '.' => Cell::Ash,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn parse_input() -> Input {
    let input = input();
    input.split("\n\n").map(parse_group).collect()
}
