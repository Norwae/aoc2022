use std::borrow::BorrowMut;
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
    blocked_north: bool,
    blocked_east: bool,
    blocked_south: bool,
    blocked_west: bool,
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
            blocked_north: false,
            blocked_east: false,
            blocked_south: false,
            blocked_west: false,
            viewing_distance_north: -1,
            viewing_distance_east: -1,
            viewing_distance_south: -1,
            viewing_distance_west: -1,
        }
    }

    fn viewing_distance_field(&mut self, dir: Direction) -> &mut i32 {
        match dir {
            Direction::NorthToSouth => &mut self.viewing_distance_north,
            Direction::WestToEast => &mut self.viewing_distance_west,
            Direction::SouthToNorth => &mut self.viewing_distance_south,
            Direction::EastToWest => &mut self.viewing_distance_east
        }
    }

    fn blocker_field(&mut self, dir: Direction) -> &mut bool {
        match dir {
            Direction::NorthToSouth => &mut self.blocked_north,
            Direction::WestToEast => &mut self.blocked_west,
            Direction::SouthToNorth => &mut self.blocked_south,
            Direction::EastToWest => &mut self.blocked_east
        }
    }

    fn scenic_score(&self) -> i32 {
        self.viewing_distance_north * self.viewing_distance_east * self.viewing_distance_south * self.viewing_distance_west
    }

    fn is_visible(&self) -> bool {
        !(self.blocked_south && self.blocked_north && self.blocked_east && self.blocked_west)
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
    #[derive(Debug, Default)]
    struct SweepState {
        highest: i32,
        visible_at_height: [i32; 10]
    }

    for dir in Direction::ALL {
        input.sweep(SweepState::default(), dir, |state| {
            state.highest = -1;
            state.visible_at_height.fill(0);
            true
        }, |state, idx, tree| {
            let height = tree.height;
            if state.highest >= height {
                let blocked = tree.blocker_field(dir);
                *blocked = true
            } else {
                state.highest = height
            }

            let height = height as usize;
            *tree.viewing_distance_field(dir) = state.visible_at_height[height];
            (&mut state.visible_at_height[0..=height]).fill(1);
            (&mut state.visible_at_height[(1 + height)..]).iter_mut().for_each(|value| *value += 1);

            true
        });
    }

    println!("Part 1: {}", input.iter().filter(|t| t.is_visible()).count());
    println!("Part 2: {}", input.iter().map(|t|t.scenic_score()).max().unwrap());
}

default_solution!(parse, solve_problem);
