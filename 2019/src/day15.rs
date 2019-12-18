use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use ncurses as nc;

use crate::day05::Machine;
use crate::day11::{Direction, Point};

enum Cell {
    Empty,
    Wall,
    Cylinder,
}
impl TryFrom<i64> for Cell {
    type Error = ();
    fn try_from(raw: i64) -> Result<Self, ()> {
        match raw {
            0 => Ok(Self::Wall),
            1 => Ok(Self::Empty),
            2 => Ok(Self::Cylinder),
            _ => Err(()),
        }
    }
}

fn test_direction(machine: &mut Machine, position: Point, direction: Direction) {
    let mut new_pos = position;
    new_pos.step(direction);

    let input = std::iter::once(direction.to_joystick_day_11());
    let opposite = std::iter::once(direction.opposite().to_joystick_day_11());
    match machine.run(input)[0].try_into().unwrap() {
        Cell::Wall => {
            nc::mvaddch(new_pos.y(), new_pos.x(), '#'.into());
        }
        Cell::Empty => {
            machine.run(opposite);
        }
        Cell::Cylinder => {
            machine.run(opposite);
        }
    }
}

const KEY_Q: i32 = 'q' as i32;
pub fn part1(mut machine: Machine) {
    nc::initscr();
    nc::cbreak();
    nc::keypad(nc::stdscr(), true);
    nc::noecho();

    let mut distances = HashMap::new();
    let mut current_distance = 0;

    let mut oxygen_distances = HashMap::new();
    let mut oxygen_distance = None;
    let mut max_oxygen_distance = 0;

    let mut position = Point(25, 25);

    distances.insert(position, 0);

    loop {
        let direction = match nc::getch() {
            nc::KEY_UP => Direction::Up,
            nc::KEY_LEFT => Direction::Left,
            nc::KEY_RIGHT => Direction::Right,
            nc::KEY_DOWN => Direction::Down,
            KEY_Q => break,
            _ => continue,
        };

        if oxygen_distance.is_some() {
            nc::mvaddch(position.y(), position.x(), 'x'.into());
        } else {
            nc::mvaddch(position.y(), position.x(), '.'.into());
        }

        let input = std::iter::once(direction.to_joystick_day_11());
        match machine.run(input)[0].try_into().unwrap() {
            Cell::Wall => {
                let mut new_point = position;
                new_point.step(direction);
                nc::mvaddch(new_point.y(), new_point.x(), '#'.into());
            }
            Cell::Empty => {
                position.step(direction);
            }
            Cell::Cylinder => {
                position.step(direction);
                oxygen_distances.insert(position, 0);
                oxygen_distance = Some(0);
            }
        }

        test_direction(&mut machine, position, Direction::Up);
        test_direction(&mut machine, position, Direction::Left);
        test_direction(&mut machine, position, Direction::Down);
        test_direction(&mut machine, position, Direction::Right);

        nc::mvaddch(position.y(), position.x(), '@'.into());

        current_distance = *distances.entry(position).or_insert(current_distance + 1);
        oxygen_distance = oxygen_distance.map(|old_distance| {
            let distance = oxygen_distances.entry(position).or_insert(old_distance + 1);
            max_oxygen_distance = max_oxygen_distance.max(*distance);
            nc::mvprintw(
                3,
                1,
                format!("Current Oxygen Distance: {}", distance).as_str(),
            );
            *distance
        });

        nc::mvprintw(1, 1, format!("Distance: {}", current_distance).as_str());
        nc::mvprintw(
            2,
            1,
            format!("Max Oxygen Distance: {}", max_oxygen_distance).as_str(),
        );

        nc::refresh();
    }
    nc::endwin();
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day15.txt")
        .expect("Something went wrong reading the file");

    let machine = Machine::from_string(&buffer);

    println!("Program Output: {:?}", part1(machine));
}
