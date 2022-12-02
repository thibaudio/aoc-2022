use std::path::Path;

use DailyChallenge;
use read_lines;

pub struct Day {}

impl DailyChallenge for Day {
    fn run() {
        let rounds = load_rounds("./assets/day02.txt");
        let score = total_score(rounds.unwrap());
        println!("Day 02 - Part1: {}", score);

        let rounds2 = load_rounds_from_outcome("./assets/day02.txt");
        let score2 = total_score(rounds2.unwrap());
        println!("Day 02 - Part2: {}", score2);
    }

}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl Play {
    fn beats(&self) -> Play {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper
        }
    }

    fn looses(&self) -> Play {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock
        }
    }
}

impl From<char> for Play {
    fn from(l: char) -> Self {
        match l {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            'X' => Play::Rock,
            'Y' => Play::Paper,
            'Z' => Play::Scissors,
            _ => panic!("Not supported")
        }
    }
}

#[derive(PartialEq, Debug)]
struct Round {
    them: Play,
    us: Play
}

impl Round {
    fn score(&self) -> u32 {
        let mut score = self.us as u32;
        if self.us.beats() == self.them {
            score += 6
        } else if self.us == self.them {
            score += 3
        }
        score
    }

    fn from_outcome(line: String) -> Self {
        let chars: Vec<char> = line.chars().collect();
        let them: Play = chars[0].into();

        let us = match chars[2] {
            'X' => them.beats(),
            'Y' => them.clone(),
            'Z' => them.looses(),
            _ => panic!("Not supported")
        };

        Round {
            them,
            us
        }
    }
}

impl From<String> for Round {
    fn from(line: String) -> Self {
        let chars: Vec<char> = line.chars().collect();
        Round {
            them: chars[0].into(),
            us: chars[2].into(),
        }
    }
}

fn load_rounds(filename: impl AsRef<Path>) -> Result<Vec<Round>, &'static str> {
    if let Ok(lines) = read_lines(filename) {
        let mut rounds = Vec::new();
        for line in lines {
            if let Ok(round_string) = line {
                let round: Round = round_string.into();
                rounds.push(round);
            }
        }
        Ok(rounds)
    } else {
        Err("Nop")
    }
}

fn load_rounds_from_outcome(filename: impl AsRef<Path>) -> Result<Vec<Round>, &'static str> {
    if let Ok(lines) = read_lines(filename) {
        let mut rounds = Vec::new();
        for line in lines {
            if let Ok(round_string) = line {
                let round: Round = Round::from_outcome(round_string);
                rounds.push(round);
            }
        }
        Ok(rounds)
    } else {
        Err("Nop")
    }
}

fn total_score(rounds: Vec<Round>) -> u32 {
    rounds.iter().fold(0, | acc, x| acc + x.score())
}

#[test]
fn test_load_rounds() {
    let rounds = load_rounds("./assets/day02_test.txt");
    assert!(rounds.is_ok());
    assert!(rounds.unwrap()[0] == Round {
        them: Play::Rock,
        us: Play::Paper
    });
}

#[test]
fn test_total_score() {
    let rounds = load_rounds("./assets/day02_test.txt");
    let score = total_score(rounds.unwrap());
    assert_eq!(score, 15);
}

#[test]
fn test_load_rounds_from_outcome() {
    let rounds = load_rounds_from_outcome("./assets/day02_test.txt");
    assert!(rounds.is_ok());
    let score = total_score(rounds.unwrap());
    assert_eq!(score, 12);
}
