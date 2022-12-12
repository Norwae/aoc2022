use std::borrow::{Borrow, Cow};
use std::ops::Index;

use nom::IResult;
use pathfinding::prelude::astar;

use crate::util::default_solution;
use crate::util::linear2d::{ALL, Direction, Index2D, Linear2DArray};

const START: i32 = 0;
const END: i32 = 27;

#[derive(Debug, Clone)]
struct Cell {
    height: i32,
}

#[derive(Debug, Clone)]
struct Map {
    heights: Linear2DArray<Cell>,
    start: Index2D,
    end: Index2D,
}

fn insert_line(storage: &mut Vec<Cell>, start: &mut Index2D, end: &mut Index2D, line: &str, border_cell: &Cell, y: usize) {
    storage.push(border_cell.clone());
    let slice = line.as_bytes();
    for x in 1..=slice.len() {
        let b = slice[x - 1];
        let height = if b == b'S' {
            *start = Index2D(x, y);
            START
        } else if b == b'E' {
            *end = Index2D(x, y);
            END
        } else {
            1 + (b - b'a') as i32
        };

        let cell = Cell {
            height,
        };
        storage.push(cell)
    }

    storage.push(border_cell.clone());
}

fn parse_input(input: &str) -> IResult<&str, Map> {
    let mut storage = Vec::new();
    let mut start = Index2D(0, 0);
    let mut end = Index2D(0, 0);

    let mut lines = input.lines();

    let first_line = lines.next().expect("Nonempty input");
    let width = first_line.len() + 2;
    let border_cell = Cell { height: i32::MAX };
    storage.append(&mut vec![border_cell.clone(); width]);
    insert_line(&mut storage, &mut start, &mut end, first_line, &border_cell, 1);

    let mut y = 2usize;

    for line in lines {
        insert_line(&mut storage, &mut start, &mut end, line, &border_cell, y);
        y += 1;
    }
    storage.append(&mut vec![border_cell.clone(); width]);

    let heights = Linear2DArray::new(storage, width);
    Ok(("", Map { heights, start, end }))
}

fn run_astar_algorithm<Res, Post: FnOnce((Vec<Index2D>, usize))-> Res> (heights: &Linear2DArray<Cell>, start: Index2D, end: Index2D, post: Post) -> Option<Res>
{
    let astar_result = astar(&start,
          |idx| {
              let mut scratch = Vec::with_capacity(4);


              for d in ALL {
                  let this_height = heights[*idx].height;
                  let stepped = idx.step(d);
                  let other_height = heights[stepped].height;
                  if other_height <= this_height + 1 {
                      scratch.push((stepped, 1usize));
                  }
              }
              scratch
          },
          |idx| idx.max_distance(end),
          |idx| *idx == end);
    astar_result.map(post)
}


fn last_a_in_path(heights: &Linear2DArray<Cell>, path: Vec<Index2D>) -> usize {
    for idx in (0..path.len()).rev() {
        if heights[path[idx]].height == 1 {
            return path.len() - idx
        }
    }

    unreachable!()
}

fn solve_problem(map: Map) -> (usize, usize) {
    let path1 = run_astar_algorithm(&map.heights, map.start, map.end, |tpl|tpl.0).expect("End reachable from start");
    let part1 = path1.len() - 1;

    let mut best = last_a_in_path(&map.heights, path1);

    for y in 1..map.heights.height - 1 {
        for x in 1..map.heights.width - 1 {
            let idx = Index2D(x, y);
            if map.heights[idx].height == 1 && best > map.end.max_distance(idx)  {
                let from_here = run_astar_algorithm(&map.heights, idx, map.end, |tpl|tpl.1).unwrap_or(usize::MAX);
                if from_here < best {
                    best = from_here
                }
            }
        }
    }

    (part1, best)
}

default_solution!(parse_input, solve_problem);