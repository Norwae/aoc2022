use nom::IResult;
use pathfinding::prelude::astar;

use crate::util::default_solution;
use crate::util::linear2d::{ALL, Index2D, Linear2DArray};

const START: i32 = 0;
const END: i32 = 27;

#[derive(Debug, Clone)]
struct Cell {
    height: i32,
    best_path_length: Option<usize>,
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
            best_path_length: None,
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
    let border_cell = Cell { height: i32::MAX, best_path_length: None };
    storage.append(&mut vec![border_cell.clone(); width]);
    insert_line(&mut storage, &mut start, &mut end, first_line, &border_cell, 1);

    let mut y = 2usize;

    for line in lines {
        insert_line(&mut storage, &mut start, &mut end, line, &border_cell, y);
        y += 1;
    }
    storage.append(&mut vec![border_cell.clone(); width]);

    let mut heights = Linear2DArray::new(storage, width);
    heights[end].best_path_length = Some(0);
    Ok(("", Map { heights, start, end }))
}

fn run_astar_algorithm(heights: &Linear2DArray<Cell>, start: Index2D, end: Index2D) -> Option<Vec<Index2D>>
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
                             |idx| heights[*idx].best_path_length.is_some());

    astar_result.map(|tpl| tpl.0)
}

fn include_into_map(map: &mut Map, path: Vec<Index2D>) {
    let terminating_index = *path.last().expect("terminating into known length");
    let mut terminating_known_length = map.heights[terminating_index].best_path_length.expect("terminating into known length");

    for index in path.iter().rev().skip(1) {
        terminating_known_length += 1;
        map.heights[*index].best_path_length = Some(terminating_known_length)
    }
}

fn solve_problem(mut map: Map) -> (usize, usize) {
    let path1 = run_astar_algorithm(&map.heights, map.start, map.end).expect("End reachable from start");
    include_into_map(&mut map, path1);
    let part1 = map.heights[map.start].best_path_length.expect("reached start");

    let mut best = part1;
    for y in 1..map.heights.height - 1 {
        for x in 1..map.heights.width - 1 {
            let idx = Index2D(x, y);
            if map.heights[idx].height == 1 && best > map.end.max_distance(idx) {
                if let Some(path_from_here) = run_astar_algorithm(&map.heights, idx, map.end) {
                    include_into_map(&mut map, path_from_here);
                    let length_from_here = map.heights[idx].best_path_length.expect("alternative found");
                    if length_from_here < best {
                        best = length_from_here
                    }
                }
            }
        }
    }

    (part1, best)
}

default_solution!(parse_input, solve_problem);