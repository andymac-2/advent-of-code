use crate::day05::Machine;

pub fn part1(mut machine: Machine) {
    println!("Memory used: {}", machine.mem_size());
    machine.print_mem();
    print!("\n\n\n---------\n\n\n");
    machine.terminal();
    print!("\n\n\n---------\n\n\n");
    println!("Memory used: {}", machine.mem_size());
    machine.print_mem();
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day21.txt")
        .expect("Something went wrong reading the file");

    let machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part1(machine));
}
