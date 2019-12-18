use std::collections::HashMap;

use crate::day11::Direction;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
    Player,
    Key(u8),
    Door(u8),
}
impl Cell {
    fn is_wall(self) -> bool {
        if let Cell::Wall = self {
            return true;
        }
        false
    }
    fn is_player(self) -> bool {
        if let Cell::Player = self {
            return true;
        }
        false
    }
    fn is_key(self) -> bool {
        if let Cell::Key(_) = self {
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
            Cell::Wall => write!(f, "{}  {}", ANSI_WHITE, ANSI_RESET),
            Cell::Empty => write!(f, "{}  {}", ANSI_BLACK, ANSI_RESET),
            Cell::Player => write!(f, " @"),
            Cell::Key(c) => {
                let name: char = c.clone().into();
                write!(f, "{} {}{}", ANSI_RED, name, ANSI_RESET)
            }
            Cell::Door(c) => {
                let mut name: char = c.clone().into();
                name.make_ascii_uppercase();
                write!(f, "{} {}{}", ANSI_GREEN, name, ANSI_RESET)
            }
        }
    }
}
impl From<u8> for Cell {
    fn from(c: u8) -> Self {
        match c {
            b'#' => Cell::Wall,
            b'.' => Cell::Empty,
            b'@' => Cell::Player,
            mut c if c.is_ascii_uppercase() => {
                c.make_ascii_lowercase();
                Cell::Door(c)
            }
            c if c.is_ascii_lowercase() => Cell::Key(c),
            _ => Cell::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Keys {
    keys: u32,
}
impl Default for Keys {
    fn default() -> Self {
        Keys { keys: 0 }
    }
}
impl Keys {
    fn has_key(self, key: u8) -> bool {
        let index = key - b'a';
        (self.keys & (1 << index)) != 0
    }
    fn add_key(&mut self, key: u8) {
        let index = key - b'a';
        self.keys |= 1 << index
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct MazeState {
    position: Point,
    keys: Keys,
}
impl MazeState {
    fn new(position: Point, keys: Keys) -> Self {
        MazeState { position, keys }
    }
}

pub struct Maze {
    grid: Vec<Vec<Cell>>,
}
impl Maze {
    fn from_str(input: &str) -> Self {
        let grid: Vec<_> = input
            .trim()
            .as_bytes()
            .split(|c| *c == b'\n')
            .map(|line| line.iter().map(|c| Cell::from(*c)).collect())
            .collect();

        Maze { grid }
    }

    fn get_players(&self) -> Vec<Point> {
        let mut players = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.as_slice().iter().enumerate() {
                if cell.is_player() {
                    players.push(Point::new(x, y));
                }
            }
        }
        players
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
    fn get_cell(&self, index: Point) -> Cell {
        self.grid.get(index.y).map_or(Cell::Wall, |row| {
            row.get(index.x).map_or(Cell::Wall, |cell| *cell)
        })
    }

    fn get_cell_mut(&mut self, index: Point) -> Option<&mut Cell> {
        self.grid.get_mut(index.y)?.get_mut(index.x)
    }

    fn wall_count(&self, index: Point) -> u32 {
        let mut wall_count = 0;

        Direction::for_each(|direction| {
            let mut new_index = index;
            new_index.step(direction);

            if self.get_cell(new_index).is_wall() {
                wall_count += 1;
            }
        });

        wall_count
    }

    fn remove_dead_ends(&mut self) {
        let mut cells_to_go = Vec::with_capacity(self.width() * self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                cells_to_go.push(Point::new(x, y));
            }
        }

        while let Some(index) = cells_to_go.pop() {
            match self.get_cell(index) {
                Cell::Wall | Cell::Key(_) | Cell::Player => continue,
                Cell::Door(_) | Cell::Empty => {
                    if self.wall_count(index) < 3 {
                        continue;
                    }
                    if let Some(cell) = self.get_cell_mut(index) {
                        *cell = Cell::Wall;
                    }

                    Direction::for_each(|direction| {
                        let mut new_index = index;
                        new_index.step(direction);

                        cells_to_go.push(new_index);
                    });
                }
            }
        }
    }

    fn naive_solve(&self) -> (Keys, u32) {
        let initial_state = MazeState::new(self.get_players()[0], Keys::default());
        let mut best_so_far = HashMap::new();

        let mut best_keys = HashMap::new();
        let mut search_space = vec![(initial_state, 0)];

        while let Some((mut state, steps)) = search_space.pop() {
            let cell = self.get_cell(state.position);
            match cell {
                Cell::Wall => continue,
                Cell::Door(key) => {
                    if !state.keys.has_key(key) {
                        continue;
                    }
                }
                Cell::Player | Cell::Empty => {}
                Cell::Key(key) => {
                    state.keys.add_key(key);
                }
            }

            let current_best = best_so_far.entry(state).or_insert_with(u32::max_value);
            if steps >= *current_best {
                continue;
            }
            *current_best = steps;

            if cell.is_key() {
                let best_key = best_keys.entry(state.keys).or_insert_with(u32::max_value);
                *best_key = steps.min(*best_key);
            }

            Direction::for_each(|direction| {
                let mut position = state.position;
                position.step(direction);

                let new_state = MazeState::new(position, state.keys);
                search_space.push((new_state, steps + 1));
            })
        }

        // could be replaced by maximim by
        best_keys
            .into_iter()
            .max_by_key(|(keys, _steps)| *keys)
            .unwrap()
    }

    fn co_solve(&self) -> usize {
        let mut total_area = 0;
        let mut longest_paths = 0;

        for player in self.get_players() {
            let mut best_so_far = HashMap::new();
            let mut search_space = vec![(player, 0)];

            while let Some((position, steps)) = search_space.pop() {
                let cell = self.get_cell(position);
                if let Cell::Wall = cell {
                    continue;
                }

                let current_best = best_so_far.entry(position).or_insert_with(usize::max_value);
                if steps >= *current_best {
                    continue;
                }
                *current_best = steps;

                Direction::for_each(|direction| {
                    let mut new_position = position;
                    new_position.step(direction);

                    search_space.push((new_position, steps + 1));
                })
            }
            total_area += best_so_far.len();
            longest_paths += best_so_far.values().copied().max().unwrap();
        }

        // subtract 2 for each player.
        total_area * 2 - longest_paths - 8
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

pub fn part1() {
    let buffer = std::fs::read_to_string("./inputs/day18.txt")
        .expect("Something went wrong reading the file");

    let mut maze = Maze::from_str(&buffer);
    maze.remove_dead_ends();
    println!("{}", maze);
    println!("{:?}", maze.naive_solve());
}

pub fn part2() {
    let buffer = std::fs::read_to_string("./inputs/day18_2.txt")
        .expect("Something went wrong reading the file");

    let mut maze = Maze::from_str(&buffer);
    maze.remove_dead_ends();
    println!("{}", maze);
    println!("{}", maze.co_solve());
}

pub fn start() {
    println!("Program Output: {:?}", part2());
}
