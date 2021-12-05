//! Advent of Code 2021 - Day 01
//! https://adventofcode.com/2021/day/1

extern crate aoc_util as util;

use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash)]
enum MeasurementChange {
    Increase,
    Decrease,
    Steady,
}

impl fmt::Display for MeasurementChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeasurementChange::Increase => write!(f, "Measurement INCREASED"),
            MeasurementChange::Decrease => write!(f, "Measurement DECREASED"),
            MeasurementChange::Steady => write!(f, "Measurement stayed steady"),
        }
    }
}

fn main() {
    let input = util::get_input();

    let mut changes = HashMap::<MeasurementChange, usize>::new();

    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .fold::<Option<u32>, _>(None, |prev_depth, depth| match prev_depth {
            None => Some(depth),
            Some(prev_depth) => {
                let change = match depth.cmp(&prev_depth) {
                    std::cmp::Ordering::Less => MeasurementChange::Decrease,
                    std::cmp::Ordering::Greater => MeasurementChange::Increase,
                    std::cmp::Ordering::Equal => MeasurementChange::Steady,
                };

                let count = changes.entry(change).or_insert(0);
                *count += 1;

                Some(depth)
            }
        });

    for (change, count) in changes.iter() {
        println!("{} {} times", change, count);
    }
}
