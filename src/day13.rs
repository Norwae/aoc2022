use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};

use nom::IResult;

use crate::util::{default_solution, parse_usize};

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum EitherScalarOrList {
    Scalar(usize),
    List(Vec<EitherScalarOrList>),
}

impl Display for EitherScalarOrList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EitherScalarOrList::Scalar(v) => f.write_fmt(format_args!("{}", v)),
            EitherScalarOrList::List(vals) => {
                let mut sep = "";
                f.write_char('[')?;
                for v in vals {
                    f.write_str(sep)?;
                    sep = ",";
                    Display::fmt(v, f)?;
                }
                f.write_char(']')
            }
        }
    }
}

impl EitherScalarOrList {
    fn evaluate_less_than_slice(mut left: &[Self], mut right: &[Self]) -> Option<Ordering>{
        loop {
            if left.is_empty() {
                return if !right.is_empty() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            } else if !left.is_empty() && right.is_empty() {
                return Some(Ordering::Greater);
            }
            let x1 = &left[0];
            let x2 = &right[0];

            let ord_first = x1.partial_cmp(x2);
            if let Some(choice) = ord_first {
                if choice != Ordering::Equal {
                    return Some(choice)
                }
            }
            left = &left[1..];
            right = &right[1..];
        }
    }
}

impl PartialOrd for EitherScalarOrList {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self, rhs) {
            (EitherScalarOrList::Scalar(l), EitherScalarOrList::Scalar(r)) => l.partial_cmp(r),
            (EitherScalarOrList::List(l), EitherScalarOrList::List(r)) => Self::evaluate_less_than_slice(l, r.as_slice()),
            (EitherScalarOrList::Scalar(v), EitherScalarOrList::List(r)) => {
                let temp = [EitherScalarOrList::Scalar(*v)];
                Self::evaluate_less_than_slice(&temp[..], r.as_slice())
            }
            (EitherScalarOrList::List(l), EitherScalarOrList::Scalar(v)) => {
                let temp = [EitherScalarOrList::Scalar(*v)];
                Self::evaluate_less_than_slice(l.as_slice(), &temp[..])
            }
        }
    }
}

fn parse_single(rest: &str) -> (&str,EitherScalarOrList) {
    if rest.starts_with('[') {
        let (rest, contents) = parse_list_contents(rest);
        (rest, EitherScalarOrList::List(contents))
    } else {
        let (rest, nr) = parse_usize(rest).expect("usize");
        (rest, EitherScalarOrList::Scalar(nr))
    }
}

fn parse_list_contents(mut rest: &str) -> (&str, Vec<EitherScalarOrList>) {
    let mut accu = Vec::new();
    if rest.starts_with("[]") {
        (&rest[2..], accu)
    } else {
        accu = Vec::with_capacity(10);
        while !rest.starts_with(']') {
            let (rest2, next) = parse_single(&rest[1..]);
            accu.push(next);
            rest = rest2
        }

        (&rest[1..], accu)
    }
}


fn parse(input: &str) -> IResult<&str, Vec<EitherScalarOrList>> {
    Ok(("", input.lines().filter(|s|!s.is_empty()).map(|line|parse_single(line).1).collect()))
}

fn solve_input(mut input: Vec<EitherScalarOrList>) -> (usize, usize) {
    let mut sum = 0;
    for i in (0..input.len()).step_by(2) {
        if input[i] < input[i + 1] {
            sum += 1 + i / 2;
        }
    }

    let divider1 = EitherScalarOrList::List(vec![EitherScalarOrList::List(vec![EitherScalarOrList::Scalar(2)])]);
    let divider2 = EitherScalarOrList::List(vec![EitherScalarOrList::List(vec![EitherScalarOrList::Scalar(6)])]);
    input.push(divider1.clone());
    input.push(divider2.clone());
    input.sort();

    let index1 = 1 + input.iter().position(|pack| *pack == divider1 || *pack == divider2).unwrap();
    let index2 = 1 + input.iter().rposition(|pack|*pack == divider1 || *pack == divider2).unwrap();

    (sum, index1 * index2)
}

default_solution!(parse, solve_input);