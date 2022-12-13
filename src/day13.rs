use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};

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
    fn evaluate_less_than_slice(left: &[Self], right: &[Self]) -> Option<Ordering>{
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

        Self::evaluate_less_than_slice(&left[1..], &right[1..])
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

#[derive(Debug)]
struct PacketPair(EitherScalarOrList, EitherScalarOrList);


fn scalar(input: &str) -> IResult<&str, EitherScalarOrList> {
    map(parse_usize, |value| EitherScalarOrList::Scalar(value))(input)
}

fn list(input: &str) -> IResult<&str, EitherScalarOrList> {
    map(
        delimited(
            tag("["),
            separated_list0(tag(","), either_scalar_or_list),
            tag("]"),
        ),
        |vec| EitherScalarOrList::List(vec),
    )(input)
}

fn either_scalar_or_list(input: &str) -> IResult<&str, EitherScalarOrList> {
    alt((scalar, list))(input)
}

fn packet(input: &str) -> IResult<&str, PacketPair> {
    map(separated_pair(list, line_ending, list), |(first, second)| PacketPair(first, second))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<PacketPair>> {
    terminated(separated_list1(tuple((line_ending, line_ending)), packet),line_ending)(input)
}

fn solve_input(input: Vec<PacketPair>) -> (usize, usize) {
    let mut sum = 0;
    for i in 0..input.len() {
        if input[i].0 < input[i].1 {
            sum += 1 + i;
        }
    }

    let divider1 = EitherScalarOrList::List(vec![EitherScalarOrList::List(vec![EitherScalarOrList::Scalar(2)])]);
    let divider2 = EitherScalarOrList::List(vec![EitherScalarOrList::List(vec![EitherScalarOrList::Scalar(6)])]);
    let mut input = input.into_iter().flat_map(|p|[p.0, p.1]).collect::<Vec<_>>();
    input.push(divider1.clone());
    input.push(divider2.clone());
    input.sort();

    let index1 = 1 + input.iter().position(|pack| *pack == divider1 || *pack == divider2).unwrap();
    let index2 = 1 + input.iter().rposition(|pack|*pack == divider1 || *pack == divider2).unwrap();

    (sum, index1 * index2)
}

default_solution!(parse, solve_input);