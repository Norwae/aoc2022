use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use crate::util::{default_solution, parse_i64};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command{
    Nop,
    StartAddX,
    CompleteAddX(i64)
}

#[derive(Debug, Clone)]
enum Slotted<X> {
    One(X),
    Two(X, X)
}

fn nop(input: &str) -> IResult<&str, Slotted<Command>> {
    map(tag("noop"), |_|Slotted::One(Command::Nop))(input)
}

fn add_x(input: &str) -> IResult<&str, Slotted<Command>> {
    map(preceded(tag("addx "), parse_i64), |value| Slotted::Two(Command::StartAddX, Command::CompleteAddX(value)))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    map(
        many1(terminated(alt((
        nop, add_x
        )), line_ending)),
        |slots| {
            let mut target = Vec::with_capacity(2 * slots.len());
            for slot in slots {
                match slot {
                    Slotted::One(cmd) => target.push(cmd),
                    Slotted::Two(cmd1, cmd2) => {
                        target.push(cmd1);
                        target.push(cmd2);
                    }
                }
            }
            target
        })(input)
}

fn run(commands: Vec<Command>) -> (i64, String) {
    static PROBES: [usize;7] = [20, 60, 100, 140, 180, 220, usize::MAX];
    let mut probes: &[usize] = &PROBES;
    let mut clock = 1;
    let mut crt_col = 0;
    let mut x_register = 1i64;
    let mut crt_buffer = String::from('\n');

    let mut signal_strength_sum = 0;

    for command in commands {
        if clock == probes[0] {
            let signal_strength = x_register * probes[0] as i64;
            signal_strength_sum += signal_strength;
            probes = &probes[1..];
        }

        let crt_scan_char = if x_register >= crt_col - 1 && x_register <= crt_col + 1 {
            '#'
        } else {
            '.'
        };

        crt_buffer.push(crt_scan_char);

        if let Command::CompleteAddX(value) = command {
            x_register += value;
        }

        crt_col += 1;
        clock += 1;

        if crt_col == 40 {
            crt_buffer.push('\n');
            crt_col = 0;
        }

    }

    (signal_strength_sum, crt_buffer)
}

default_solution!(parse, run);