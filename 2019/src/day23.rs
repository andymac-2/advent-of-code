use std::collections::VecDeque;
use std::convert::TryFrom;

use crate::day05::Machine;

struct Network {
    machines: Vec<Machine>,
    packets: VecDeque<[i64; 3]>,
    current_idle: usize,
    nat: Option<(i64, i64)>,
}
impl Network {
    fn new(machine: &Machine, length: usize) -> Self {
        let machines: Vec<_> = std::iter::repeat_with(|| machine.clone())
            .take(length)
            .collect();
        let packets = VecDeque::new();

        let mut network = Network {
            machines,
            packets,
            current_idle: 0,
            nat: None,
        };

        for i in 0..network.machines.len() {
            let address = i64::try_from(i).unwrap();
            let output = network.machines[i].run(std::iter::once(address));
            network.disperse(&output);
        }

        network
    }
    fn disperse(&mut self, output: &[i64]) {
        for packet in output.chunks_exact(3) {
            match packet {
                [addr, x, y] => self.packets.push_back([*addr, *x, *y]),
                _ => unreachable!(),
            }
        }
    }
    fn step(&mut self) {
        let output = if let Some(packet) = self.packets.pop_front() {
            self.current_idle = 0;

            let addr = usize::try_from(packet[0]).unwrap();
            let x = packet[1];
            let y = packet[2];

            if let Some(machine) = self.machines.get_mut(addr) {
                machine.run([x, y].iter().copied())
            } else {
                assert!(addr == 255);
                self.nat = Some((x, y));
                return;
            }
        } else {
            let current = self.current_idle;
            self.current_idle += 1;
            if self.current_idle >= self.machines.len() {
                // idle
                let (x, y) = self.nat.unwrap();
                dbg!(y);
                self.machines[0].run([x, y].iter().copied())
            } else {
                self.machines[current].run(std::iter::once(-1))
            }
        };

        self.disperse(&output);
    }
    fn last_nat(&self) -> Option<(i64, i64)> {
        self.nat
    }
}

const NUM_MACHINES: usize = 50;
pub fn part1(machine: &Machine) -> i64 {
    let mut network = Network::new(machine, NUM_MACHINES);
    loop {
        network.step();
        if let Some((_, y)) = network.last_nat() {
            break y;
        }
    }
}

pub fn part2(machine: &Machine) -> i64 {
    let mut network = Network::new(machine, NUM_MACHINES);
    //let mut ys = HashSet::new();
    loop {
        network.step();
    }
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day23.txt").unwrap();
    let machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part2(&machine));
}
