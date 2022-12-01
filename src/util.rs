use std::io::stdin;


pub fn read_to_eof_line() -> String {
    let mut accu = String::new();
    let mut buf = String::new();

    loop {
        buf.clear();
        stdin().read_line(&mut buf).expect("IO error");
        if buf == "EOF\n" || buf == "EOF\r\n" {
            break;
        } else {
            accu.push_str(&buf)
        }
    }

    accu
}

pub fn read_usize(prompt: &str) -> usize {
    let mut buf = String::new();
    let mut parsed: Option<usize> = None;

    loop {
        println!("{}", prompt);
        buf.clear();
        stdin().read_line(&mut buf).expect("IO error");
        parsed = buf.trim().parse().ok();

        if let Some(value) = parsed {
            return value;
        } else {
            println!("Invalid input: {}", buf);
        }
    }
}


pub fn day_not_solved() {
    println!("Day not solved yet")
}
