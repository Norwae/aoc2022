use std::cmp::{max, min};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, terminated};
use crate::util::linear2d::{Direction, Index2D, Linear2DArray};
use crate::util::{default_solution, parse_usize};
use crate::util::linear2d::Direction::{EastToWest, NorthToSouth, SouthToNorth, WestToEast};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Rock,
    SettledSand,
}

const ORIGIN: Index2D = Index2D(500, 0);
const LINE_WIDTH: usize = 1000;
const SAFE_HEIGHT: usize = 200;

#[derive(Debug)]
struct Problem {
    state: Linear2DArray<Tile>,
    trace: Vec<Index2D>,
}

impl Problem {
    fn new() -> Self {
        Self {
            state: Linear2DArray::new(vec![Tile::Open; SAFE_HEIGHT * LINE_WIDTH], 1000),
            trace: vec![ORIGIN],
        }
    }

    fn can_drop_to(&self, idx: Index2D) -> bool {
        idx.1 < self.state.height && idx.0 < self.state.width && self.state[idx] == Tile::Open
    }

    fn drop_step(&self, idx: Index2D) -> Option<Index2D> {
        let straight_down = idx.step(NorthToSouth);
        let diagonal_right = straight_down.step(WestToEast);
        let diagonal_left = straight_down.step(EastToWest);
        if self.can_drop_to(straight_down) {
            Some(straight_down)
        } else if self.can_drop_to(diagonal_left) {
            Some(diagonal_left)
        } else if self.can_drop_to(diagonal_right) {
            Some(diagonal_right)
        } else {
            None
        }
    }

    fn drop_grain(&mut self) -> bool {
        loop {
            if let Some(last_floating) = self.trace.last() {
                if last_floating.1 == self.state.height - 1{
                    return false; // dropping out of bottom
                }

                if let Some(next_floating) = self.drop_step(*last_floating) {
                    self.trace.push(next_floating);
                } else {
                    self.state[*last_floating] = Tile::SettledSand;
                    self.trace.pop();
                    return true; // grain came to rest
                }
            } else {
                return false; // clogging the source
            }
        }
    }

    fn draw_line(&mut self, mut from: Index2D, mut to: Index2D, contents: Tile) {
        // drawing a new line invalidates pathfinding cache
        self.trace.truncate(1);

        let direction = if from.0 < to.0 {
            WestToEast
        } else if from.1 < to.1 {
            NorthToSouth
        } else if from.0 > to.0 {
            EastToWest
        } else {
            SouthToNorth
        };

        loop {
            self.state[from] = contents;

            if from == to {
                break;
            } else {
                from = from.step(direction);
            }
        }
    }
}

fn path(input: &str) -> IResult<&str, Vec<Index2D>> {
    separated_list1(
        tag(" -> "),
        map(
            separated_pair(parse_usize, tag(","), parse_usize),
            |(x, y)| Index2D(x, y),
        ),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Problem, usize)> {
    map(
        many1(terminated(path, line_ending)),
        |paths| {
            let mut problem = Problem::new();
            let mut floor_y = 0;
            for path in paths {
                for (from, to) in path.iter().tuple_windows() {
                    floor_y = max(floor_y, max(from.1, to.1) + 2);
                    problem.draw_line(*from, *to, Tile::Rock)
                }
            }

            (problem, floor_y)
        },
    )(input)
}

fn compute_solution(tpl: (Problem, usize)) -> (usize, usize) {
    let (mut problem, floor_y) = tpl;
    let mut dropped = 0;
    while problem.drop_grain() {
        dropped += 1;
    }

    let part1 = dropped;

    problem.draw_line(Index2D(0, floor_y), Index2D(LINE_WIDTH - 1, floor_y), Tile::Rock);
    while problem.drop_grain() {
        dropped += 1;
    }

    (part1, dropped)
}

default_solution!(parse, compute_solution);