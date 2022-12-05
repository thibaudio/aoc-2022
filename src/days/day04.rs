extern crate regex;
use std::{path::Path, ops::Range, fs};
use self::regex::Regex;

use DailyChallenge;

pub struct Day {}

impl DailyChallenge for Day {
    fn run() {
        let total = part1("./assets/day04.txt");
        println!("Day 04 - Part1: {}", total);

        let total2 = part2("./assets/day04.txt");
        println!("Day 04 - Part2: {}", total2);
    }

}

fn load_pairs(filename: impl AsRef<Path>) -> Vec<(Range<usize>, Range<usize>)> {
    let content = fs::read_to_string(filename).expect("Not able to read file");
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut pairs = Vec::new();
    for cap in re.captures_iter(&content) {
        pairs.push((cap[1].parse::<usize>().unwrap()..cap[2].parse::<usize>().unwrap(), cap[3].parse::<usize>().unwrap()..cap[4].parse::<usize>().unwrap()));
    }
    pairs
}

fn part1(filename: impl AsRef<Path>) -> usize {
    let mut total = 0;
    for i in load_pairs(filename) {
        if (i.0.start <= i.1.start && i.0.end >= i.1.end) ||
            (i.1.start <= i.0.start && i.1.end >= i.0.end)
        {
            total += 1;
        }
    }
    total
}

fn part2(filename: impl AsRef<Path>) -> usize {
    let mut total = 0;
    for i in load_pairs(filename) {
        if (i.0.start <= i.1.start && i.0.end >= i.1.start) ||
            (i.1.start <= i.0.start && i.1.end >= i.0.start)
        {
            total += 1;
        }
    }
    total
}

#[test]
fn test_load_pairs() {
    let pairs = load_pairs("./assets/day04_test.txt");
    assert_eq!(pairs[0], ((2..4, 6..8)));
}

#[test]
fn test_part1() {
    let total = part1("./assets/day04_test.txt");
    assert_eq!(total, 2);
}

#[test]
fn test_part2() {
    let total = part2("./assets/day04_test.txt");
    assert_eq!(total, 4);
}
