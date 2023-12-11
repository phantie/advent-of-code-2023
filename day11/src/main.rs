fn main() {
    let r = part_one::part_one();
    dbg!(r);
}

mod part_one {
    use super::*;

    pub fn part_one() -> usize {
        let space = Space(read_input().map(Result::unwrap).map(parse_line).collect());

        let empty_rows = space.empty_rows();
        let empty_columns = space.empty_columns();

        let galaxy_pairs = space.galaxy_pairs();

        galaxy_pairs
            .into_iter()
            .map(|(l, r)| shortest_distance(l, r, &empty_rows, &empty_columns))
            .sum::<usize>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 9686930);
    }
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

type I = usize;
type J = usize;
type Pos = (I, J);

type SpaceRow = Vec<Cell>;

struct Space(Vec<SpaceRow>);

impl std::ops::Deref for Space {
    type Target = Vec<SpaceRow>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn shortest_distance(
    (y_l, x_l): Pos,
    (y_r, x_r): Pos,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
) -> usize {
    let x_expansion = empty_columns
        .into_iter()
        .filter(|i| **i > x_l.min(x_r) && **i < x_r.max(x_l))
        .count();

    let y_expansion = empty_rows
        .into_iter()
        .filter(|i| **i > y_l.min(y_r) && **i < y_r.max(y_l))
        .count();

    ((x_r as isize - x_l as isize).abs() + (y_r as isize - y_l as isize).abs()) as usize
        + x_expansion
        + y_expansion
}

impl Space {
    pub fn galaxy_pairs(&self) -> Vec<(Pos, Pos)> {
        let galaxies = self.galaxies();

        (0..galaxies.len())
            .map(|i| (0..galaxies.len()).map(move |j| (i, j)))
            .flatten()
            .filter(|(i, j)| i != j && i < j)
            .map(|(i, j)| (galaxies[i], galaxies[j]))
            .collect()
    }

    pub fn galaxies(&self) -> Vec<Pos> {
        self.iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, cell)| if cell.is_universe() { Some(i) } else { None })
            .map(|flat_index| num::integer::div_rem(flat_index, self.x_dim()))
            .collect()
    }

    pub fn x_dim(&self) -> usize {
        self[0].len()
    }

    pub fn empty_rows(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .filter(|(_i, row)| row.into_iter().all(|cell| cell.is_empty()))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn empty_columns(&self) -> Vec<usize> {
        (0..self.x_dim())
            .filter(|j| (0..self.len()).all(|i| self[i][*j].is_empty()))
            .collect()
    }
}

#[derive(strum::EnumIs, Clone, Copy)]
enum Cell {
    Empty,
    Universe,
}

fn parse_line(value: String) -> SpaceRow {
    value
        .chars()
        .map(|char| match char {
            '.' => Cell::Empty,
            '#' => Cell::Universe,
            _ => unreachable!(),
        })
        .collect()
}
