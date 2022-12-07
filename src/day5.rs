use std::mem::swap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, line_ending, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use crate::util::{default_solution, parse_usize};


#[derive(Debug, Default, Clone)]
struct Column {
    stack: Vec<char>,
}

#[derive(Debug, Clone)]
struct Problem {
    columns: Vec<Column>,
    moves: Vec<Move>,
    legacy_mode: bool,
}

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Problem {
    fn run(mut self) -> String {
        let mut moves = Vec::new();
        swap(&mut moves, &mut self.moves);
        for m in moves {
            self.apply_move(m);
        }

        self.columns.iter().filter_map(|col| col.stack.last()).collect::<String>()
    }

    fn apply_move(&mut self, m: Move) {
        let Move { from, to, count } = m;
        let mut temp = Vec::new();
        swap(&mut self.columns[from].stack, &mut temp);
        let mid = temp.len() - count;
        let mve = &mut temp[mid..];
        if self.legacy_mode {
            mve.reverse();
        }

        self.columns[to].stack.extend_from_slice(mve);
        temp.truncate(mid);
        swap(&mut self.columns[from].stack, &mut temp);
    }
}


fn crates(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    let single_crate_spec = alt((
        map(tag("   "), |_| None),
        map(delimited(tag("["), anychar, tag("]")), |c|Some(c))
    ));
    let line = terminated(separated_list1(tag(" "), single_crate_spec), line_ending);
    let crate_section = many1(line);
    let end_of_crates = terminated(many1(alt((space1, digit1))), line_ending);

    terminated(crate_section, end_of_crates)(input)
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    let line = map(
        tuple((tag("move "), parse_usize, tag(" from "), parse_usize, tag(" to "), parse_usize, line_ending)),
        |(_, count, _, from, _, to, _)| {
            // fix 1-based indices
            let from = from - 1;
            let to = to - 1;
            Move { count, from, to }
        },
    );
    many1(line)(input)
}

fn columns(input: &str) -> IResult<&str, Vec<Column>> {
    map(
        crates,
        |double_vec| {
            let height = double_vec.len();
            let width = double_vec[0].len();
            let mut target = vec![Column::default(); width];

            for x in 0..width {
                for y in 0..height {
                    if let Some(ch) = double_vec[y][x] {
                        target[x].stack.push(ch)
                    }
                }

                target[x].stack.reverse()
            }

            target
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Problem> {
    map(separated_pair(columns, line_ending, moves), |(columns, moves)| Problem { columns, moves, legacy_mode: false })(input)
}

fn solve_problem(problem: Problem) {
    let mut part1 = problem.clone();
    part1.legacy_mode = true;
    println!("Part 1: {}", part1.run());
    println!("Part 2: {}", problem.run());
}


default_solution!(parse_input, solve_problem);