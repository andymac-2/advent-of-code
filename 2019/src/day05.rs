use std::convert::TryFrom;
use std::io;
use std::io::Read;

use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::multi::separated_list;
use nom::sequence::tuple;
use nom::IResult;

fn number_p(input: &str) -> IResult<&str, i32> {
    let (input, (sign, num_str)) = tuple((opt(char('-')), digit1))(input)?;

    let mut number: i32 = num_str.parse().unwrap();
    if sign.is_some() {
        number = -number;
    }
    Ok((input, number))
}

fn program_p(input: &str) -> IResult<&str, Machine> {
    map(separated_list(char(','), number_p), Machine::new)(input)
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

struct ParameterModes(i32);
impl Iterator for ParameterModes {
    type Item = ParameterMode;
    fn next(&mut self) -> Option<ParameterMode> {
        let result = match self.0 % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        self.0 /= 10;
        Some(result)
    }
}

enum OpcodeType {
    Add,
    Mul,
    Read,
    Write,
    JumpNotZero,
    JumpZero,
    LessThan,
    Equals,
    Halt,
}

struct Opcode(i32);
impl Opcode {
    fn op(&self) -> OpcodeType {
        match self.0 % 100 {
            1 => OpcodeType::Add,
            2 => OpcodeType::Mul,
            3 => OpcodeType::Read,
            4 => OpcodeType::Write,
            5 => OpcodeType::JumpNotZero,
            6 => OpcodeType::JumpZero,
            7 => OpcodeType::LessThan,
            8 => OpcodeType::Equals,
            99 => OpcodeType::Halt,
            _ => unreachable!(),
        }
    }
    fn param_modes(&self) -> ParameterModes {
        ParameterModes(self.0 / 100)
    }
}

pub struct Machine {
    ip: usize,
    mem: Vec<i32>,
}
impl Machine {
    fn new(mem: Vec<i32>) -> Machine {
        Machine { ip: 0, mem }
    }
    fn read_opcode(&mut self) -> Opcode {
        let opcode = Opcode(self.mem[self.ip]);
        self.ip += 1;
        opcode
    }
    fn read_parameter(&mut self, modes: &mut ParameterModes) -> i32 {
        let value = match modes.next().unwrap() {
            ParameterMode::Immediate => self.mem[self.ip],
            ParameterMode::Position => self.mem[self.get_address()],
        };
        self.ip += 1;
        value
    }
    fn get_address(&self) -> usize {
        usize::try_from(self.mem[self.ip]).unwrap()
    }
    fn write_parameter(&mut self, value: i32, modes: &mut ParameterModes) {
        assert_eq!(modes.next().unwrap(), ParameterMode::Position);
        let addr = self.get_address();
        self.mem[addr] = value;
        self.ip += 1;
    }
    fn run(&mut self, mut input: &[i32]) -> Vec<i32> {
        let mut output = Vec::new();
        loop {
            let opcode = self.read_opcode();
            let mut modes = opcode.param_modes();
            match opcode.op() {
                OpcodeType::Add => {
                    let input1 = self.read_parameter(&mut modes);
                    let input2 = self.read_parameter(&mut modes);
                    self.write_parameter(input1 + input2, &mut modes);
                }
                OpcodeType::Mul => {
                    let input1 = self.read_parameter(&mut modes);
                    let input2 = self.read_parameter(&mut modes);
                    self.write_parameter(input1 * input2, &mut modes);
                }
                OpcodeType::Read => {
                    let (head, tail) = input.split_first().unwrap();
                    input = tail;
                    self.write_parameter(*head, &mut modes);
                }
                OpcodeType::Write => {
                    output.push(self.read_parameter(&mut modes));
                }
                OpcodeType::JumpNotZero => {
                    let predicate = self.read_parameter(&mut modes);
                    let new_ip = usize::try_from(self.read_parameter(&mut modes)).unwrap();
                    if predicate != 0 {
                        self.ip = new_ip;
                    }
                }
                OpcodeType::JumpZero => {
                    let predicate = self.read_parameter(&mut modes);
                    let new_ip = usize::try_from(self.read_parameter(&mut modes)).unwrap();
                    if predicate == 0 {
                        self.ip = new_ip;
                    }
                }
                OpcodeType::LessThan => {
                    let input1 = self.read_parameter(&mut modes);
                    let input2 = self.read_parameter(&mut modes);
                    if input1 < input2 {
                        self.write_parameter(1, &mut modes);
                    } else {
                        self.write_parameter(0, &mut modes);
                    }
                }
                OpcodeType::Equals => {
                    let input1 = self.read_parameter(&mut modes);
                    let input2 = self.read_parameter(&mut modes);
                    if input1 == input2 {
                        self.write_parameter(1, &mut modes);
                    } else {
                        self.write_parameter(0, &mut modes);
                    }
                }
                OpcodeType::Halt => break,
            }
        }
        output
    }
}

pub fn part1(mut machine: Machine) -> Vec<i32> {
    machine.run(&[1])
}

pub fn part2(mut machine: Machine) -> Vec<i32> {
    machine.run(&[5])
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let machine = program_p(&buffer).unwrap().1;

    println!("Program Output: {:?}", part2(machine));
}
