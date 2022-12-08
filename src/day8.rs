use std::cell::RefCell;
use nom::branch::alt;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, peek};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::tuple;
use crate::util::{default_solution, parse_single_digit, linear2d::Linear2DArray, linear2d::Direction};
use crate::util::linear2d::Index2D;

#[derive(Debug)]
struct Tree {
    blocker_north_height: i32,
    blocker_east_height: i32,
    blocker_south_height: i32,
    blocker_west_height: i32,
    viewing_distance_north: i32,
    viewing_distance_east: i32,
    viewing_distance_south: i32,
    viewing_distance_west: i32,
    height: i32,
}

impl Tree {
    fn new(height: i32) -> Self {
        Self {
            height,
            blocker_north_height: -1,
            blocker_east_height: -1,
            blocker_south_height: -1,
            blocker_west_height: -1,
            viewing_distance_north: -1,
            viewing_distance_east: -1,
            viewing_distance_south: -1,
            viewing_distance_west: -1,
        }
    }

    fn blocker_field(&mut self, dir: Direction) -> &mut i32 {
        match dir {
            Direction::NorthToSouth => &mut self.blocker_north_height,
            Direction::WestToEast => &mut self.blocker_west_height,
            Direction::SouthToNorth => &mut self.blocker_south_height,
            Direction::EastToWest => &mut self.blocker_east_height
        }
    }

    fn scenic_score(&self) -> i32 {
        self.viewing_distance_north * self.viewing_distance_east * self.viewing_distance_south * self.viewing_distance_west
    }

    fn is_visible(&self) -> bool {
        self.blocker_north_height == -1 || self.blocker_east_height == -1 || self.blocker_south_height == -1 || self.blocker_west_height == -1
    }
}


fn parse(input: &str) -> IResult<&str, Linear2DArray<Tree>> {
    map(tuple((
        peek(digit1),
        many1(alt((
            map(line_ending, |_| Vec::new()),
            many1(parse_single_digit),
        )))
    )), |(first_line, lines)| {
        let width = first_line.len();
        let storage = lines.into_iter().flatten().map(|height| Tree::new(height)).collect();

        Linear2DArray::new(storage, width)
    })(input)
}

fn solve_problem(mut input: Linear2DArray<Tree>) {
    let max_height = RefCell::new(-2i32);
    for dir in Direction::ALL {
        input.sweep(dir, || {
            *max_height.borrow_mut() = -1;
            true
        }, |idx, tree| {
            let height = tree.height;
            let max_so_far = *max_height.borrow();
            let field = tree.blocker_field(dir);
            if max_so_far >= height {
                *field = max_so_far
            } else {
                *max_height.borrow_mut() = height;
            }
            true
        });
    }

    println!("Part 1: {}", input.iter().filter(|t| t.is_visible()).count())
}

default_solution!(parse, solve_problem);
