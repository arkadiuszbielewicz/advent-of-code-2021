use crate::day1::task1::count_increments;
use crate::day1::task2::count_increments_mean;
use crate::util::read_file;

mod task1;
mod task2;

pub fn task1() -> usize {
    let report = read_file::<usize>("res/day_1.txt").unwrap();
    count_increments(&report)
}

pub fn task2() -> usize {
    let report = read_file::<usize>("res/day_1.txt").unwrap();
    count_increments_mean(&report)
}
