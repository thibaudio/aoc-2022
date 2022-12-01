use std::path::Path;

use DailyChallenge;
use read_lines;

pub struct Day {}

impl DailyChallenge for Day {
    fn run() {
        let elves = load_elves("./assets/day01.txt");
        let result = find_max_calories(&elves.unwrap());
        println!("Day 01 - Part1: {}", result[0]);
        println!("Day 01 - Part2: {}", result[0] + result[1] + result[2]);
    }

}
#[derive(Debug, PartialEq)]
struct Elf {
    calories: Vec<u32>
}

impl Elf {
    fn new() -> Elf {
        Elf {
            calories: Vec::new()
        }
    }

    fn sum(&self) -> u32 {
        self.calories.iter().sum()
    }
}

fn load_elves(filename: impl AsRef<Path>) -> Result<Vec<Elf>, &'static str> {
    if let Ok(lines) = read_lines(filename) {
        let mut elves: Vec<Elf> = Vec::new();
        let mut current_elf = Elf::new();
        for line in lines {
            if let Ok(cal) = line {
                if cal.is_empty() {
                    elves.push(current_elf);
                    current_elf = Elf::new();
                } else {
                    current_elf.calories.push(cal.parse().unwrap());
                }
            }
        }
        Ok(elves)
    } else {
        Err("Nop")
    }
}

fn find_max_calories(elves: &Vec<Elf>) -> Vec<u32> {
    let mut calories = elves.into_iter().map(|e| e.sum()).collect::<Vec<u32>>();
    calories.sort_by(| a, b| b.cmp(a));
    calories
}

#[test]
fn test_load_elves() {
    let elves = load_elves("./assets/day01_test.txt");
    assert!(elves.is_ok());
    assert!(elves.unwrap()[0] == Elf {
        calories: vec![1000, 2000, 3000]
    });
}

#[test]
fn test_find_max_calories() {
    let elves = load_elves("./assets/day01_test.txt");
    let result = find_max_calories(&elves.unwrap());

    assert!(result[0] == 24000);
}
