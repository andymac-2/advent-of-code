use std::io;
use std::io::Read;

use nom::character::complete::{char, digit1};
use nom::multi::separated_list;
use nom::IResult;

fn number_p(input: &str) -> IResult<&str, usize> {
    let (input, num_str) = digit1(input)?;
    Ok((input, num_str.parse().unwrap()))
}

fn program_p(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list(char(','), number_p)(input)
}

fn run(mem: &mut [usize]) {
    // instruction pointer
    let mut ip = 0;
    loop {
        match mem[ip] {
            1 => {
                let input1 = mem[mem[ip + 1]];
                let input2 = mem[mem[ip + 2]];
                mem[mem[ip + 3]] = input1 + input2;
                ip += 4;
            }
            2 => {
                let input1 = mem[mem[ip + 1]];
                let input2 = mem[mem[ip + 2]];
                mem[mem[ip + 3]] = input1 * input2;
                ip += 4;
            }
            99 => return,
            _ => unreachable!(),
        }
    }
}

pub fn part1(mut mem: Vec<usize>) -> usize {
    mem[1] = 12;
    mem[2] = 2;
    run(&mut mem);
    mem[0]
}

pub fn part2(original_mem: Vec<usize>) -> (usize, usize) {
    for x in 0..100 {
        for y in 0..100 {
            let mut mem = original_mem.clone();
            mem[1] = x;
            mem[2] = y;
            run(&mut mem);
            if mem[0] == 19_690_720 {
                return (x, y);
            }
        }
    }
    unreachable!()
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mem = program_p(&buffer).unwrap().1;

    println!("Program Output: {:?}", part2(mem));
}
