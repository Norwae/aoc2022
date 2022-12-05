use std::mem::swap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, line_ending, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use crate::util::{default_solution, parse_usize};

enum CrateSpec {
    Space,
    Named(char),
}

#[derive(Debug, Default, Clone)]
struct Column {
    stack: Vec<char>,
}

#[derive(Debug, Clone)]
struct Problem {
    columns: Vec<Column>,
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Problem {
    fn run(&mut self) {
        let mut moves = Vec::new();
        swap(&mut moves, &mut self.moves);
        for m in moves {
            self.apply_move(m);
        }
    }

    fn apply_move(&mut self, m: Move) {
        let Move { from, to, count } = m;
        let mut source_stack = Vec::new();
        swap(&mut self.columns[from].stack, &mut source_stack);
        let mid = source_stack.len() - count;
        let (retain, mve) = source_stack.split_at_mut(mid);
        // disable for part 2
        // mve.reverse();

        self.columns[from].stack.extend_from_slice(retain);
        self.columns[to].stack.extend_from_slice(mve);
    }

    fn step(&mut self) {
        let x = self.moves.remove(0);
        self.apply_move(x);
    }
}


fn crates(input: &str) -> IResult<&str, Vec<Vec<CrateSpec>>> {
    let single_crate_spec = alt((
        map(tag("   "), |_| CrateSpec::Space),
        map(delimited(tag("["), anychar, tag("]")), |ch| CrateSpec::Named(ch))
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
                    if let CrateSpec::Named(ch) = double_vec[y][x] {
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
    map(separated_pair(columns, line_ending, moves), |(columns, moves)| Problem { columns, moves })(input)
}


pub fn solve() {
    default_solution(parse_input, |mut problem| {
        problem.run();
        let solution = problem.columns.iter().filter_map(|col| col.stack.last()).collect::<String>();
        println!("Part 2: {}", solution)
    })
}