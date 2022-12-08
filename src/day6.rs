use crate::util::{default_solution, parse_identity};

fn detect_marker(slice: &[u8]) -> bool {
    let mut seen = [false; 127];
    for byte in slice {
        if seen[*byte as usize] {
            return false;
        } else {
            seen[*byte as usize] = true
        }
    }

    true
}

fn detect_distinct_range(whole: &[u8], window_size: usize) -> usize {
    for i in 0..whole.len() - window_size {
        if detect_marker(&whole[i..i + window_size]) {
            return i + window_size;
        }
    }

    usize::MAX
}

fn solve_problem(input: &str) -> (usize, usize){
    let bytes = input.as_bytes();
    (detect_distinct_range(bytes, 4),
     detect_distinct_range(bytes, 14))
}

default_solution!(parse_identity, solve_problem);