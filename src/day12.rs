use std::sync::Arc;
use nom::IResult;
use pathfinding::prelude::{astar, dijkstra};

use crate::util::default_solution;
use crate::util::linear2d::{ALL, Index2D, Linear2DArray};
use crate::util::parallel::{block_on, in_parallel};

use smallvec::SmallVec;

const START: i32 = 0;
const END: i32 = 27;

#[derive(Debug, Clone, Copy)]
struct Cell {
    height: i32,
}

#[derive(Debug, Clone)]
struct Map {
    heights: Linear2DArray<Cell>,
    start: Index2D,
    end: Index2D,
}

fn insert_line(storage: &mut Vec<Cell>, start: &mut Index2D, end: &mut Index2D, line: &str, border_cell: Cell, y: usize) {
    storage.push(border_cell);
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

    storage.push(border_cell);
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
    insert_line(&mut storage, &mut start, &mut end, first_line, border_cell, 1);

    let mut y = 2usize;

    for line in lines {
        insert_line(&mut storage, &mut start, &mut end, line, border_cell, y);
        y += 1;
    }
    storage.append(&mut vec![border_cell.clone(); width]);

    let mut heights = Linear2DArray::new(storage, width);
    Ok(("", Map { heights, start, end }))
}

fn run_astar_algorithm(heights: &Linear2DArray<Cell>, start: Index2D, end: Index2D) -> usize
{
    let astar_result = astar(&start, |idx| {
        let mut scratch = SmallVec::<[(Index2D, usize); 4]>::new();

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

    astar_result.expect("A* found path").1
}

fn run_dijkstra_algorithm(heights: &Linear2DArray<Cell>, start: Index2D) -> usize {
    let dijkstra_result = dijkstra(&start, |idx| {
        let mut scratch = SmallVec::<[(Index2D, usize); 4]>::new();

        for d in ALL {
            let this_height = heights[*idx].height;
            let stepped = idx.step(d);
            let other_height = heights[stepped].height;
            if other_height >= this_height - 1 && other_height < i32::MAX {
                scratch.push((stepped, 1usize));
            }
        }
        scratch
    }, |idx| heights[*idx].height == 1);

    dijkstra_result.expect("Dikjstra found path").1
}

fn solve_problem(map: Map) -> (usize, usize) {
    let map1 = Arc::new(map);
    let map2 = map1.clone();
    let part1 = in_parallel(move || run_astar_algorithm(&map1.heights, map1.start, map1.end));
    let part2 = in_parallel(move || run_dijkstra_algorithm(&map2.heights, map2.end));

    block_on(async move {
        let part1 = part1.await.expect("success");
        let part2 = part2.await.expect("success");
        (part1, part2)
    })
}

default_solution!(parse_input, solve_problem);