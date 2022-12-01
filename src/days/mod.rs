use DailyChallenge;
mod day01;

pub fn run_day(day: u32) {
    match day {
        1 => day01::Day::run(),
        _ => panic!("Day not implemented"),
    }
}
