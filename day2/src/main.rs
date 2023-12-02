mod part_one {
    use super::*;

    pub fn part_one() -> u32 {
        read_input().map(Result::unwrap).map(id_if_possible).sum()
    }

    pub fn id_if_possible(v: String) -> u32 {
        let mut q = v.split(":");

        let p1 = q.next().unwrap();

        let id = {
            let mut q = p1.split(" ");
            let _ = q.next();
            q.next().unwrap().parse::<u32>().unwrap()
        };

        let games = q.next().unwrap();

        for game in games.split(";") {
            for count_color in game.split(",") {
                let count_color = count_color.trim();
                let mut z = count_color.trim().split(" ");

                let count = z.next().unwrap().parse::<u32>().unwrap();
                let color = z.next().unwrap();

                if color == "red" && count <= 12 {
                    continue;
                }
                if color == "green" && count <= 13 {
                    continue;
                }
                if color == "blue" && count <= 14 {
                    continue;
                }

                return 0;
            }
        }

        id
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn part_one() {
            assert_eq!(super::part_one(), 2617);
        }
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> u32 {
        read_input().map(Result::unwrap).map(mult_pow).sum()
    }

    pub fn mult_pow(v: String) -> u32 {
        let mut q = v.split(":");

        let _p1 = q.next().unwrap();

        let games = q.next().unwrap();

        let mut color_count = std::collections::HashMap::<&str, u32>::new();

        for game in games.split(";") {
            for count_color in game.split(",") {
                let mut z = count_color.trim().split(" ");
                let count = z.next().unwrap().parse::<u32>().unwrap();
                let color = z.next().unwrap();

                color_count
                    .entry(color)
                    .and_modify(|v| {
                        *v = (*v).max(count);
                    })
                    .or_insert(count);
            }
        }

        color_count.values().fold(1, |acc, x| acc * x)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn part_two() {
            assert_eq!(super::part_two(), 59795);
        }
    }
}

fn main() {
    dbg!(part_one::part_one()); // 2617
    dbg!(part_two::part_two()); // 59795
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}
