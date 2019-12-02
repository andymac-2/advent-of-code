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

fn run(program: &mut [usize]) {
    // instruction pointer
    let mut ip = 0;
    loop {
        match program[ip] {
            1 => {
                let input1 = program[program[ip + 1]];
                let input2 = program[program[ip + 2]];
                program[program[ip + 3]] = input1 + input2;
                ip += 4;
            }
            2 => {
                let input1 = program[program[ip + 1]];
                let input2 = program[program[ip + 2]];
                program[program[ip + 3]] = input1 * input2;
                ip += 4;
            }
            99 => return,
            _ => unreachable!(),
        }
    }
}

pub fn part1(mut program: Vec<usize>) -> usize {
    program[1] = 12;
    program[2] = 2;
    run(&mut program);
    program[0]
}

pub fn part2(original_program: Vec<usize>) -> (usize, usize) {
    for x in 0..100 {
        for y in 0..100 {
            let mut program = original_program.clone();
            program[1] = x;
            program[2] = y;
            run(&mut program);
            if program[0] == 19_690_720 {
                return (x, y);
            }
        }
    }
    unreachable!()
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let program = program_p(&buffer).unwrap().1;

    println!("Program Output: {:?}", part2(program));
}
