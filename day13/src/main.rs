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

fn check_full_col_reflection_smudged(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection_smudged(group, ab, get_column, width)
}

fn check_full_row_reflection_smudged(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection_smudged(group, ab, get_row, height)
}

fn check_full_reflection_smudged(
    group: &Group,
    (initial_a, initial_b): AB,
    get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    upper_limit: impl Fn(&Group) -> usize,
) -> Option<usize> {
    type FixedSmuged = bool;
    fn check_reflection_smudged(
        group: &Group,
        (a, b): AB,
        get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    ) -> (bool, FixedSmuged) {
        let a = get_row_or_col(group, a);
        let b = get_row_or_col(group, b);

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

    let (mut a, mut b) = (Some(initial_a), Some(initial_b));

    let mut fixed_smudge = false;
    loop {
        match (a, b) {
            (None, None) => unreachable!(),
            (Some(_a), Some(_b)) => {
                match check_reflection_smudged(group, (_a, _b), &get_row_or_col) {
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
                    Some(initial_a + 1)
                } else {
                    None
                };
            }
            (None, Some(_)) => {
                return if fixed_smudge {
                    Some(initial_a + 1)
                } else {
                    None
                };
            }
        }
    }
}

fn check_full_col_reflection(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection(group, ab, get_column, width)
}

fn check_full_row_reflection(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection(group, ab, get_row, height)
}

fn check_full_reflection(
    group: &Group,
    (initial_a, initial_b): AB,
    get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    upper_limit: impl Fn(&Group) -> usize,
) -> Option<usize> {
    fn check_reflection(
        group: &Group,
        (a, b): AB,
        get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    ) -> bool {
        get_row_or_col(group, a) == get_row_or_col(group, b)
    }

    let (mut a, mut b) = (Some(initial_a), Some(initial_b));

    loop {
        match (a, b) {
            (None, None) => unreachable!(),
            (Some(_a), Some(_b)) => {
                if !check_reflection(group, (_a, _b), &get_row_or_col) {
                    return None;
                }

                (a, b) = move_away_indeces((_a, _b), 1, upper_limit(group));
            }
            (Some(_l), None) => {
                return Some(initial_a + 1);
            }
            (None, Some(_r)) => {
                return Some(initial_a + 1);
            }
        }
    }
}

fn calc_group(group: Group) -> usize {
    let col_indeces = generate_initital_column_indeces(&group);

    let col_reflection = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_col_reflection(&group, (l, r)));

    let row_indeces = generate_initial_row_indeces(&group);

    let row_reflection = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_row_reflection(&group, (l, r)));

    col_reflection.unwrap_or(0) + row_reflection.unwrap_or(0) * 100
}

fn calc_group_smudged(group: Group) -> usize {
    let col_indeces = generate_initital_column_indeces(&group);

    let col_reflection = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_col_reflection_smudged(&group, (l, r)));

    let row_indeces = generate_initial_row_indeces(&group);

    let row_reflection = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_row_reflection_smudged(&group, (l, r)));

    col_reflection.unwrap_or(0) + row_reflection.unwrap_or(0) * 100
}

type Index = usize;
type AB = (Index, Index);

type Row = Vec<Cell>;
type Group = Vec<Row>;
type Input = Vec<Group>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Ash,
    Rock,
}

fn generate_initital_column_indeces(group: &Group) -> Vec<AB> {
    generate_initial_indeces(width(group))
}

fn generate_initial_row_indeces(group: &Group) -> Vec<AB> {
    generate_initial_indeces(height(group))
}

fn generate_initial_indeces(count: usize) -> Vec<AB> {
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

fn move_away_indeces((a, b): AB, c: usize, upper_limit: usize) -> (Option<usize>, Option<usize>) {
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

fn main() {}

fn input() -> &'static str {
    include_str!("../input.txt")
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
            .find_map(|(l, r)| check_full_row_reflection_smudged(&group, (l, r)));
        assert_eq!(r, Some(3));
    }

    #[test]
    fn test_calc_test_group_2_smudged() {
        let group = parse_group(test_group_2());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_row_reflection_smudged(&group, (l, r)));
        assert_eq!(r, Some(1));
    }

    #[test]
    fn test_calc_test_group_1() {
        let group = parse_group(test_group_1());

        let col_indeces = generate_initital_column_indeces(&group);

        let r = col_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_col_reflection(&group, (l, r)));

        assert_eq!(r, Some(5));
    }

    #[test]
    fn test_calc_test_group_2() {
        let group = parse_group(test_group_2());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_row_reflection(&group, (l, r)));

        assert_eq!(r, Some(4));
    }
}
