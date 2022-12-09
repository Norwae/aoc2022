use std::io::stdin;
use nom::character::complete::{digit1};
use nom::combinator::map;
use nom::IResult;

pub mod linear2d;
pub mod parallel;

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |str: &str| str.parse::<usize>().expect("digits -> usize"))(input)
}

pub fn parse_identity(input: &str) -> IResult<&str, &str> {
    Ok(("", input))
}

pub fn read_to_eof_line() -> String {
    let mut accu = String::new();

    loop {
        let line_length = stdin().read_line(&mut accu).expect("IO error");

        if line_length == 4 && accu.ends_with("EOF\n") {
            accu.truncate(accu.len() - 4);
            return accu;
        }

        if line_length == 5 && accu.ends_with("EOF\r\n") {
            accu.truncate(accu.len() - 5);
            return accu;
        }
    }
}

pub fn read_usize(prompt: &str) -> usize {
    let mut buf = String::new();

    loop {
        println!("{}", prompt);
        buf.clear();
        stdin().read_line(&mut buf).expect("IO error");
        let parsed = buf.trim().parse().ok();

        if let Some(value) = parsed {
            return value;
        } else {
            println!("Invalid input: {}", buf);
        }
    }
}

macro_rules! default_solution {
    ($parse:ident, $solve:ident) => {
pub fn benchmark_full(input: &str) {
       ($solve)(($parse)(input).expect("Example okay").1);
}

pub fn benchmark_parse(input: &str){
    $parse(input).expect("Example ok");
}

pub fn solve() {
    let input = crate::util::read_to_eof_line();
    use std::time::Instant;

    let start = Instant::now();
    let parsed = $parse(&input);

    if let Ok(("", input)) = parsed {
        let (part1, part2) = $solve(input);
        println!("Part 1: {}\nPart 2: {}\nTook: {:?}", part1, part2, Instant::now() - start);
    } else {
        println!("Could not parse fully: {:?}", parsed)
    }
}
    };
}

pub(crate) use default_solution;

pub fn day_not_solved() {
    println!("Day not solved yet")
}
