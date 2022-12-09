use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use crate::util::linear2d::Direction;
use crate::util::{default_solution, parse_usize};

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Index2D(i32, i32);

impl Display for Index2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.0, self.1))
    }
}

impl Index2D {
    pub fn step(self, dir: Direction) -> Index2D {
        let Index2D(x, y) = self;
        match dir {
            Direction::NorthToSouth => Self(x, y - 1),
            Direction::WestToEast => Self(x + 1, y),
            Direction::SouthToNorth => Self(x, y + 1),
            Direction::EastToWest => Self(x - 1, y)
        }
    }

    pub fn max_distance(self, other: Index2D) -> usize {
        let Index2D(x1, y1) = self;
        let Index2D(x2, y2) = other;

        (x1 - x2).abs().max((y1 - y2).abs()) as usize
    }
}

#[derive(Debug)]
struct RopeKnot {
    head: Index2D,
    tail: Option<Box<RopeKnot>>,
}

impl Display for RopeKnot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.head.0, self.head.1))?;
        if let Some(tail) = &self.tail {
            f.write_str(" -> ")?;
            Display::fmt(tail, f)
        } else {
            Ok(())
        }
    }
}

impl RopeKnot {
    fn of_length(size: usize) -> RopeKnot {
        let head = Index2D(0, 0);
        let tail = if size == 0 {
            None
        } else {
            Some(Box::new(Self::of_length(size - 1)))
        };
        Self { head, tail }
    }

    fn step<TT>(&mut self, direction: Direction, tail_tracking: TT) where TT: FnMut(Index2D) {
        self.head = self.head.step(direction);
        self.reconnect(tail_tracking)
    }

    fn reconnect<TT>(&mut self, mut tail_tracking: TT) where TT: FnMut(Index2D) {
        if let Some(tail) = &mut self.tail {
            if self.head.max_distance(tail.head) > 1 {
                tail.head.0 += (self.head.0 - tail.head.0).signum();
                tail.head.1 += (self.head.1 - tail.head.1).signum();

                tail.reconnect(tail_tracking);
            }
        } else {
            tail_tracking(self.head)
        }
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    map(one_of("LRUD"), |ch| match ch {
        'L' => Direction::EastToWest,
        'R' => Direction::WestToEast,
        'U' => Direction::SouthToNorth,
        'D' => Direction::NorthToSouth,
        _ => unreachable!()
    })(input)
}

fn parse_move(input: &str) -> IResult<&str, Vec<Move>> {
    many1(
        terminated(
            map(separated_pair(parse_direction, space1, parse_usize), |(direction, steps)| Move { direction, steps }),
            line_ending))
        (input)
}

fn solve_problem(moves: Vec<Move>) -> (usize, usize) {
    let mut rope1 = RopeKnot::of_length(1);
    let mut rope2 = RopeKnot::of_length(9);
    let mut visited_1 = HashSet::new();
    let mut visited_2 = HashSet::new();
    visited_1.insert(rope1.head);
    visited_2.insert(rope2.head);

    for Move { direction, steps } in moves {
        for _ in 0..steps {
            rope1.step(direction, |idx| {
                visited_1.insert(idx);
            });
            rope2.step(direction, |idx|{
                visited_2.insert(idx);
            });
        }
    }


    (visited_1.len(), visited_2.len())
}

default_solution!(parse_move, solve_problem);