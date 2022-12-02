use DailyChallenge;
mod day01;
mod day02;

pub fn run_day(day: u32) {
    match day {
        1 => day01::Day::run(),
        2 => day02::Day::run(),
        _ => panic!("Day not implemented"),
    }
}
