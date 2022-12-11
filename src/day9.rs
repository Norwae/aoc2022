use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};

use crate::util::{default_solution, parse_usize};
use crate::util::linear2d::Direction;

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

struct Rope {
    knots: [Index2D; 10],
    last_tails: HashSet<Index2D>,
    first_tails: HashSet<Index2D>
}

impl Rope {
    fn new() -> Self {
        let origin = Index2D(0, 0);
        let mut tails = HashSet::new();
        tails.insert(origin);
        Self {
            knots: [origin; 10],
            first_tails: tails.clone(),
            last_tails: tails
        }
    }

    fn step(&mut self, direction: Direction) {
        self.knots[0] = self.knots[0].step(direction);

        for idx in 1..10 {
            if !Self::reconnect(self.knots[idx - 1], &mut self.knots[idx]) {
                break;
            }
        }


        self.last_tails.insert(self.knots[9]);
        self.first_tails.insert(self.knots[1]);
    }

    fn first_tail_locations(&self) -> &HashSet<Index2D> {
        &self.first_tails
    }

    fn last_tail_locations(&self) -> &HashSet<Index2D> {
        &self.last_tails
    }

    fn reconnect(head: Index2D, tail: &mut Index2D) -> bool {
        if head.max_distance(*tail) > 1 {
            tail.0 += (head.0 - tail.0).signum();
            tail.1 += (head.1 - tail.1).signum();
            true
        } else {
            false
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
    let mut rope = Rope::new();

    for Move { direction, steps } in moves {
        for _ in 0..steps {
            rope.step(direction);
        }
    }


    (rope.first_tail_locations().len(), rope.last_tail_locations().len())
}

default_solution!(parse_move, solve_problem);