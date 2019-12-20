use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::day11::Direction;

#[must_use]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(usize, usize);
impl Point {
    pub fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Point(self.0, self.1 - 1),
            Direction::Down => Point(self.0, self.1 + 1),
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Wall,
    Empty,
    OuterPortal(u8, u8),
    InnerPortal(u8, u8),
}
impl Cell {
    fn is_wall(self) -> bool {
        if let Cell::Wall = self {
            return true;
        }
        false
    }
}
const ANSI_WHITE: &str = "\x1b[47m";
const ANSI_BLACK: &str = "\x1b[40m";
const ANSI_RED: &str = "\x1b[41m";
const ANSI_GREEN: &str = "\x1b[42m";
const ANSI_RESET: &str = "\x1b[0m";
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "{}##{}", ANSI_WHITE, ANSI_RESET),
            Cell::Empty => write!(f, "{}  {}", ANSI_BLACK, ANSI_RESET),
            Cell::InnerPortal(a, b) => {
                let a = char::from(*a);
                let b = char::from(*b);
                write!(f, "{}{}{}{}", ANSI_RED, a, b, ANSI_RESET)
            }
            Cell::OuterPortal(a, b) => {
                let a = char::from(*a);
                let b = char::from(*b);
                write!(f, "{}{}{}{}", ANSI_GREEN, a, b, ANSI_RESET)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Portal(Point, Cell);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MazeState(Point, Level);
impl MazeState {
    fn position(&self) -> Point {
        self.0
    }
    fn level(&self) -> Level {
        self.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Level(u32);
impl Level {
    fn zero(self) -> bool {
        self.0 == 0
    }
    fn succ(self) -> Self {
        Level(self.0 + 1)
    }
    fn pred(self) -> Option<Self> {
        Some(Level(self.0.checked_sub(1)?))
    }
}

const MARGIN: usize = 4;
pub struct Maze {
    portals: HashMap<Cell, Point>,
    grid: Vec<Vec<Cell>>,
}
impl Maze {
    fn from_str(input: &str) -> Self {
        let mut portals = HashMap::new();
        let char_grid: Vec<_> = input.as_bytes().split(|c| *c == b'\n').collect();

        let grid = char_grid
            .iter()
            .enumerate()
            .map(|(y, &line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, _)| {
                        let cell = Maze::coord_to_cell(&char_grid, Point(x, y));

                        match cell {
                            Cell::InnerPortal(_, _) | Cell::OuterPortal(_, _) => {
                                portals.insert(cell, Point(x, y));
                            }
                            Cell::Wall | Cell::Empty => {}
                        }

                        cell
                    })
                    .collect()
            })
            .collect();

        Maze { portals, grid }
    }

    fn coord_to_cell(char_grid: &[&[u8]], position: Point) -> Cell {
        let character = char_grid[position.1][position.0];
        if character != b'.' {
            return Cell::Wall;
        }

        let mut result = Cell::Empty;

        Direction::for_each(|direction| {
            let mut first_pos = position.step(direction);
            let mut second_pos = first_pos.step(direction);

            if first_pos > second_pos {
                std::mem::swap(&mut first_pos, &mut second_pos);
            }

            let first_char = char_grid[first_pos.1][first_pos.0];
            let second_char = char_grid[second_pos.1][second_pos.0];

            if first_char.is_ascii_alphabetic() && second_char.is_ascii_alphabetic() {
                assert_eq!(result, Cell::Empty);

                let y = position.1;
                let x = position.0;

                if y > MARGIN
                    && y < char_grid.len() - MARGIN
                    && x > MARGIN
                    && x < char_grid[y].len() - MARGIN
                {
                    result = Cell::InnerPortal(first_char, second_char);
                } else {
                    result = Cell::OuterPortal(first_char, second_char);
                }
            }
        });
        result
    }

    fn width(&self) -> usize {
        self.grid[2].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get_cell(&self, index: Point) -> Cell {
        self.grid.get(index.1).map_or(Cell::Wall, |row| {
            row.get(index.0).map_or(Cell::Wall, |cell| *cell)
        })
    }

    fn get_cell_mut(&mut self, index: Point) -> Option<&mut Cell> {
        self.grid.get_mut(index.1)?.get_mut(index.0)
    }

    fn wall_count(&self, index: Point) -> u32 {
        let mut wall_count = 0;

        Direction::for_each(|direction| {
            if self.get_cell(index.step(direction)).is_wall() {
                wall_count += 1;
            }
        });

        wall_count
    }

    fn remove_dead_ends(&mut self) {
        let mut cells_to_go = Vec::with_capacity(self.width() * self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                cells_to_go.push(Point(x, y));
            }
        }

        while let Some(index) = cells_to_go.pop() {
            match self.get_cell(index) {
                Cell::Wall | Cell::InnerPortal(_, _) | Cell::OuterPortal(_, _) => continue,
                Cell::Empty => {
                    if self.wall_count(index) < 3 {
                        continue;
                    }
                    if let Some(cell) = self.get_cell_mut(index) {
                        *cell = Cell::Wall;
                    }

                    Direction::for_each(|direction| {
                        cells_to_go.push(index.step(direction));
                    });
                }
            }
        }
    }

    fn find_portal_end(&self, cell: Cell) -> Option<Point> {
        match cell {
            Cell::Wall | Cell::Empty => None,
            Cell::InnerPortal(f, s) => self.portals.get(&Cell::OuterPortal(f, s)).copied(),
            Cell::OuterPortal(f, s) => self.portals.get(&Cell::InnerPortal(f, s)).copied(),
        }
    }

    fn get_aa(&self) -> Point {
        *self.portals.get(&Cell::OuterPortal(b'A', b'A')).unwrap()
    }

    fn aa_to_zz(&self) -> u32 {
        let start = self.get_aa();

        let mut best_so_far = HashMap::new();
        let mut search_space = vec![(start, 0)];
        let mut current_best_solve = u32::max_value();

        while let Some((position, steps)) = search_space.pop() {
            let cell = self.get_cell(position);
            if let Cell::Wall = cell {
                continue;
            }

            let current_best = best_so_far.entry(position).or_insert_with(u32::max_value);
            if steps >= *current_best {
                continue;
            }
            *current_best = steps;

            match cell {
                Cell::InnerPortal(f, s) | Cell::OuterPortal(f, s) => {
                    if f == b'Z' && s == b'Z' {
                        current_best_solve = current_best_solve.min(steps);
                    }
                    if let Some(other_end) = self.find_portal_end(cell) {
                        search_space.push((other_end, steps + 1));
                    }
                }
                Cell::Wall | Cell::Empty => {}
            }

            Direction::for_each(|direction| {
                search_space.push((position.step(direction), steps + 1));
            })
        }

        current_best_solve
    }

    fn aa_to_zz_rec(&self) -> Option<u32> {
        let start = MazeState(self.get_aa(), Level(0));

        let mut best_so_far = HashMap::new();
        let mut search_space = BinaryHeap::new();
        search_space.push(Reverse((0, start)));

        while let Some(Reverse((steps, state))) = search_space.pop() {
            let cell = self.get_cell(state.position());
            if let Cell::Wall = cell {
                continue;
            }

            let current_best = best_so_far.entry(state).or_insert_with(u32::max_value);
            if steps >= *current_best {
                continue;
            }
            *current_best = steps;

            match cell {
                Cell::InnerPortal(_, _) => {
                    if let Some(other_end) = self.find_portal_end(cell) {
                        let next_state = MazeState(other_end, state.level().succ());
                        search_space.push(Reverse((steps + 1, next_state)));
                    }
                }
                Cell::OuterPortal(f, s) => {
                    if f == b'Z' && s == b'Z' && state.level().zero() {
                        return Some(steps);
                    }
                    if let Some(new_level) = state.level().pred() {
                        if let Some(other_end) = self.find_portal_end(cell) {
                            let next_state = MazeState(other_end, new_level);
                            search_space.push(Reverse((steps + 1, next_state)));
                        }
                    }
                }
                Cell::Wall | Cell::Empty => {}
            }

            Direction::for_each(|direction| {
                let new_state = MazeState(state.position().step(direction), state.level());
                search_space.push(Reverse((steps + 1, new_state)));
            })
        }
        None
    }
}
impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row.as_slice().iter() {
                write!(f, "{}", cell)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub fn part1(mut maze: Maze) -> u32 {
    maze.remove_dead_ends();
    println!("{}", maze);
    maze.aa_to_zz()
}

pub fn part2(mut maze: Maze) -> Option<u32> {
    maze.remove_dead_ends();
    println!("{}", maze);
    maze.aa_to_zz_rec()
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day20.txt")
        .expect("Something went wrong reading the file");

    let maze = Maze::from_str(&buffer);
    println!("Program Output: {:?}", part2(maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let buffer = std::fs::read_to_string("./inputs/day20ex1.txt").unwrap();

        let maze = Maze::from_str(&buffer);
        assert_eq!(part1(maze), 23);
    }

    #[test]
    fn part1_ex2() {
        let buffer = std::fs::read_to_string("./inputs/day20ex2.txt").unwrap();

        let maze = Maze::from_str(&buffer);
        assert_eq!(part1(maze), 58);
    }

    #[test]
    fn part2_ex1() {
        let buffer = std::fs::read_to_string("./inputs/day20ex1.txt").unwrap();

        let maze = Maze::from_str(&buffer);
        assert_eq!(part2(maze), Some(26));
    }
}
