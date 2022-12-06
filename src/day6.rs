use std::io::stdin;
use std::time::Instant;

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

fn detect_distinct_range(whole: &[u8], window_size: usize, prefix: &'static str) {
    for i in 0..whole.len() - window_size {
        if detect_marker(&whole[i..i + window_size]) {
            println!("Part {}: {}", prefix, i + window_size);
            break;
        }
    }
}

pub fn solve() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Read line");
    let start = Instant::now();
    let bytes = buffer.as_bytes();
    detect_distinct_range(bytes, 4, "1");
    detect_distinct_range(bytes, 14, "2");

    println!("Solution (no parse): {:?}", Instant::now() - start);
}