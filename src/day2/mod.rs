use crate::util::read_file;

mod task1;
mod task2;

pub fn task1() -> isize {
    let steps = read_file("res/day_2.txt").unwrap();
    task1::calculate_position(&steps).score()
}

pub fn task2() -> isize {
    let steps = read_file("res/day_2.txt").unwrap();
    task2::calculate_position(&steps).score()
}
