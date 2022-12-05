use std::fmt::Debug;
use std::io::stdin;
use nom::IResult;


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

pub fn default_solution<T, Parse, Solve>(parse: Parse, solve: Solve) where
    T: Debug,
    Parse: FnOnce(&str) -> IResult<&str, T>,
    Solve: FnOnce(T) {
    let input = read_to_eof_line();
    let parsed = parse(&input);

    if let Ok(("", input)) = parsed {
        solve(input)
    } else {
        println!("Could not parse fully: {:?}", parsed)
    }
}


pub fn day_not_solved() {
    println!("Day not solved yet")
}
