mod part_one {
    pub fn part_one() -> u32 {
        super::read_input()
            .map(Result::unwrap)
            .map(extract_calibration_value)
            .sum()
    }

    fn extract_calibration_value(value: String) -> u32 {
        let first_num = value.chars().find(|char| char.is_numeric()).unwrap();
        let last_num = value.chars().rev().find(|char| char.is_numeric()).unwrap();
        format!("{first_num}{last_num}").parse().unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_part_one() {
            assert_eq!(part_one(), 53194);
        }
    }
}

mod part_two {
    pub fn part_two() -> u32 {
        super::read_input()
            .map(Result::unwrap)
            .map(extract_real_calibration_value)
            .sum()
    }

    fn extract_real_calibration_value(value: String) -> u32 {
        let nums = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        fn spelling_to_num(value: &str) -> u32 {
            match value {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => unreachable!(),
            }
        }

        let first_num: u32 = 'first_num: {
            let maybe_first_num_1 = nums
                .into_iter()
                .map(|num| {
                    value
                        .find(num)
                        .map(|idx| (idx, &value[idx..(idx + num.len())]))
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .min_by_key(|(idx, _)| *idx);

            let (idx_2, first_num_2) = value
                .chars()
                .enumerate()
                .find(|(_idx, char)| char.is_numeric())
                .unwrap();

            {
                if let Some((idx_1, first_num_1)) = maybe_first_num_1 {
                    if idx_1 < idx_2 {
                        break 'first_num spelling_to_num(&first_num_1);
                    } else {
                        break 'first_num first_num_2.to_string().parse().unwrap();
                    }
                }

                first_num_2.to_string().parse().unwrap()
            }
        };

        let last_num: u32 = 'last_num: {
            let nums_reversed = nums.map(|v| v.chars().rev().collect::<String>());

            let maybe_last_num_1 = nums_reversed
                .into_iter()
                .map(|num| {
                    value
                        .chars()
                        .rev()
                        .collect::<String>()
                        .find(num.as_str())
                        .map(|idx| {
                            (
                                idx,
                                value.chars().rev().collect::<String>()[idx..(idx + num.len())]
                                    .chars()
                                    .rev()
                                    .collect::<String>(),
                            )
                        })
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .min_by_key(|(idx, _)| *idx);

            let (idx_2, last_num_2) = value
                .chars()
                .rev()
                .enumerate()
                .find(|(_idx, char)| char.is_numeric())
                .unwrap();

            {
                if let Some((idx_1, last_num_1)) = maybe_last_num_1 {
                    if idx_1 < idx_2 {
                        break 'last_num spelling_to_num(&last_num_1);
                    } else {
                        break 'last_num last_num_2.to_string().parse().unwrap();
                    }
                }
                last_num_2.to_string().parse().unwrap()
            }
        };

        first_num * 10 + last_num
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_part_two() {
            assert_eq!(extract_real_calibration_value("46threevqs8114".into()), 44);
            assert_eq!(
                extract_real_calibration_value("sevenntgvnrrqfvxh2ttnkgffour8fiveone".into()),
                71
            );
            assert_eq!(
                extract_real_calibration_value("fzrpfhbfvj6dbxbtfs7twofksfbshrzkdeightwoqg".into()),
                62
            );

            assert_eq!(part_two(), 54249);
        }
    }
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

fn main() {
    let result = part_one::part_one();
    println!("result: {result}");

    let result = part_two::part_two();
    println!("result: {result}");
}
