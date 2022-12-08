use std::cmp::max;
use nom::IResult;

use crate::util::{default_solution, linear2d::Direction, linear2d::Linear2DArray};

#[derive(Debug)]
struct Tree {
    blocked_direction_count: i32,
    scenic_score: i32,
    height: i32,
}

impl Tree {
    fn new(height: i32) -> Self {
        Self {
            height,
            blocked_direction_count: 0,
            scenic_score: 1,
        }
    }

    fn is_visible(&self) -> bool {
        self.blocked_direction_count < 4
    }
}


fn parse(input: &str) -> IResult<&str, Linear2DArray<Tree>> {
    let bytes = input.as_bytes();
    let width = bytes.iter().filter(|b| **b < b'0').count();
    let storage = bytes.into_iter().filter(|b| **b >= b'0' && **b <= b'9').map(|b|{
        let height = (*b - b'0') as i32;
        Tree::new(height)
    }).collect();

    IResult::Ok(("", Linear2DArray::new(storage, width)))
}

fn compute_solution(mut input: Linear2DArray<Tree>) -> (usize, i32) {

    #[derive(Debug, Default)]
    struct SweepState {
        highest: i32,
        visible_at_height: [i32; 10],
        visible: usize,
        best_scenic_score: i32
    }
    let mut state = SweepState {
        highest: 0,
        visible_at_height: [0; 10],
        visible: input.height * input.width,
        best_scenic_score: 0
    };
    for dir in Direction::ALL {
        state = input.sweep(state, dir, |state| {
            state.highest = -1;
            state.visible_at_height.fill(0);
            true
        }, |state, _idx, tree| {
            let height = tree.height;
            if state.highest >= height {
                tree.blocked_direction_count += 1;

                if !tree.is_visible() {
                    state.visible -= 1
                }
            } else {
                state.highest = height;
            }

            let height = height as usize;
            tree.scenic_score *= state.visible_at_height[height];
            state.best_scenic_score = max(tree.scenic_score, state.best_scenic_score);

            (&mut state.visible_at_height[0..=height]).fill(1);
            (&mut state.visible_at_height[(1 + height)..]).iter_mut().for_each(|value| *value += 1);

            true
        });
    }

    (state.visible, state.best_scenic_score)
}


default_solution!(parse, compute_solution);
