use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use ncurses as nc;
use ncurses::constants as ncc;

use crate::day05::Machine;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Empty, Wall, Block, Paddle, Ball,
}
impl From<Pixel> for char {
    fn from(pixel: Pixel) -> char {
        match pixel {
            Pixel::Empty => ' ',
            Pixel::Wall => '#',
            Pixel::Block => 'x',
            Pixel::Paddle => '_',
            Pixel::Ball => 'o',
        }
    }
}
impl TryFrom<i64> for Pixel {
    type Error = ();
    fn try_from(data: i64) -> Result<Self, ()> {
        match data {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(()),
        }
    }
}

struct Screen {
    buffer: HashMap<(i64, i64), Pixel>,
}
impl Screen {
    fn new(output: Vec<i64>) -> Self {
        let mut buffer = HashMap::new();
        for entry in output.chunks_exact(3) {
            match entry {
                [x, y, data] => buffer
                    .insert((*x, *y), Pixel::try_from(*data).unwrap()),
                _ => unreachable!()
            };
        }
        Screen { buffer }
    }
    fn block_count(&self) -> usize {
        self.buffer.values().filter(|v| **v == Pixel::Block).count()
    }
}

pub fn part1(mut machine: Machine) -> usize {
    let output = machine.run(std::iter::empty());
    let screen = Screen::new(output);
    screen.block_count()
}

const KEY_Q: i32 = 'q' as i32;
const KEY_SPACE: i32 = ' ' as i32;
const KEY_S: i32 = 's' as i32;
const KEY_R: i32 = 'r' as i32;

const DAY_13_MACHINE: u32 = 13;

pub fn part2(mut machine: Machine) {

    machine.mem_set(0, 2);

    nc::initscr();
    nc::cbreak();
    nc::keypad(nc::stdscr(), true);
    nc::noecho();

    loop {
        let input = nc::getch();
        let mut joystick = 0;

        match input {
            ncc::KEY_LEFT => joystick = -1,
            ncc::KEY_RIGHT => joystick = 1,
            KEY_SPACE => joystick = 0,
            KEY_Q => break,
            KEY_S => machine.save(DAY_13_MACHINE),
            KEY_R => machine = Machine::restore(DAY_13_MACHINE),
            _ => continue,
        };

        let output = machine.run(std::iter::once(joystick));

        for entry in output.chunks_exact(3) {
            match entry {
                [-1, 0, score] => {
                    nc::mvprintw(26, 0, format!("Score: {}", score).as_str());
                }
                [x, y, data] => {
                    let c: char = Pixel::try_from(*data).unwrap().into();
                    let x = i32::try_from(*x).unwrap();
                    let y = i32::try_from(*y).unwrap();
                    nc::mvaddch(y, x, c.into());
                }
                _ => unreachable!()
            };
        }
        nc::refresh();
    }
    nc::endwin();
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day13.txt")
        .expect("Something went wrong reading the file");

    let machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part2(machine));
}
