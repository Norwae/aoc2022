use nom::character::complete::{alpha1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::terminated;
use crate::util::read_to_eof_line;

#[derive(Debug)]
struct Rucksack {
    // bit 0 is error, rest index
    whole: u64,
    left: u64,
    right: u64
}

impl  Rucksack {
    fn new(stringy: &str) -> Self {
        let (left_str, right_str) = stringy.split_at(stringy.len() / 2);
        let mut left = 0u64;
        let mut right = 0u64;
        let mut whole = 0u64;

        for ch in left_str.chars() {
            let coded = Self::charcode_to_flag(charcode(ch));
            left = left | coded;
            whole = whole | coded;
        }

        for ch in right_str.chars() {
            let coded = Self::charcode_to_flag(charcode(ch));
            right = right | coded;
            whole = whole | coded;
        }

        Self { left, right, whole }
    }

    fn mismatched_item_code(&self) -> i32 {
        let intersect = self.left & self.right;
        Self::flag_to_charcode(intersect)
    }

    fn lowest_item_code(&self) -> i32 {
        Self::flag_to_charcode(self.whole)
    }

    fn flag_to_charcode(flag: u64) -> i32 {
        flag.trailing_zeros() as i32
    }

    fn charcode_to_flag(charcode: i32) -> u64 {
        1 << charcode
    }

    fn intersect(&self, other: &Rucksack) -> Self {
        let whole = self.whole & other.whole;
        Rucksack {
            whole,
            left: 1,
            right: 1
        }
    }
}

fn charcode(ch: char) -> i32 {
    const ACODE: i32 = 'a' as i32;
    const ZCODE: i32 = 'z' as i32;
    const CACODE: i32 = 'A' as i32;
    const CZCODE: i32 = 'Z' as i32;

    let ch = ch as i32;

    if  ch >= ACODE && ch <= ZCODE{
        ch - ACODE + 1
    } else if ch >= CACODE && ch <= CZCODE {
        ch - CACODE + 27
    } else {
        0
    }
}

fn rucksack_lines(input: &str) -> IResult<&str, Vec<Rucksack>> {
    many1(
        map(
            terminated(alpha1, line_ending),
            |whole: &str| {
                Rucksack::new(whole)
            },
        )
    )(input)
}


pub fn solve() {
    let input = read_to_eof_line();
    if let Ok(("", lines)) = rucksack_lines(&input) {
        let part1 = lines.iter().fold(0i32, |sum: i32, rs: &Rucksack|{
            sum + rs.mismatched_item_code()
        });
        println!("Part 1: {}", part1);
        let mut part2 = 0i32;
        let mut chunks = lines.chunks(3);
        while let Some([r1, r2, r3]) = chunks.next() {
            part2 += r1.intersect(r2).intersect(r3).lowest_item_code();
        }

        println!("Part2: {}", part2);
    }
}