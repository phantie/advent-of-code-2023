#![allow(unused)]

mod part_one {
    use super::*;

    pub fn part_one() -> usize {
        read_input()
            .map(Result::unwrap)
            .map(parse_line)
            .map(|(pattern, damaged_seq)| get_possible_combinations(pattern, damaged_seq))
            .sum::<usize>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 7361);
    }
}

// relatively slow, but clean
fn arrangement_count(pattern: &[Cell], damaged_seq: &[usize]) -> usize {
    fn matches_pattern(pattern: &[Cell], perm: &[Cell]) -> bool {
        pattern
            .into_iter()
            .zip(perm)
            .all(|(pat, perm)| pat == perm || pat.is_unknown())
    }

    fn matches_damaged_seq(damaged_seq: &[usize], perm: &[Cell]) -> bool {
        use itertools::Itertools;
        let group = perm.into_iter().group_by(|cell| cell.is_damaged());
        let this_damaged_seq = group
            .into_iter()
            .filter(|(b, group)| *b)
            .map(|(b, group)| group.count())
            .collect::<Vec<_>>();
        damaged_seq == this_damaged_seq.as_slice()
    }

    permutations_with_replacement(&mut [Cell::Damaged, Cell::Operational], pattern.len())
        .into_iter()
        .filter(|perm| matches_pattern(pattern, perm))
        .filter(|perm| matches_damaged_seq(damaged_seq, perm))
        .count()
}

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumIs, Eq, Hash)]
enum Cell {
    Unknown,
    Operational,
    Damaged,
}

#[cfg(test)]
#[test]
fn test_calc() {
    use Cell::*;
    assert_eq!(
        arrangement_count(
            &[
                Unknown,
                Unknown,
                Unknown,
                Operational,
                Damaged,
                Damaged,
                Damaged,
            ],
            &[1, 1, 3],
        ),
        1
    );

    assert_eq!(
        arrangement_count(
            &[
                Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown,
                Damaged, Unknown, Damaged, Unknown, Damaged, Unknown,
            ],
            &[1, 3, 1, 6],
        ),
        1
    );

    assert_eq!(
        arrangement_count(
            &[
                Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown, Unknown,
                Unknown, Unknown, Unknown,
            ],
            &[3, 2, 1],
        ),
        10
    );
}

fn permutations_with_replacement<E: Clone>(items: &[E], length: usize) -> Vec<Vec<E>> {
    fn permutations_with_replacement_helper<E: Clone>(
        items: &[E],
        current_permutation: &mut Vec<E>,
        result: &mut Vec<Vec<E>>,
        length: usize,
    ) {
        if length == 0 {
            result.push(current_permutation.clone());
            return;
        }

        for item in items {
            current_permutation.push(item.clone());
            permutations_with_replacement_helper(items, current_permutation, result, length - 1);
            current_permutation.pop();
        }
    }

    let mut result = Vec::new();
    let mut current_permutation = Vec::new();
    permutations_with_replacement_helper(items, &mut current_permutation, &mut result, length);
    result
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

fn parse_line(value: String) -> (Vec<Cell>, Vec<usize>) {
    let mut s = value.split(" ");
    let pattern = s.next().unwrap();
    let damage_seq = s.next().unwrap();

    let pattern = pattern
        .chars()
        .map(|c| match c {
            '#' => Cell::Damaged,
            '.' => Cell::Operational,
            '?' => Cell::Unknown,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let damage_seq = damage_seq
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (pattern, damage_seq)
}

type Pattern = Vec<Cell>;
type DamagedSeq = Vec<usize>;

fn extend_input((mut pattern, mut damaged_seq): (Pattern, DamagedSeq)) -> (Pattern, DamagedSeq) {
    pattern.extend(
        std::iter::repeat(std::iter::once(Cell::Unknown).chain(pattern.clone().into_iter()))
            .take(4)
            .flatten(),
    );
    damaged_seq.extend(std::iter::repeat(damaged_seq.clone()).take(4).flatten());
    (pattern, damaged_seq)
}

fn fast_arrangement_count(pattern: &[Cell], damaged_seq: &[usize]) -> usize {
    unimplemented!()
}

mod part_two {
    use super::*;

    pub fn part_two() -> usize {
        read_input()
            .map(Result::unwrap)
            .map(parse_line)
            .map(extend_input)
            .map(|(pattern, damaged_seq)| get_possible_combinations(pattern, damaged_seq))
            .sum::<usize>()
    }

    #[cfg(test)]
    #[ignore]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 83317216247365);
    }
}

fn main() {
    let part_one = part_one::part_one();
    dbg!(part_one);
    // let part_two = part_two::part_two();
    // dbg!(part_two);
}

#[memoize::memoize]
fn get_possible_combinations(pattern: Vec<Cell>, damaged_seq: Vec<usize>) -> usize {
    if damaged_seq.len() == 0 {
        // check no damaged cells left
        // if any: 0 else: 1
        return pattern.iter().all(|c| !c.is_damaged()) as usize;
    } else {
        // if damaged_seq is not consumed but pattern is: 0
        if pattern.len() == 0 {
            return 0;
        }
    }

    use Cell::*;
    match pattern[0] {
        Operational => {
            // Case 1: pattern starts with dots -- skip the dots
            get_possible_combinations(
                pattern
                    .clone()
                    .into_iter()
                    .skip_while(|c| c.is_operational())
                    .collect(),
                damaged_seq,
            )
        }
        Damaged => {
            // Case 2: pattern starts with a hash -- try to match it with a group
            let damaged_seq_len = damaged_seq[0];
            if pattern.len() >= damaged_seq_len
                && pattern[..damaged_seq_len]
                    .iter()
                    .all(|c| !c.is_operational())
            {
                let pattern = pattern[damaged_seq_len..].to_vec();
                let damaged_seq = damaged_seq[1..].to_vec();

                if pattern.len() > 0 {
                    if !pattern[0].is_damaged() {
                        get_possible_combinations(pattern[1..].to_vec(), damaged_seq)
                    } else {
                        0
                    }
                } else {
                    get_possible_combinations(pattern, damaged_seq)
                }
            } else {
                0
            }
        }
        Unknown => {
            // Case 3: pattern starts with a question mark -- either match it or don't
            let mut acc = 0;
            acc += get_possible_combinations(pattern[1..].to_vec(), damaged_seq.clone());
            let damaged_seq_len = damaged_seq[0];
            if pattern.len() >= damaged_seq_len
                && pattern[..damaged_seq_len]
                    .iter()
                    .all(|c| !c.is_operational())
            {
                let pattern = pattern[damaged_seq_len..].to_vec();
                let damaged_seq = damaged_seq[1..].to_vec();
                if pattern.len() > 0 {
                    if !pattern[0].is_damaged() {
                        acc += get_possible_combinations(pattern[1..].to_vec(), damaged_seq);
                    }
                } else {
                    acc += get_possible_combinations(pattern, damaged_seq);
                }
            }
            acc
        }
    }
}
