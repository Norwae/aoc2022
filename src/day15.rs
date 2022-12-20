use std::cmp::{max, min};
use std::mem::swap;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{terminated, tuple};

use crate::util::{default_solution, parse_i64};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct SignedIndex(i32, i32);

impl SignedIndex {
    pub fn max_distance(self, other: Self) -> usize {
        ((self.0 - other.0).abs() as usize) + ((self.1 - other.1).abs() as usize)
    }

}

#[derive(Debug)]
struct ScanRange {
    from: i32,
    to: i32,
    beacons: Vec<SignedIndex>,
}

impl ScanRange {
    fn new(from: i32, to: i32) -> Self {
        Self { from, to, beacons: Vec::new() }
    }
    fn length(&self) -> usize {
        (1 + self.to - self.from) as usize
    }

    fn contains(&self, index: i32) -> bool {
        index >= self.from && index <= self.to
    }

    fn intersects(&self, other: &Self) -> bool {
        (self.from <= other.from && other.from <= self.to) || // [self.from, other.from, self.to, other.to ] OR [self.from, other.from, other.to, self.to ]
            (other.from <= self.from && self.from <= other.to)  // [other.from, self.from, other.to, self.to ] OR [ other.from, self.from, self.to, other.to ]
    }

    fn merged_with(&mut self, other: ScanRange) {
        self.from = min(self.from, other.from);
        self.to = max(self.to, other.to);

        for beacon in other.beacons {
            self.add_beacon(beacon)
        }
    }

    fn add_beacon(&mut self, beacon: SignedIndex) {
        if self.contains(beacon.0) {
            if !self.beacons.contains(&beacon) {
                self.beacons.push(beacon);
            }
        }
    }
}

fn s_index(input: &str) -> IResult<&str, SignedIndex> {
    map(tuple((
        tag("x="),
        parse_i64,
        tag(", y="),
        parse_i64
    )), |(_, x, _, y)| SignedIndex(x as i32, y as i32))(input)
}

fn line(input: &str) -> IResult<&str, (SignedIndex, SignedIndex)> {
    map(tuple((
        tag("Sensor at "),
        s_index,
        tag(": closest beacon is at "),
        s_index
    )), |(_, i1, _, i2)| (i1, i2))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(SignedIndex, SignedIndex)>> {
    many1(terminated(line, tag("\n")))(input)
}


fn solve_problem(sensor_beacon_pairs: Vec<(SignedIndex, SignedIndex)>) -> (usize, usize) {
    const EXAMINED_ROW: i32 = 2000000;
    //const EXAMINED_ROW: i32 = 10;

    let mut ranges = Vec::<ScanRange>::new();

    for (sensor, beacon) in sensor_beacon_pairs.iter().cloned() {
        let distance_to_beacon = sensor.max_distance(beacon);
        let row_in_same_column = SignedIndex(sensor.0, EXAMINED_ROW);
        let distance_to_sensor = row_in_same_column.max_distance(sensor);

        if distance_to_sensor <= distance_to_beacon {
            let remaining = (distance_to_beacon - distance_to_sensor) as i32;
            let range_in_row = ScanRange::new(sensor.0 - remaining, sensor.0 + remaining);
            ranges.push(range_in_row);
        }
    }

    merge_ranges(&mut ranges);
    for beacon in sensor_beacon_pairs.iter().map(|(_, b)| b) {
        if beacon.1 == EXAMINED_ROW {
            for range in &mut ranges {
                range.add_beacon(*beacon)
            }
        }
    }

    let mut range_sum = 0;
    let mut beacon_count = 0;

    for range in ranges {
        range_sum += range.length();
        beacon_count += range.beacons.len()
    }

    (range_sum - beacon_count, 0)
}

fn merge_ranges(covered_ranges: &mut Vec<ScanRange>) {
    let mut eliminated_range = true;
    let mut temp: Vec<ScanRange>;
    while eliminated_range {
        temp = Vec::new();
        eliminated_range = false;
        swap(&mut temp, covered_ranges);

        for considered_range in temp {
            if let Some(existing) = covered_ranges.iter_mut().find(|vested_range| considered_range.intersects(vested_range)) {
                existing.merged_with(considered_range);
                eliminated_range = true;
            } else {
                covered_ranges.push(considered_range);
            }
        }
    }
}

default_solution!(parse_input, solve_problem);