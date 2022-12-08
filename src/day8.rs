use std::cmp::max;
use nom::IResult;
use crate::util;

use crate::util::{default_solution, linear2d::Direction, linear2d::Linear2DArray};

#[derive(Debug, Clone)]
struct Tree {
    blocked: bool,
    sight_range: u32,
    height: u32,
}

impl Tree {
    fn new(height: u32) -> Self {
        Self {
            height,
            blocked: false,
            sight_range: 0,
        }
    }
}


fn parse(input: &str) -> IResult<&str, Linear2DArray<Tree>> {
    let bytes = input.as_bytes();
    let width = bytes.iter().filter(|b| **b < b'0').count();
    let storage = bytes.into_iter().filter(|b| **b >= b'0' && **b <= b'9').map(|b| {
        let height = (*b - b'0') as u32;
        Tree::new(height)
    }).collect();

    IResult::Ok(("", Linear2DArray::new(storage, width)))
}

#[derive(Debug, Default)]
struct SweepState {
    highest: i32,
    visible_at_height: [u32; 10],
}

fn compute_sweep(mut trees: Linear2DArray<Tree>, direction: Direction) -> Linear2DArray<Tree> {
    let mut state = SweepState::default();
    trees.sweep(&mut state, direction, |state| {
        state.highest = -1;
        state.visible_at_height.fill(0);
        true
    }, |state, _idx, tree| {
        let height = tree.height as i32;
        if state.highest >= height {
            tree.blocked = true;
        } else {
            state.highest = height;
        }

        let height = height as usize;
        tree.sight_range = state.visible_at_height[height];

        (&mut state.visible_at_height[0..=height]).fill(1);
        (&mut state.visible_at_height[(1 + height)..]).iter_mut().for_each(|value| *value += 1);

        true
    });

    trees
}

fn compute_solution(input: Linear2DArray<Tree>) -> (u32, u32) {
    let input1 = input.clone();
    let west = util::parallel::in_parallel(move ||
        compute_sweep(input1, Direction::WestToEast)
    );
    let input1 = input.clone();
    let north = util::parallel::in_parallel(move ||
        compute_sweep(input1, Direction::NorthToSouth)
    );

    let input1 = input.clone();
    let east = util::parallel::in_parallel(move ||
        compute_sweep(input1, Direction::EastToWest)
    );
    let south = util::parallel::in_parallel(move ||
        compute_sweep(input, Direction::SouthToNorth)
    );

    let finish = async move {
        if let (Ok(west), Ok(north), Ok(east), Ok(south)) = tokio::join!(west, north, east, south) {
            let mut total_visible = 0u32;
            let mut best_scenery_score = 0u32;
            for (((a, b), c), d) in west.iter().zip(north.iter()).zip(east.iter()).zip(south.iter()) {
                let visible = !(a.blocked && b.blocked && c.blocked && d.blocked);
                let scenery_score = a.sight_range * b.sight_range * c.sight_range * d.sight_range;

                if visible {
                    total_visible += 1
                }
                if scenery_score > best_scenery_score {
                    best_scenery_score = scenery_score
                }
            }

            (total_visible, best_scenery_score)
        } else {
            (0, 0)
        }
    };

    util::parallel::block_on(finish)
}


default_solution!(parse, compute_solution);
