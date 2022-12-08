use std::ops::{BitAnd, BitOr};
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::terminated;
use crate::util::default_solution;


fn charcode(ch: char) -> i32 {
    const ACODE: i32 = 'a' as i32;
    const ZCODE: i32 = 'z' as i32;
    const CACODE: i32 = 'A' as i32;
    const CZCODE: i32 = 'Z' as i32;

    let ch = ch as i32;

    if ch >= ACODE && ch <= ZCODE {
        ch - ACODE + 1
    } else if ch >= CACODE && ch <= CZCODE {
        ch - CACODE + 27
    } else {
        0
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct FlagSet(u64);

impl FlagSet {
    fn new(str: &str) -> Self {
        let mut value = 0u64;
        for ch in str.chars() {
            let coded = 1 << charcode(ch);
            value = value | coded
        }
        Self(value)
    }

    fn to_charcode(self) -> i32 {
        self.0.trailing_zeros() as i32
    }
}

impl BitAnd for FlagSet {
    type Output = FlagSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        FlagSet(self.0 & rhs.0)
    }
}

impl BitOr for FlagSet {
    type Output = FlagSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        FlagSet(self.0 | rhs.0)
    }
}

#[derive(Debug)]
struct Rucksack {
    // bit 0 is error, rest index
    left: FlagSet,
    right: FlagSet,
}

impl Rucksack {
    fn new(stringy: &str) -> Self {
        let (left, right) = stringy.split_at(stringy.len() / 2);
        let left = FlagSet::new(left);
        let right = FlagSet::new(right);

        Self { left, right }
    }

    fn mismatched_item_code(&self) -> i32 {
        let intersect = self.left & self.right;
        intersect.to_charcode()
    }

    fn intersect_and_determine_present(&self, o1: &Rucksack, o2: &Rucksack) -> i32 {
        let r1 = self.left | self.right;
        let r2 = o1.left | o1.right;
        let r3 = o2.left | o2.right;
        (r1 & r2 & r3).to_charcode()
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

fn solve_problem(lines: Vec<Rucksack>) -> (i32, i32) {
    let part1 = lines.iter().fold(0i32, |sum: i32, rs: &Rucksack| {
        sum + rs.mismatched_item_code()
    });
    let mut part2 = 0i32;
    let mut chunks = lines.chunks(3);
    while let Some([r1, r2, r3]) = chunks.next() {
        part2 += r1.intersect_and_determine_present(r2, r3);
    }

    (part1, part2)
}

default_solution!(rucksack_lines, solve_problem);