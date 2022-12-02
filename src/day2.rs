use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated, tuple};
use crate::util::read_to_eof_line;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Lose,
    Tie,
    Win,
}

impl Outcome {
    fn score(self) -> i64 {
        3 * (self as i64)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}


macro_rules! rps_match {
    ($matched:expr, $rock:expr, $paper:expr, $scissors:expr) => {
        match $matched {
            RPS::Rock => $rock,
            RPS::Paper => $paper,
            RPS::Scissors => $scissors
        }
    };
}

impl RPS {
    fn play(self, other: RPS) -> Outcome {
        rps_match!(self,
            rps_match!(other, Outcome::Tie, Outcome::Lose, Outcome::Win),
            rps_match!(other, Outcome::Win, Outcome::Tie, Outcome::Lose),
            rps_match!(other, Outcome::Lose, Outcome::Win, Outcome::Tie)
        )
    }

    fn countermove_for_outcome(self, outcome: Outcome) -> RPS {
        rps_match!(self, match outcome {
            Outcome::Win => RPS::Paper,
            Outcome::Tie => RPS::Rock,
            Outcome::Lose => RPS::Scissors
        },
        match outcome {
            Outcome::Win => RPS::Scissors,
            Outcome::Tie => RPS::Paper,
            Outcome::Lose => RPS::Rock
        },
        match outcome {
            Outcome::Win => RPS::Rock,
            Outcome::Tie => RPS::Scissors,
            Outcome::Lose => RPS::Paper
        })
    }

    fn score(self) -> i64 {
        rps_match!(self, 1, 2, 3)
    }
}

fn score_round(left: RPS, right: RPS) -> i64 {
    let move_score = right.score();
    let outcome = right.play(left);
    let match_score = outcome.score();

    move_score + match_score
}

fn rps_move(input: &str) -> IResult<&str, RPS> {
    map(one_of("ABCXYZ"),
        |ch|
            match ch {
                'A' | 'X' => RPS::Rock,
                'B' | 'Y' => RPS::Paper,
                'C' | 'Z' => RPS::Scissors,
                _ => unreachable!()
            },
    )(input)
}

fn rps_outcome(input: &str) -> IResult<&str, Outcome> {
    map(one_of("XYZ"),
        |ch| match ch {
            'X' => Outcome::Lose,
            'Y' => Outcome::Tie,
            'Z' => Outcome::Win,
            _ => unreachable!()
        }
    )(input)
}

fn rps_move_line(input: &str) -> IResult<&str, (RPS, RPS)> {
    terminated(separated_pair(rps_move, space1, rps_move), line_ending)(input)
}

fn rps_move_outcome_line(input: &str) -> IResult<&str, (RPS, RPS)> {
    let parse_move_outcome = terminated(separated_pair(rps_move, space1, rps_outcome), line_ending);

    map(
        parse_move_outcome,
        |(mve, outcome)| (mve, mve.countermove_for_outcome(outcome))
    )(input)
}

fn round_1_strategy(input: &str) -> IResult<&str, Vec<(RPS, RPS)>> {
    many1(rps_move_line)(input)
}

fn round_2_strategy(input: &str) -> IResult<&str, Vec<(RPS, RPS)>> {
    many1(rps_move_outcome_line)(input)
}

pub fn solve() {
    let input = read_to_eof_line();
    if let Ok(("", strat)) = round_1_strategy(&input) {
        let score = strat.iter().fold(0i64, |sum, next|
            sum + score_round(next.0, next.1)
        );

        println!("Part 1: Expected score {}", score)
    }

    if let Ok(("", strat)) = round_2_strategy(&input) {
        let score = strat.iter().fold(0i64, |sum, next|
            sum + score_round(next.0, next.1)
        );

        println!("Part 2: Expected score {}", score)
    }
}