#![allow(unused)]
// Modified https://doc.rust-lang.org/std/collections/binary_heap/index.html example

mod part_one {
    use super::*;

    fn part_one() -> usize {
        let input = input();
        let space = parse_input(input);
        shortest_path(
            &space,
            (0, 0),
            ((height(&space) - 1) as _, (width(&space) - 1) as _),
        )
        .unwrap()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 1110);
    }
}

mod part_two {
    use super::*;

    fn part_two() -> usize {
        let input = input();
        let space = parse_input(input);
        shortest_path_two(
            &space,
            (0, 0),
            ((height(&space) - 1) as _, (width(&space) - 1) as _),
        )
        .unwrap()
    }

    #[test]
    fn test_part_two_test_input() {
        let input = test_input();
        let space = parse_input(input);
        assert_eq!(
            shortest_path_two(
                &space,
                (0, 0),
                ((height(&space) - 1) as _, (width(&space) - 1) as _),
            )
            .unwrap(),
            94
        );
    }

    #[test]
    fn test_part_two_test_input_2() {
        let input = test_input_2();
        let space = parse_input(input);
        assert_eq!(
            shortest_path_two(
                &space,
                (0, 0),
                ((height(&space) - 1) as _, (width(&space) - 1) as _),
            )
            .unwrap(),
            71
        );
    }
}

type HeatLoss = usize;
type Cell = HeatLoss;
type Row = Vec<Cell>;
type Space = Vec<Row>;

type X = isize;
type Y = isize;
type Pos = (Y, X);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Pos,
    direction: Direction,
    same_direction_streak: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct DistKey {
    position: Pos,
    direction: Direction,
    same_direction_streak: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(space: &Space, start: Pos, goal: Pos) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    use strum::IntoEnumIterator;

    let mut dist = std::collections::HashMap::<DistKey, usize>::new();

    let mut heap = std::collections::BinaryHeap::new();

    let directions = Direction::iter().collect::<Vec<_>>();

    // We're at `start`, with a zero cost
    dist.insert(
        DistKey {
            position: start,
            direction: Direction::Right,
            same_direction_streak: 0,
        },
        0,
    );
    dist.insert(
        DistKey {
            position: start,
            direction: Direction::Down,
            same_direction_streak: 0,
        },
        0,
    );
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::Right,
        same_direction_streak: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        direction,
        same_direction_streak,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        let dist_key = DistKey {
            position,
            direction,
            same_direction_streak,
        };
        if dist.contains_key(&dist_key) && cost > dist[&dist_key] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for _direction in directions
            .iter()
            .map(Clone::clone)
            .filter(|d| *d != direction.opposite())
        {
            let position = move_to_direction(position, _direction);
            if let None = pick_space(space, position) {
                continue;
            }

            let next = State {
                position: position,
                direction: _direction,
                same_direction_streak: if _direction == direction {
                    same_direction_streak + 1
                } else {
                    // Reset
                    1
                },
                cost: cost + space[position.0 as usize][position.1 as usize],
            };

            let dist_key = DistKey {
                position,
                direction: _direction,
                same_direction_streak: next.same_direction_streak,
            };
            // If so, add it to the frontier and continue
            if next.same_direction_streak <= 3
                && (!dist.contains_key(&dist_key) || next.cost < dist[&dist_key])
            {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(dist_key, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path_two(space: &Space, start: Pos, goal: Pos) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    use strum::IntoEnumIterator;

    let mut dist = std::collections::HashMap::<DistKey, usize>::new();

    let mut heap = std::collections::BinaryHeap::new();

    let directions = Direction::iter().collect::<Vec<_>>();

    // We're at `start`, with a zero cost
    dist.insert(
        DistKey {
            position: start,
            direction: Direction::Right,
            same_direction_streak: 0,
        },
        0,
    );
    dist.insert(
        DistKey {
            position: start,
            direction: Direction::Down,
            same_direction_streak: 0,
        },
        0,
    );
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::Right,
        same_direction_streak: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        direction,
        same_direction_streak,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if position == goal && same_direction_streak >= 4 {
            return Some(cost);
        }

        // Important as we may have already found a better way
        let dist_key = DistKey {
            position,
            direction,
            same_direction_streak,
        };
        if dist.contains_key(&dist_key) && cost > dist[&dist_key] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for _direction in directions
            .iter()
            .map(Clone::clone)
            .filter(|d| *d != direction.opposite())
        {
            let position = move_to_direction(position, _direction);
            if let None = pick_space(space, position) {
                continue;
            }

            let next = State {
                position: position,
                direction: _direction,
                same_direction_streak: if _direction == direction {
                    same_direction_streak + 1
                } else {
                    // Reset
                    1
                },
                cost: cost + space[position.0 as usize][position.1 as usize],
            };

            let dist_key = DistKey {
                position,
                direction: _direction,
                same_direction_streak: next.same_direction_streak,
            };

            // If so, add it to the frontier and continue
            if (direction == _direction || same_direction_streak >= 4)
                && next.same_direction_streak <= 10
                && (!dist.contains_key(&dist_key) || next.cost < dist[&dist_key])
            {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(dist_key, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn width(space: &Space) -> usize {
    space[0].len()
}

fn height(space: &Space) -> usize {
    space.len()
}

fn pick_space(space: &Space, (y, x): Pos) -> Option<Cell> {
    space
        .get(y as usize)
        .map(|row| row.get(x as usize))
        .flatten()
        .cloned()
}

fn pick_space_mut(space: &mut Space, (y, x): Pos) -> Option<&mut Cell> {
    space
        .get_mut(y as usize)
        .map(|row| row.get_mut(x as usize))
        .flatten()
}

fn move_to_direction((y, x): Pos, direction: Direction) -> Pos {
    use Direction::*;
    match direction {
        Up => (y - 1, x),
        Down => (y + 1, x),
        Right => (y, x + 1),
        Left => (y, x - 1),
    }
}

fn parse_input(v: &str) -> Space {
    v.lines()
        .map(|line| {
            line.chars()
                .map(|v| v.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

fn test_input() -> &'static str {
    "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
}

fn test_input_2() -> &'static str {
    "111111111111
999999999991
999999999991
999999999991
999999999991"
}

fn main() {}
