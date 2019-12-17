use std::convert::TryInto;

use crate::day05::Machine;

pub fn part1(mut machine: Machine) -> usize {
    let buffer: Vec<char> = machine
        .run(std::iter::empty())
        .into_iter()
        .map(|char_num| {
            let char_8: u8 = char_num.try_into().unwrap();
            char::from(char_8)
        })
        .collect();

    let grid: Vec<&[char]> = buffer.split(|c| *c == '\n').collect();

    let mut sum = 0;

    for (y, line) in grid.iter().copied().enumerate() {
        for (x, c) in line.iter().copied().enumerate() {
            if c != '#' || y == 0 || x == 0 {
                continue;
            }

            let result = (|| {
                let up = grid.get(y - 1)?.get(x)?;
                let down = grid.get(y + 1)?.get(x)?;
                let right = grid.get(y)?.get(x + 1)?;
                let left = grid.get(y)?.get(x - 1)?;

                if *up == '#' && *down == '#' && *left == '#' && *right == '#' {
                    return Some(x * y);
                }
                None
            })();

            let _ = result.map(|alignment| sum += alignment);
        }
    }

    sum
}

pub fn part2(mut machine: Machine) {
    machine.mem_set(0, 2);
    machine.terminal();
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day17.txt")
        .expect("Something went wrong reading the file");

    let machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part2(machine));
}
