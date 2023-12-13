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

mod part_two {
    use super::*;

    #[allow(unused)]
    fn part_two() -> usize {
        parse_input()
            .into_iter()
            .map(calc_group_smudged)
            .sum::<usize>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 44615);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Ash,
    Rock,
}

fn main() {}

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

type Index = usize;
type AB = (Index, Index);

fn check_reflection(group: &Group, (a, b): AB, f: impl Fn(&Group, Index) -> Vec<Cell>) -> bool {
    f(group, a) == f(group, b)
}

fn check_reflection_smudged(
    group: &Group,
    (a, b): AB,
    f: impl Fn(&Group, Index) -> Vec<Cell>,
) -> (bool, FixedSmuged) {
    let a = f(group, a);
    let b = f(group, b);

    let eq = a
        .clone()
        .into_iter()
        .zip(b.into_iter())
        .filter(|(a, b)| a == b)
        .count();

    if eq == a.len() {
        (true, false)
    } else if eq + 1 == a.len() {
        (true, true)
    } else {
        (false, false)
    }
}

type FixedSmuged = bool;

fn check_full_reflection_smudged(
    group: &Group,
    (initital_a, initital_b): AB,
    f: impl Fn(&Group, Index) -> Vec<Cell> + Clone,
    upper_limit: impl Fn(&Group) -> usize,
) -> Option<usize> {
    let (mut a, mut b) = (Some(initital_a), Some(initital_b));

    let mut fixed_smudge = false;
    loop {
        match (a, b) {
            (None, None) => unreachable!(),
            (Some(_a), Some(_b)) => {
                match check_reflection_smudged(group, (_a, _b), f.clone()) {
                    (true, true) => {
                        if fixed_smudge {
                            return None;
                        } else {
                            fixed_smudge = true;
                        }
                    }
                    (true, false) => {}
                    (false, false) => {
                        return None;
                    }
                    (false, true) => unreachable!(),
                }

                (a, b) = move_away_indeces((_a, _b), 1, upper_limit(group));
            }
            (Some(_), None) => {
                return if fixed_smudge {
                    Some(initital_a + 1)
                } else {
                    None
                };
            }
            (None, Some(_)) => {
                return if fixed_smudge {
                    Some(initital_a + 1)
                } else {
                    None
                };
            }
        }
    }
}

fn check_full_reflection(
    group: &Group,
    (initital_a, initital_b): AB,
    f: impl Fn(&Group, Index) -> Vec<Cell> + Clone,
    upper_limit: impl Fn(&Group) -> usize,
) -> Option<usize> {
    let (mut a, mut b) = (Some(initital_a), Some(initital_b));

    loop {
        match (a, b) {
            (None, None) => unreachable!(),
            (Some(_a), Some(_b)) => {
                if !check_reflection(group, (_a, _b), f.clone()) {
                    return None;
                }

                (a, b) = move_away_indeces((_a, _b), 1, upper_limit(group));
            }
            (Some(_l), None) => {
                return Some(initital_a + 1);
            }
            (None, Some(_r)) => {
                return Some(initital_a + 1);
            }
        }
    }
}

fn calc_group(group: Group) -> usize {
    let col_indeces = generate_initital_column_indeces(&group);

    let col_reflection = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_reflection(&group, (l, r), get_column, width));

    let row_indeces = generate_initial_row_indeces(&group);

    let row_reflection = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_reflection(&group, (l, r), get_row, height));

    col_reflection.unwrap_or(0) + row_reflection.unwrap_or(0) * 100
}

fn calc_group_smudged(group: Group) -> usize {
    let col_indeces = generate_initital_column_indeces(&group);

    let col_reflection = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_reflection_smudged(&group, (l, r), get_column, width));

    let row_indeces = generate_initial_row_indeces(&group);

    let row_reflection = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_reflection_smudged(&group, (l, r), get_row, height));

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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_group_1() -> &'static str {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
    }

    fn test_group_2() -> &'static str {
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    }

    #[test]
    fn test_calc_test_group_1_smudged() {
        let group = parse_group(test_group_1());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_reflection_smudged(&group, (l, r), get_row, height));
        assert_eq!(r, Some(3));
    }

    #[test]
    fn test_calc_test_group_2_smudged() {
        let group = parse_group(test_group_2());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_reflection_smudged(&group, (l, r), get_row, height));
        assert_eq!(r, Some(1));
    }

    #[test]
    fn test_calc_test_group_1() {
        let group = parse_group(test_group_1());

        let col_indeces = generate_initital_column_indeces(&group);

        let r = col_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_reflection(&group, (l, r), get_column, width));

        assert_eq!(r, Some(5));
    }

    #[test]
    fn test_calc_test_group_2() {
        let group = parse_group(test_group_2());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_reflection(&group, (l, r), get_row, height));

        assert_eq!(r, Some(4));
    }
}
