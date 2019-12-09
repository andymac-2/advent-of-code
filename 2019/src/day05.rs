use std::convert::TryFrom;
use std::io;
use std::io::Read;

use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::multi::separated_list;
use nom::sequence::tuple;
use nom::IResult;

fn number_p(input: &str) -> IResult<&str, i64> {
    let (input, (sign, num_str)) = tuple((opt(char('-')), digit1))(input)?;

    let mut number: i64 = num_str.parse().unwrap();
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
    Relative,
}

struct ParameterModes(i64);
impl Iterator for ParameterModes {
    type Item = ParameterMode;
    fn next(&mut self) -> Option<ParameterMode> {
        let result = match self.0 % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
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
    StackPtrAdd,
}

struct Opcode(i64);
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
            9 => OpcodeType::StackPtrAdd,
            99 => OpcodeType::Halt,
            _ => unreachable!(),
        }
    }
    fn param_modes(&self) -> ParameterModes {
        ParameterModes(self.0 / 100)
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    pc: usize,
    sp: i64,
    mem: Vec<i64>,
}
impl Machine {
    pub fn from_string(string: &str) -> Machine {
        program_p(string).unwrap().1
    }
    pub fn new(mem: Vec<i64>) -> Machine {
        Machine { pc: 0, sp: 0, mem }
    }
    pub fn is_halted(&self) -> bool {
        if let OpcodeType::Halt = Opcode(self.mem_get(self.pc)).op() {
            return true;
        }
        false
    }
    pub fn mem_size(&self) -> usize {
        self.mem.len()
    }
    pub fn run<I>(&mut self, mut input: I) -> Vec<i64>
    where
        I: Iterator<Item = i64>,
    {
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
                    if let Some(input) = input.next() {
                        self.write_parameter(input, &mut modes);
                    }
                    else {
                        self.pc -= 1;
                        break;
                    }
                }
                OpcodeType::Write => {
                    output.push(self.read_parameter(&mut modes));
                }
                OpcodeType::JumpNotZero => {
                    let predicate = self.read_parameter(&mut modes);
                    let new_pc = usize::try_from(self.read_parameter(&mut modes)).unwrap();
                    if predicate != 0 {
                        self.pc = new_pc;
                    }
                }
                OpcodeType::JumpZero => {
                    let predicate = self.read_parameter(&mut modes);
                    let new_pc = usize::try_from(self.read_parameter(&mut modes)).unwrap();
                    if predicate == 0 {
                        self.pc = new_pc;
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
                OpcodeType::StackPtrAdd => {
                    self.sp += self.read_parameter(&mut modes);
                }
                OpcodeType::Halt => {
                    self.pc -= 1;
                    break;
                }
            }
        }
        output
    }

    fn read_opcode(&mut self) -> Opcode {
        let opcode = Opcode(self.mem_get(self.pc));
        self.pc += 1;
        opcode
    }
    fn read_parameter(&mut self, modes: &mut ParameterModes) -> i64 {
        let value = match modes.next().unwrap() {
            ParameterMode::Immediate => self.mem_get(self.pc),
            ParameterMode::Position => self.mem_get(self.pc_indirect_addr()),
            ParameterMode::Relative => self.mem_get(self.sp_indirect_addr()),
        };
        self.pc += 1;
        value
    }

    fn mem_get(&self, addr: usize) -> i64 {
        *self.mem.get(addr).unwrap_or(&0)
    }
    fn mem_set(&mut self, addr: usize, value: i64) {
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = value;
    }

    fn pc_indirect_addr(&self) -> usize {
        usize::try_from(self.mem_get(self.pc)).unwrap()
    }
    fn sp_indirect_addr(&self) -> usize {
        let offset = self.mem_get(self.pc);
        usize::try_from(self.sp + offset).unwrap()
    }

    fn write_parameter(&mut self, value: i64, modes: &mut ParameterModes) {
        match modes.next().unwrap() {
            ParameterMode::Immediate => unreachable!(),
            ParameterMode::Position => self.mem_set(self.pc_indirect_addr(), value),
            ParameterMode::Relative => self.mem_set(self.sp_indirect_addr(), value),
        };
        self.pc += 1;
    }
}

pub fn part1(mut machine: Machine) -> Vec<i64> {
    machine.run([1].iter().copied())
}

pub fn part2(mut machine: Machine) -> Vec<i64> {
    machine.run([5].iter().copied())
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let machine = program_p(&buffer).unwrap().1;

    println!("Program Output: {:?}", part2(machine));
}
