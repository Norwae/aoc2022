use std::io::stdin;

use nom::{character, IResult};
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;

#[derive(Debug)]
struct Elf {
    supplies: u64,
}

impl Elf {
    fn new(supplies: Vec<u64>) -> Self {
        Self { supplies: supplies.into_iter().sum() }
    }

}

fn calorie_count(input: &str) -> IResult<&str, u64> {
    map(
        terminated(digit1, line_ending),
        |line| u64::from_str_radix(line, 10).expect("within range"),
    )(input)
}

fn elf_supplies(input: &str) -> IResult<&str, Elf> {
    map(
        many1(calorie_count), Elf::new,
    )(input)
}

fn puzzle_input(input: &str) -> IResult<&str, Vec<Elf>> {
    separated_list1(line_ending, elf_supplies)(input)
}


pub fn solve() {
    let input = crate::util::read_to_eof_line();
    println!("{}", &input);
    if let Ok(("", parsed_input)) = puzzle_input(&input) {
        if parsed_input.len() < 3 {
            println!("Too few elves");
        } else {
            let mut supplies = parsed_input.into_iter().map(|e|e.supplies).collect::<Vec<u64>>();
            supplies.sort();
            println!("Part 1: {}", supplies[supplies.len() - 1]);
            println!("Part 2: {}", supplies[supplies.len() - 3] + supplies[supplies.len() - 2] + supplies[supplies.len() - 1]);
        }
    }
}