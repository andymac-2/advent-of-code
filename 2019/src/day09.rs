use std::io;
use std::io::Read;

use crate::day05::Machine;

pub fn part1(machine: &mut Machine) -> Vec<i64> {
    machine.run([1].iter().copied())
}

pub fn part2(machine: &mut Machine) -> Vec<i64> {
    machine.run([2].iter().copied())
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part1(&mut machine));
    println!("memory size: {}", machine.mem_size());
}
