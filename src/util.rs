use std::io::stdin;
use nom::character::complete::{digit1, one_of};
use nom::combinator::map;
use nom::IResult;

pub mod linear2d {
    use std::ops::{Index, IndexMut};

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub enum Direction {
        NorthToSouth,
        WestToEast,
        SouthToNorth,
        EastToWest,
    }

    impl Direction {
        pub const ALL: [Direction; 4] = [Direction::NorthToSouth, Direction::WestToEast, Direction::SouthToNorth, Direction::EastToWest];
    }

    #[derive(Debug)]
    pub struct Linear2DArray<T> {
        storage: Vec<T>,
        width: usize,
        height: usize,
    }

    impl<T> Linear2DArray<T> {
        pub fn new(storage: Vec<T>, width: usize) -> Self {
            let height = storage.len() / width;
            assert_eq!(storage.len() % width, 0);
            Self { storage, width, height }
        }

        pub fn dimensions(&self) -> (usize, usize) {
            (self.width, self.height)
        }

        pub fn iter(&self) -> impl Iterator<Item=&T> {
            self.storage.iter()
        }

        pub fn sweep_by<IndexMutator1, IndexMutator2, LineInit, Element>(&mut self, mut index: (usize, usize), index_increment: IndexMutator1, line_increment: IndexMutator2, mut line_init: LineInit, mut element: Element)
            where IndexMutator1: Fn(&mut (usize, usize)),
                  IndexMutator2: Fn(&mut (usize, usize)),
                  LineInit: FnMut() -> bool,
                  Element: FnMut(&mut T) -> bool
        {
            while index.0 < self.width && index.1 < self.height {
                if !line_init() {
                    break;
                }
                while index.0 < self.width && index.1 < self.height {
                    if !element(&mut self[index]) {
                        break;
                    }
                    index_increment(&mut index)
                }

                line_increment(&mut index)
            }
        }

        pub fn sweep<LineInit, Element>(&mut self, dir: Direction, line_init: LineInit, element: Element)
            where LineInit: FnMut() -> bool,
                  Element: FnMut(&mut T) -> bool {
            let height = self.height;
            let width = self.width;
            match dir {
                Direction::NorthToSouth => self.sweep_by(
                    (0, 0),
                    |idx| idx.1 += 1,
                    |idx| *idx = (idx.0 + 1, 0),
                    line_init,
                    element,
                ),
                Direction::WestToEast => self.sweep_by(
                    (0, 0),
                    |idx| idx.0 += 1,
                    |idx| *idx = (0, idx.1 + 1),
                    line_init,
                    element,
                ),
                Direction::SouthToNorth => self.sweep_by(
                    (0, self.height - 1),
                    |idx| idx.1 = idx.1.wrapping_sub(1),
                    |idx| *idx = (idx.0 + 1, height - 1),
                    line_init,
                    element,
                ),
                Direction::EastToWest => self.sweep_by(
                    (self.width - 1, 0),
                    |idx| idx.0 = idx.0.wrapping_sub(1),
                    |idx| *idx = (width - 1, idx.1 + 1),
                    line_init,
                    element,
                )
            }
        }
    }

    impl<T> Index<(usize, usize)> for Linear2DArray<T> {
        type Output = T;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            let (x, y) = index;
            assert!(x < self.width);
            assert!(y < self.height);
            &self.storage[x + y * self.width]
        }
    }

    impl<T> IndexMut<(usize, usize)> for Linear2DArray<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            let (x, y) = index;
            assert!(x < self.width);
            assert!(y < self.height);
            &mut self.storage[x + y * self.width]
        }
    }
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |str: &str| str.parse::<usize>().expect("digits -> usize"))(input)
}

pub fn parse_single_digit(input: &str) -> IResult<&str, i32> {
    map(one_of("0123456789"), |charcode| charcode as i32 - '0' as i32)(input)
}

pub fn parse_identity(input: &str) -> IResult<&str, &str> {
    Ok(("", input))
}

pub fn read_to_eof_line() -> String {
    let mut accu = String::new();

    loop {
        let line_length = stdin().read_line(&mut accu).expect("IO error");

        if line_length == 4 && accu.ends_with("EOF\n") {
            accu.truncate(accu.len() - 4);
            return accu;
        }

        if line_length == 5 && accu.ends_with("EOF\r\n") {
            accu.truncate(accu.len() - 5);
            return accu;
        }
    }
}

pub fn read_usize(prompt: &str) -> usize {
    let mut buf = String::new();

    loop {
        println!("{}", prompt);
        buf.clear();
        stdin().read_line(&mut buf).expect("IO error");
        let parsed = buf.trim().parse().ok();

        if let Some(value) = parsed {
            return value;
        } else {
            println!("Invalid input: {}", buf);
        }
    }
}

macro_rules! default_solution {
    ($parse:ident, $solve:ident) => {
pub fn solve() {
    use std::time::Instant;
    let input = crate::util::read_to_eof_line();
    let start_parse = Instant::now();
    let parsed = $parse(&input);

    if let Ok(("", input)) = parsed {
        let start_solve = Instant::now();
        $solve(input);
        let end = Instant::now();
        println!("Solving duration (including parse): {:?} ({:?})", end - start_solve, end - start_parse)
    } else {
        println!("Could not parse fully: {:?}", parsed)
    }
}
    };
}

pub(crate) use default_solution;

pub fn day_not_solved() {
    println!("Day not solved yet")
}
