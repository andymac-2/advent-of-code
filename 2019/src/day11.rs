use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::Read;

use crate::day05::Machine;

#[must_use]
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn turn_cw(&mut self) {
        match self {
            Self::Up => *self = Self::Right,
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
        }
    }
    fn turn_ccw(&mut self) {
        match self {
            Self::Up => *self = Self::Left,
            Self::Right => *self = Self::Up,
            Self::Down => *self = Self::Right,
            Self::Left => *self = Self::Down,
        }
    }
    fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::CCW => self.turn_ccw(),
            Rotation::CW => self.turn_cw(),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point(i32, i32);
impl Point {
    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.1 -= 1,
            Direction::Right => self.0 += 1,
            Direction::Down => self.1 += 1,
            Direction::Left => self.0 -= 1,
        }
    }
}

const ANSI_RED: &str = "\x1B[41m";
const ANSI_GREEN: &str = "\x1B[42m";
const ANSI_RESET: &str = "\x1B[0m";

#[derive(Clone, Copy, Debug)]
enum Colour {
    Black = 0,
    White = 1,
}
impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "{}  {}", ANSI_GREEN, ANSI_RESET),
            Self::White => write!(f, "{}  {}", ANSI_RED, ANSI_RESET),
        }
    }
}
impl From<Colour> for i64 {
    fn from(colour: Colour) -> Self {
        colour as i64
    }
}
impl TryFrom<i64> for Colour {
    type Error = ();
    fn try_from(int: i64) -> Result<Self, ()> {
        match int {
            1 => Ok(Colour::White),
            0 => Ok(Colour::Black),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    CCW = 0,
    CW = 1,
}
impl From<Rotation> for i64 {
    fn from(rotation: Rotation) -> Self {
        rotation as i64
    }
}
impl TryFrom<i64> for Rotation {
    type Error = ();
    fn try_from(int: i64) -> Result<Self, ()> {
        match int {
            1 => Ok(Rotation::CW),
            0 => Ok(Rotation::CCW),
            _ => Err(()),
        }
    }
}

struct Robot {
    position: Point,
    direction: Direction,
    cpu: Machine,
    canvas: HashMap<Point, Colour>,
}
impl Robot {
    pub fn from_string(string: &str) -> Self {
        Robot {
            position: Point(0, 0),
            direction: Direction::Up,
            cpu: Machine::from_string(string),
            canvas: HashMap::new(),
        }
    }
    pub fn run(&mut self) {
        while !self.cpu.is_halted() {
            self.step()
        }
    }
    fn step(&mut self) {
        let input = std::iter::once(self.get().into());
        let output = self.cpu.run(input);

        assert_eq!(output.len(), 2);

        self.paint(output[0].try_into().unwrap());
        self.advance(output[1].try_into().unwrap());
    }

    fn paint(&mut self, colour: Colour) {
        self.canvas.insert(self.position, colour);
    }
    fn advance(&mut self, rotation: Rotation) {
        self.direction.rotate(rotation);
        self.position.step(self.direction);
    }
    fn get(&self) -> Colour {
        *self.canvas.get(&self.position).unwrap_or(&Colour::Black)
    }
}

pub fn part1(cpu: &str) -> usize {
    let mut robot = Robot::from_string(cpu);
    robot.run();
    robot.canvas.len()
}

pub fn part2(cpu: &str) {
    let mut robot = Robot::from_string(cpu);
    robot.paint(Colour::White);
    robot.run();

    for y in 0..6 {
        for x in 0..50 {
            let colour = robot.canvas.get(&Point(x, y)).unwrap_or(&Colour::Black);
            print!("{}", colour)
        }
        println!()
    }
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("Program Output: {:?}", part2(&buffer));
}
