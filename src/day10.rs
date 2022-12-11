use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, terminated};
use crate::util::{default_solution, parse_i64};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command{
    Nop,
    AddX(i64)
}

fn nop(input: &str) -> IResult<&str, Command> {
    map(tag("noop"), |_|Command::Nop)(input)
}

fn add_x(input: &str) -> IResult<&str, Command> {
    map(preceded(tag("addx "), parse_i64), |value| Command::AddX(value))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    many1(terminated(alt((
        nop, add_x
        )), line_ending))(input)
}

fn run(commands: Vec<Command>) -> (i64, i64) {
    static PROBES: [usize;7] = [20, 60, 100, 140, 180, 220, usize::MAX];
    let mut probes: &[usize] = &PROBES;
    let mut clock = 0;
    let mut x_register = 1i64;

    let mut signal_strength_sum = 0;

    for cmd in commands {
        let (delta_x, delta_time) = match cmd {
            Command::AddX(value) => (value, 2),
            _ => (0, 1)
        };

        if clock + delta_time >= probes[0] {
            let signal_strength = x_register * probes[0] as i64;
            signal_strength_sum += signal_strength;
            probes = &probes[1..];
        }

        clock += delta_time;
        x_register += delta_x;
    }

    (signal_strength_sum, 0)
}

default_solution!(parse, run);