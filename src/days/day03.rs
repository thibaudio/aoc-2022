use std::path::Path;

use DailyChallenge;
use read_lines;

const LETTERS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
];

pub struct Day {}

impl DailyChallenge for Day {
    fn run() {
        let total = part1("./assets/day03.txt");
        println!("Day 01 - Part1: {}", total);

        let total2 = part2("./assets/day03.txt");
        println!("Day 01 - Part2: {}", total2);
    }

}

fn load_bags(filename: impl AsRef<Path>) -> Result<Vec<Vec<char>>, &'static str> {
    if let Ok(lines) = read_lines(filename) {
        let mut bags = Vec::new();
        for line in lines {
            if let Ok(bag_string) = line {
                let bag = bag_string.chars().collect();
                bags.push(bag);
            }
        }
        Ok(bags)
    } else {
        Err("Nop")
    }
}

fn find_duplicates_in_bag(bag: &Vec<char>) -> char {
    let hbags: Vec<&[char]> = bag.chunks(bag.len() / 2).collect();
    for i in hbags[0] {
        if hbags[1].contains(i) {
            return i.clone();
        }
    }
    panic!("Duplicated not found in {:?} : {:?}", hbags[0], hbags[1]);
}

fn find_duplicates_in_groups(bags: &[Vec<char>]) -> char {
    for i in &bags[0] {
        if bags[1].contains(&i) && bags[2].contains(&i) {
            return i.clone();
        }
    }
    panic!("Duplicated not found in {:?}\n{:?}\n{:?}", bags[0], bags[1], bags[2]);
}

fn part1(filename: impl AsRef<Path>) -> usize {
    let bags = load_bags(filename).unwrap();
    let mut total = 0;
    for i in bags {
        total += to_index(find_duplicates_in_bag(&i));
    }
    total
}

fn part2(filename: impl AsRef<Path>) -> usize {
    let binding = load_bags(filename).unwrap();
    let bags: Vec<&[Vec<char>]> = binding.chunks(3).collect();
    let mut total = 0;
    for i in bags {
        total += to_index(find_duplicates_in_groups(i));
    }
    total
}

fn to_index(character: char) -> usize {
    if let Some(index) = LETTERS.iter().position(| &x | x == character) {
        index + 1
    }
    else {
        panic!("Letter not foud {}", character);
    }
}

#[test]
fn test_load_bags() {
    let bags = load_bags("./assets/day03_test.txt");
    assert!(bags.is_ok());
}


#[test]
fn test_part1() {
    let total = part1("./assets/day03_test.txt");
    assert_eq!(total, 157);
}


#[test]
fn test_part2() {
    let total = part2("./assets/day03_test.txt");
    assert_eq!(total, 70);
}
