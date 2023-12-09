use std::str::FromStr;

fn main() {
    dbg!(read_input().next());

    let part_one = part_one::part_one();
    dbg!(part_one);

    // let part_one = test_input()
    //     .into_iter()
    //     .map(parse_line)
    //     .map(predict_next)
    //     .sum::<SeqItem>();
}

mod part_one {
    use super::*;

    pub fn part_one() -> SeqItem {
        read_input()
            .map(Result::unwrap)
            .map(parse_line)
            .map(predict_next)
            .sum::<SeqItem>()
    }
}

fn parse_line(value: String) -> Seq {
    value
        .split(" ")
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect()
}

#[allow(unused)]
fn test_input() -> Vec<String> {
    "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"
        .split("\n")
        .map(ToOwned::to_owned)
        .collect()
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

type SeqItem = i32;
type Seq = Vec<SeqItem>;

fn predict_next(seq: Seq) -> SeqItem {
    let mut diffs = vec![seq];

    loop {
        let diff_seq = diffs
            .last()
            .unwrap()
            .as_slice()
            .windows(2)
            .map(|window| {
                let (l, r) = (window[0], window[1]);
                let diff = r - l;
                diff
            })
            .collect::<Vec<_>>();

        let all_zeroes = diff_seq.iter().all(|v| *v == 0);

        diffs.push(diff_seq);

        if all_zeroes {
            break;
        }
    }

    diffs.as_slice().windows(2).rev().fold(0, |acc, window| {
        let (upper, _lower) = (&window[0], &window[1]);
        upper.last().unwrap() + acc
    })
}
