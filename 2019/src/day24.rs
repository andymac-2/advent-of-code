use std::collections::{HashSet, HashMap};
use std::convert::TryFrom;
use crate::parsers::{ParseResult, byte};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Bug = 1,
    Empty = 0,
}
impl Cell {
    fn bug(s: &[u8]) -> ParseResult<Cell> {
        let (s, _) = byte(b'#')(s)?;
        Some((s, Cell::Bug))
    }
    fn empty(s: &[u8]) -> ParseResult<Cell> {
        let (s, _) = byte(b'.')(s)?;
        Some((s, Cell::Empty))
    }
    fn cell(s: &[u8]) -> ParseResult<Cell> {
        Cell::bug(s).or_else(|| Cell::empty(s))
    }
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Bug => write!(f, "#"),
            Cell::Empty => write!(f, "."),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Eris(u32);
impl Default for Eris {
    fn default() -> Self {
        Eris(0)
    }
}
impl Eris {
    const WIDTH: i32 = 5;
    const HEIGHT: i32 = 5;

    fn bug_count(self, x: i32, y: i32) -> u32 {
        let mut count = 0;
        count += self.get(x + 1, y) as u32;
        count += self.get(x - 1, y) as u32;
        count += self.get(x, y + 1) as u32;
        count += self.get(x, y - 1) as u32;
        count
    }
    fn index(x: i32, y: i32) -> i32 {
        (y * Eris::WIDTH) + x
    }
    fn get(self, x: i32, y: i32) -> Cell {
        if x < 0 || y < 0 || x >= Eris::WIDTH || y >= Eris::HEIGHT {
            return Cell::Empty;
        }

        if self.0 & (1 << Eris::index(x, y)) == 0 {
            return Cell::Empty;
        }

        Cell::Bug
    }
    fn set(&mut self, x: i32, y: i32, cell: Cell) {
        assert!(x < Eris::WIDTH && x >= 0);
        assert!(y < Eris::HEIGHT && y >= 0);

        match cell {
            Cell::Bug => {
                self.0 |= 1 << Eris::index(x, y)
            }
            Cell::Empty => {
                self.0 &= !(1 << Eris::index(x, y))
            }
        }
    }
    fn next(self) -> Self {
        let mut next = Self::default();
        for y in 0..5 {
            for x in 0..5 {
                let count = self.bug_count(x, y);
                let cell = self.get(x, y);
                let not_dies = cell == Cell::Bug && count == 1;
                let infested = cell == Cell::Empty && (count == 1 || count == 2);

                if not_dies || infested {
                    next.set(x, y, Cell::Bug);
                }
            }
        }
        next
    }
    fn eris(mut s: &[u8]) -> ParseResult<Eris> {
        let mut eris = Self::default();
        for y in 0..5 {
            for x in 0..5 {
                let (new_s, cell) = Cell::cell(s)?;
                s = new_s;
                eris.set(x, y, cell);
            }

            let (new_s, _) = byte(b'\n')(s)?;
            s = new_s;
        }
        Some((s, eris))
    }
    fn value(self) -> u32 {
        self.0
    }
}
impl std::fmt::Display for Eris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct ErisRec {
    eris: HashSet<(i32, u8, u8)>
}
#[derive(Debug, Copy, Clone)]
pub struct ErisCentreNotEmpty;
impl std::convert::TryFrom<Eris> for ErisRec {
    type Error = ErisCentreNotEmpty;
    fn try_from(eris: Eris) -> Result<Self, ErisCentreNotEmpty> {
        let mut map = HashSet::new();
        for x in 0..5 {
            for y in 0..5 {
                if eris.get(i32::from(x), i32::from(y)) == Cell::Bug {
                    if x == 2 && y == 2 {
                        return Err(ErisCentreNotEmpty);
                    }
                    map.insert((ErisRec::INITIAL_LEVEL, x, y));
                }
            }
        }

        Ok(ErisRec { eris: map })
    }
}
impl ErisRec {
    const INITIAL_LEVEL: i32 = 0;
    const WIDTH: u8 = 5;
    const HEIGHT: u8 = 5;
    fn adjacents(&self) -> HashMap<(i32, u8, u8), u8> {
        let mut adjacents = HashMap::new();
        let mut increment = |level, x, y| {
            let count = adjacents.entry((level, x, y)).or_default();
            *count += 1;
        };

        for (level, x, y) in self.eris.iter().copied() {
            assert!(x < ErisRec::WIDTH);
            assert!(y < ErisRec::HEIGHT);
            assert!(x != 2 || y != 2);

            // left
            if x == 0 {
                increment(level - 1, 1, 2);
            }
            else if x == 3 && y == 2 {
                for iy in 0..5 {
                    increment(level + 1, 4, iy);
                }
            }
            else {
                increment(level, x - 1, y);
            }

            // right
            if x == 4 {
                increment(level - 1, 3, 2);
            }
            else if x == 1 && y == 2 {
                for iy in 0..5 {
                    increment(level + 1, 0, iy);
                }
            }
            else {
                increment(level, x + 1, y);
            }

            // up
            if y == 0 {
                increment(level - 1, 2, 1);
            }
            else if y == 3 && x == 2 {
                for ix in 0..5 {
                    increment(level + 1, ix, 4);
                }
            }
            else {
                increment(level, x, y - 1);
            }

            // down
            if y == 4 {
                increment(level - 1, 2, 3);
            }
            else if y == 1 && x == 2 {
                for ix in 0..5 {
                    increment(level + 1, ix, 0);
                }
            }
            else {
                increment(level, x, y + 1);
            }
        }
        adjacents
    }
    fn step(&self) -> Self {
        let mut next = HashSet::new();
        for (index, count) in self.adjacents() {
            let is_bug = self.eris.contains(&index);

            let not_dies = is_bug && count == 1;
            let infested = !is_bug && (count == 1 || count == 2);
            if not_dies || infested {
                next.insert(index);
            }
        }
        ErisRec {
            eris: next
        }
    }
    fn size(&self) -> usize {
        self.eris.len()
    }
}

pub fn part1(mut eris: Eris) -> u32 {
    let mut seen = HashSet::new();
    loop {
        println!("{}", eris);
        if !seen.insert(eris) {
            break eris.value()
        }
        eris = eris.next();
    }
}

pub fn part2(eris: Eris) -> usize {
    let mut eris_rec = ErisRec::try_from(eris).unwrap();
    for _ in 0..200 {
        eris_rec = eris_rec.step();
    }
    eris_rec.size()
}


pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day24.txt").unwrap();
    let eris = Eris::eris(buffer.as_bytes()).unwrap().1;
    println!(
        "Program Output: {:?}",
        part2(eris)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        let buffer = std::fs::read_to_string("./inputs/day24ex1.txt").unwrap();
        let eris = Eris::eris(buffer.as_bytes()).unwrap().1;
        assert_eq!(part1(eris), 2_129_920);
    }
    #[test]
    fn ex2() {
        let buffer = std::fs::read_to_string("./inputs/day24ex1.txt").unwrap();
        let eris = Eris::eris(buffer.as_bytes()).unwrap().1;
        let mut eris_rec = ErisRec::try_from(eris).unwrap();

        for _ in 0..10 {
            eris_rec = eris_rec.step();
        }
        assert_eq!(eris_rec.size(), 99);
    }
}