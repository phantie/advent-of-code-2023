fn main() {
    dbg!(part_one::part_one());
}

mod part_one {
    use super::*;

    pub fn part_one() -> u32 {
        parse_input(input()).map(hash).sum::<u32>()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 504036);
    }
}

fn hash(v: &str) -> u32 {
    v.chars().map(char_to_ascii_code).fold(0, |acc, x| {
        let (_div, rem) = num::integer::div_rem((acc + x) * 17, 256);
        rem
    })
}

fn char_to_ascii_code(c: char) -> u32 {
    c as u32
}

fn parse_input(i: &str) -> impl Iterator<Item = &str> {
    i.split(",")
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

#[test]
fn test_hash_example() {
    assert_eq!(hash("HASH"), 52);
}

#[test]
fn test_sequence_example() {
    let r = parse_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        .map(hash)
        .sum::<u32>();
    assert_eq!(r, 1320);
}
