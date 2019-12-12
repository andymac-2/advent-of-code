use std::convert::TryFrom;
use std::io;
use std::io::Read;

use crate::day05::Machine;

struct Permutation {
    iteration: usize,
    length: usize,
}
impl Permutation {
    fn new(length: usize) -> Self {
        Permutation {
            iteration: 0,
            length,
        }
    }
}
impl Iterator for Permutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let mut vec = Vec::new();
        let mut iteration = self.iteration;

        for x in 0..self.length {
            let index = iteration % (x + 1);
            iteration /= x + 1;

            vec.insert(usize::try_from(index).unwrap(), x);
        }

        if iteration == 0 {
            self.iteration += 1;
            Some(vec)
        } else {
            None
        }
    }
}

pub fn part1(machine: &Machine) -> i64 {
    Permutation::new(5)
        .map(|permutation| {
            permutation.into_iter().fold(0, |input, phase| {
                let mut machine = machine.clone();
                let phase = i64::try_from(phase).unwrap();
                machine.run([phase, input].iter().copied())[0]
            })
        })
        .max()
        .unwrap()
}

const NUM_MACHINES: usize = 5;
pub fn part2(machine: &Machine) -> i64 {
    Permutation::new(NUM_MACHINES)
        .map(|permutation| {
            let mut inputs: Vec<_> = permutation
                .into_iter()
                .map(|phase| vec![i64::try_from(phase).unwrap() + 5])
                .collect();
            let mut machines: Vec<_> = (0..NUM_MACHINES).map(|_| machine.clone()).collect();

            inputs[0].push(0);

            while !machines[NUM_MACHINES - 1].is_halted() {
                for i in 0..NUM_MACHINES {
                    assert!(!inputs[i].is_empty());
                    let mut output = machines[i].run(inputs[i].drain(0..));
                    inputs[(i + 1) % NUM_MACHINES].append(&mut output);
                }
            }

            *inputs[0].last().unwrap()
        })
        .max()
        .unwrap()
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part2(&machine));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1_0() {
        let machine = Machine::from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(part1(&machine), 43210);
    }
    #[test]
    fn example_1_1() {
        let machine = Machine::from_string(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,\
             99,0,0",
        );
        assert_eq!(part1(&machine), 54321);
    }
    #[test]
    fn example_1_2() {
        let machine = Machine::from_string(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,\
             33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        assert_eq!(part1(&machine), 65210);
    }

    #[test]
    fn example_2_0() {
        let machine = Machine::from_string(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,\
             28,1005,28,6,99,0,0,5",
        );
        assert_eq!(part2(&machine), 139_629_729);
    }

    #[test]
    fn example_2_1() {
        let machine = Machine::from_string(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,\
             54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,\
             4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(part2(&machine), 18216);
    }
}
