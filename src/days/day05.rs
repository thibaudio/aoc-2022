extern crate regex;
use std::{path::Path, ops::Range, fs};
use crate::read_lines;

use self::regex::Regex;

use DailyChallenge;
use structopt::lazy_static::lazy_static;

pub struct Day {}

impl DailyChallenge for Day {
    fn run() {
        let total = part1("./assets/day05.txt");
        println!("Day 5 - Part1: {}", total);

        let total2 = part2("./assets/day05.txt");
        println!("Day 05 - Part2: {}", total2);
    }

}

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize
}


impl From<String> for Move {
    fn from(line: String) -> Self {
        lazy_static! {
            static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
        }
        for m in MOVE_RE.captures_iter(&line) {
            return Move {
                quantity: m[1].parse::<usize>().expect(""),
                from: m[2].parse::<usize>().expect(""),
                to: m[3].parse::<usize>().expect("")
            };
        }
        panic!("Cannot parse move: {}", line);
    }
}

#[derive(Debug)]
struct Stack(Vec<Vec<char>>);

impl Stack {
    fn add_line(&mut self, line: String) {
        for (i, c) in line.chars().enumerate() {
            if c != ' ' && c != '[' && c != ']' {
                self.0[(i - 1) / 4 + 1].push(c);
            }
        }
    }

    fn finalize(&mut self) {
        for i in self.0.iter_mut() {
            i.reverse();
        }
    }

    fn new() -> Self {
        let mut stack = Stack(Vec::new());
        for i in 0..10 {
            stack.0.push(Vec::new());
        }
        stack
    }

    fn apply(&mut self, moves: Vec<Move>) {
        for m in moves {
            for _ in 0..m.quantity {
                let to_move = self.0[m.from].pop().expect("Weird stack");
                self.0[m.to].push(to_move);
            }
        }
    }

    fn apply2(&mut self, moves: Vec<Move>) {
        for m in moves {
            let mut to_move = Vec::new();
            for _ in 0..m.quantity {
                to_move.push(self.0[m.from].pop().expect("Weird stack"));
                
            }
            to_move.reverse();
            self.0[m.to].extend(to_move);
        }
    }
}


fn load_input(filename: impl AsRef<Path>) -> Result<(Stack, Vec<Move>), &'static str> {
    let mut reading_stacks = true;
    if let Ok(lines) = read_lines(filename) {
        let mut stack = Stack::new();
        let mut moves = Vec::new();
        for line in lines {
            if let Ok(stack_string) = line {
                if !reading_stacks {
                    moves.push(stack_string.into());
                }
                else {
                    if stack_string == " 1   2   3 " || 
                        stack_string == " 1   2   3   4   5   6   7   8   9 "
                    {
                        continue;
                    }
                    if stack_string.is_empty() {
                        stack.finalize();
                        reading_stacks = false;
                    }
                    stack.add_line(stack_string);
                }
            }
        }
        Ok((stack,moves))
    } else {
        Err("Nop")
    }
}

fn part1(filename: impl AsRef<Path>) -> String {
    let mut input = load_input(filename).unwrap();
    input.0.apply(input.1);
    let mut output = "".to_string();
    for i in input.0.0 {
        if i.len() > 0 {
            output.push_str(&i[i.len() - 1].to_string());
        }
    }
    output
}

fn part2(filename: impl AsRef<Path>) -> String {
    let mut input = load_input(filename).unwrap();
    input.0.apply2(input.1);
    let mut output = "".to_string();
    for i in input.0.0 {
        if i.len() > 0 {
            output.push_str(&i[i.len() - 1].to_string());
        }
    }
    output
}

#[test]
fn test_load_input() {
    let input = load_input("./assets/day05_test.txt");
    assert!(input.is_ok());
    assert_eq!(input.unwrap().0.0[1], vec!['Z','N']);
}

#[test]
fn test_part1() {
    let total = part1("./assets/day05_test.txt");
    assert_eq!(total, "CMZ");
}

#[test]
fn test_part2() {
    let total = part2("./assets/day05_test.txt");
    assert_eq!(total, "MCD");
}
