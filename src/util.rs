use std::io::stdin;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |str: &str| str.parse::<usize>().expect("digits -> usize"))(input)
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
pub fn solve() {
    use std::time::Instant;
    let input = crate::util::read_to_eof_line();
    let start_parse = Instant::now();
    let parsed = $parse(&input);

    if let Ok(("", input)) = parsed {
        let start_solve = Instant::now();
        $solve(input);
        let end = Instant::now();
        println!("Solving duration (including parse): {:?} ({:?})", end - start_solve, end - start_parse)
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
