use std::io;
use std::io::Read;

pub mod day01;
pub mod day02;
pub mod day03;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let result = day03::part2(&buffer);

    println!("{}", result);
}
