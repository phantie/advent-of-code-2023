mod part_one {
    use super::*;

    pub fn part_one() -> usize {
        let space = Space::default();
        space.shortest_distances(1)
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 9686930);
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> usize {
        let space = Space::default();
        space.shortest_distances(10usize.pow(6) - 1)
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 630728425490);
    }
}

pub fn shortest_distance(
    (x_l, y_l): Pos,
    (x_r, y_r): Pos,
    expansion_multiplier: usize,
    empty_rows: &[usize],
    empty_columns: &[usize],
) -> usize {
    let x_expansion = empty_columns
        .into_iter()
        .filter(|i| **i > x_l.min(x_r) && **i < x_r.max(x_l))
        .count()
        * expansion_multiplier;

    let y_expansion = empty_rows
        .into_iter()
        .filter(|i| **i > y_l.min(y_r) && **i < y_r.max(y_l))
        .count()
        * expansion_multiplier;

    ((x_r as isize - x_l as isize).abs() + (y_r as isize - y_l as isize).abs()) as usize
        + x_expansion
        + y_expansion
}

impl Space {
    pub fn shortest_distances(&self, expansion_multiplier: usize) -> usize {
        let space = self;

        let empty_rows = space.empty_rows();
        let empty_columns = space.empty_columns();

        let galaxy_pairs = space.galaxy_pairs();

        galaxy_pairs
            .into_iter()
            .map(|(l, r)| {
                shortest_distance(l, r, expansion_multiplier, &empty_rows, &empty_columns)
            })
            .sum::<usize>()
    }

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
            .map(|flat_index| {
                let (div, rem) = num::integer::div_rem(flat_index, self.x_dim());
                (rem, div)
            })
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

type X = usize;
type Y = usize;
type Pos = (X, Y);

type SpaceRow = Vec<Cell>;

struct Space(Vec<SpaceRow>);

impl Default for Space {
    fn default() -> Self {
        Self(read_input().map(Result::unwrap).map(parse_line).collect())
    }
}

impl std::ops::Deref for Space {
    type Target = Vec<SpaceRow>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
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

fn main() {
    let part_one = part_one::part_one();
    dbg!(part_one);
    let part_two = part_two::part_two();
    dbg!(part_two);
}
