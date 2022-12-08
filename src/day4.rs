use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use crate::util::default_solution;

#[derive(Debug)]
struct Assignment {
    high: usize,
    low: usize,
}

impl Assignment {
    fn full_overlap(&self, other: &Assignment) -> bool {
        (self.low <= other.low && self.high >= other.high) ||
            (other.low <= self.low && other.high >= self.high)
    }

    fn partial_overlap(&self, other: &Assignment) -> bool {
        (self.low <= other.high && self.high >= other.low) ||
            (other.low <= self.high && other.high >= self.low)
    }
}

fn section(input: &str) -> IResult<&str, usize> {
    map(digit1, |str: &str| str.parse::<usize>().expect("digits -> usize"))(input)
}

fn assignment(input: &str) -> IResult<&str, Assignment> {
    map(
        separated_pair(section, tag("-"), section),
        |(low, high)| Assignment { high, low },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(Assignment, Assignment)>> {
    let pair = separated_pair(assignment, tag(","), assignment);

    many1(terminated(pair, line_ending))(input)
}

fn solve_problem(lines: Vec<(Assignment, Assignment)>) -> (usize, usize) {
    let part1 = lines.iter().filter(|(a1, a2)| a1.full_overlap(a2)).count();

    let part2 = lines.iter().filter(|(a1, a2)| a1.partial_overlap(a2)).count();

    (part1, part2)
}


default_solution!(parse, solve_problem);