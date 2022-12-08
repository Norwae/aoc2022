extern crate core;

use crate::util::day_not_solved;

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

static SOLVERS: [fn() -> (); 24] = [
    day1::solve, day2::solve, day3::solve, day4::solve,
    day5::solve, day6::solve, day7::solve, day8::solve,
    day_not_solved, day_not_solved, day_not_solved, day_not_solved,
    day_not_solved, day_not_solved, day_not_solved, day_not_solved,
    day_not_solved, day_not_solved, day_not_solved, day_not_solved,
    day_not_solved, day_not_solved, day_not_solved, day_not_solved,
];


fn main() {
    let day = util::read_usize("Day to solve:");
    if day == 0 || day > 24 {
        println!("{} is out of range (1 to 24)", day)
    } else {
        SOLVERS[day - 1]()
    }
}
