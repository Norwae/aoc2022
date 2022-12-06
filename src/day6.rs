use std::io::stdin;
use std::time::Instant;

use nom::IResult;

fn detect_marker(slice: &[u8]) -> bool {
    let mut seen = [false; 127];
    for byte in slice {
        if seen[*byte as usize] {
            return false
        } else {
            seen[*byte as usize] = true
        }
    }

    true
}


pub fn solve() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Read line");
    let start = Instant::now();
    let bytes = buffer.as_bytes();
    for i in 0..bytes.len() - 4 {
        if detect_marker(&bytes[i..i+4]) {
            println!("Part1: {}", i + 4);
            break;
        }
    }
    for i in 0..bytes.len() - 14 {
        if detect_marker(&bytes[i..i+14]) {
            println!("Part2: {}", i + 14);
            break;
        }
    }

    println!("Solution (no parse): {:?}", Instant::now() - start);
}