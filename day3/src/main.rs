#![allow(unused)]

mod part_one {
    pub fn part_one() -> u32 {
        find(
            super::read_input()
                .map(Result::unwrap)
                .map(parse_line)
                .collect::<Vec<_>>(),
        )
    }

    #[derive(strum::EnumIs, Clone)]
    enum Cell {
        Number(u32),
        Symbol,
        Empty,
    }

    fn parse_line(value: String) -> Vec<Cell> {
        std::iter::once(Cell::Empty)
            .chain(value.chars().map(|c| match c {
                c if c.is_numeric() => Cell::Number(c.to_string().parse().unwrap()),
                c if c == '.' => Cell::Empty,
                _c => Cell::Symbol,
            }))
            .chain(std::iter::once(Cell::Empty))
            .collect()
    }

    fn find(rows: Vec<Vec<Cell>>) -> u32 {
        let row_len = rows[0].len();
        let empty_row = (0..row_len).map(|_| Cell::Empty).collect::<Vec<_>>();

        let rows = std::iter::once(empty_row.clone())
            .chain(rows)
            .chain(std::iter::once(empty_row.clone()))
            .collect::<Vec<_>>();

        let mut total = 0;

        for (i, row) in rows.iter().enumerate() {
            if i == 0 || i == rows.len() - 1 {
                continue;
            }

            let prev_row = rows[i - 1].clone();
            let next_row = rows[i + 1].clone();

            let mut numbers = vec![];
            let mut finished_num = true;
            for (i, cell) in row.iter().enumerate() {
                if !cell.is_number() {
                    finished_num = true
                }

                if let Cell::Number(num) = cell {
                    if finished_num {
                        numbers.push((i, *num));
                        finished_num = false;
                    } else {
                        let (_i, last) = numbers.last_mut().unwrap();
                        *last = *last * 10 + num;
                    }
                }
            }

            for (i, num) in numbers.clone() {
                let num_len = (num.checked_ilog10().unwrap_or(0) + 1) as usize;

                let sym_precedes = row[i - 1].is_symbol();
                let sym_succeeds = row[i + num_len].is_symbol();

                let prev_row_match = ((i - 1)..=(i + num_len)).any(|i| prev_row[i].is_symbol());
                let next_row_match = ((i - 1)..=(i + num_len)).any(|i| next_row[i].is_symbol());

                if sym_precedes || sym_succeeds || prev_row_match || next_row_match {
                    total += num;
                }
            }
        }

        total
    }
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

fn main() {
    // let result = part_one::part_one();
    // println!("result: {result}");
    let result = part_two::part_two();
    println!("result: {result}");
}

mod part_two {
    pub fn part_two() -> u32 {
        let r = find(
            super::read_input()
                .map(Result::unwrap)
                .map(parse_line)
                .collect::<Vec<_>>(),
        );

        r
    }

    fn num_len(num: u32) -> usize {
        (num.checked_ilog10().unwrap_or(0) + 1) as usize
    }

    #[derive(strum::EnumIs, Clone)]
    enum Cell {
        Number(u32),
        Symbol,
        Empty,
    }

    fn parse_line(value: String) -> Vec<Cell> {
        std::iter::once(Cell::Empty)
            .chain(value.chars().map(|c| match c {
                c if c.is_numeric() => Cell::Number(c.to_string().parse().unwrap()),
                c if c == '.' => Cell::Empty,
                _c => Cell::Symbol,
            }))
            .chain(std::iter::once(Cell::Empty))
            .collect()
    }

    fn extract_nums(row: &Vec<Cell>) -> Vec<(usize, u32)> {
        let mut numbers = vec![];
        let mut finished_num = true;
        for (i, cell) in row.iter().enumerate() {
            if !cell.is_number() {
                finished_num = true
            }

            if let Cell::Number(num) = cell {
                if finished_num {
                    numbers.push((i, *num));
                    finished_num = false;
                } else {
                    let (_i, last) = numbers.last_mut().unwrap();
                    *last = *last * 10 + num;
                }
            }
        }
        numbers
    }

    fn find(rows: Vec<Vec<Cell>>) -> u32 {
        let row_len = rows[0].len();
        let empty_row = (0..row_len).map(|_| Cell::Empty).collect::<Vec<_>>();

        let rows = std::iter::once(empty_row.clone())
            .chain(rows)
            .chain(std::iter::once(empty_row.clone()))
            .collect::<Vec<_>>();

        let mut total = 0;

        for (i, row) in rows.iter().enumerate() {
            if i == 0 || i == rows.len() - 1 {
                continue;
            }

            let prev_row = rows[i - 1].clone();
            let next_row = rows[i + 1].clone();

            let mut symbols = vec![];
            for (i, cell) in row.iter().enumerate() {
                if let Cell::Symbol = cell {
                    symbols.push(i);
                }
            }

            let numbers = extract_nums(&row);

            for i in symbols {
                let matched_nums = numbers
                    .iter()
                    .filter(|(n_i, num)| {
                        let num_len = num_len(*num);
                        n_i + num_len == i || n_i - 1 == i
                    })
                    .collect::<Vec<_>>();

                let prev_row_matched = extract_nums(&prev_row)
                    .into_iter()
                    .filter(|(n_i, num)| ((*n_i - 1)..=(n_i + num_len(*num))).any(|n_i| n_i == i))
                    .collect::<Vec<_>>();

                let next_row_matched = extract_nums(&next_row)
                    .into_iter()
                    .filter(|(n_i, num)| ((*n_i - 1)..=(n_i + num_len(*num))).any(|n_i| n_i == i))
                    .collect::<Vec<_>>();

                if matched_nums.len() + prev_row_matched.len() + next_row_matched.len() == 2 {
                    total += matched_nums
                        .into_iter()
                        .chain(prev_row_matched.iter())
                        .chain(next_row_matched.iter())
                        .fold(1, |acc, (_, x)| acc * x)
                }
            }
        }

        total
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 87287096);
    }
}
