use crate::util::read_file;

pub mod task1;
pub mod task2;

pub fn task1() -> usize{
    let vec = read_file::<String>("res/day_3.txt").unwrap();
    let result = task1::calculate(&vec);
    result.unwrap().score()
}
