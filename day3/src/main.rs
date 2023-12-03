#![allow(unused)]

mod part_one {
    pub fn part_one() -> u32 {
        let r = find(
            super::read_input()
                .map(Result::unwrap)
                .map(parse_line)
                .collect::<Vec<_>>(),
        );
        dbg!(r);

        r
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
    let result = part_one::part_one();
    println!("result: {result}");
}
