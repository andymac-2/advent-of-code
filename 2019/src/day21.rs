use crate::day05::Machine;

pub fn part1(mut machine: Machine) {
    machine.terminal();
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day21.txt")
        .expect("Something went wrong reading the file");

    let machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part1(machine));
}
