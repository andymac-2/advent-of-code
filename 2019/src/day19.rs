use crate::day05::Machine;

fn affected(machine: &Machine, x: i64, y: i64) -> bool {
    let output = machine.clone().run([x, y].iter().copied());
    output[0] == 1
}

/// question asks for default extent of 50.
pub fn part1(extent: i64) -> i64 {
    let buffer = std::fs::read_to_string("./inputs/day19.txt")
        .expect("Something went wrong reading the file");
    let machine = Machine::from_string(&buffer);

    let mut num_affected = 0;

    for x in 0..extent {
        for y in 0..extent {
            let output = machine.clone().run([x, y].iter().copied());
            num_affected += output[0];
        }
    }
    num_affected
}

/// question asks for default extent of 100.
pub fn part2(extent: i64) -> i64 {
    let buffer = std::fs::read_to_string("./inputs/day19.txt")
        .expect("Something went wrong reading the file");
    let machine = Machine::from_string(&buffer);

    let mut left = 0;
    let mut bottom = 100;
    loop {
        if !affected(&machine, left, bottom) {
            left += 1;
            continue;
        }
        let top = bottom - extent + 1;
        let right = left + extent - 1;
        if !affected(&machine, right, top) {
            bottom += 1;
            continue;
        }
        return left * 10_000 + top;
    }
}

pub fn start() {
    println!("Program Output: {:?}", part2(100));
}
