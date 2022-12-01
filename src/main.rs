extern crate structopt;
use std::{fs::File, io::{self, BufRead}, path::Path};

use structopt::StructOpt;

mod days;

#[derive(Debug, StructOpt)]
struct Cli {
    // Which day we want to run
    #[structopt(short, long)]
    day: u32,
}

pub trait DailyChallenge {
    fn run();
}

fn main() {
    let args = Cli::from_args();
    days::run_day(args.day);
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
