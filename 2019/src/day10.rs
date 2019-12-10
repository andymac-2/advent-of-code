use std::io;
use std::io::Read;
use std::convert::TryInto;
use std::collections::{HashMap, HashSet};

use crate::parsers::*;

fn line_p(s: &[u8]) -> ParseResult<&[u8]> {
    take_while1_p(|c| c == b'#' || c == b'.')(s)
}

fn arena_p(s: &[u8]) -> ParseResult<Arena> {
    map(
        sep_by(line_p, byte(b'\n')),
        |buffer| Arena(buffer)
    )(s)
}

fn gcd (a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

pub struct Arena<'a>(Vec<&'a[u8]>);
impl<'a> Arena<'a> {
    fn iter(&self) -> Iter {
        Iter::new(self.0.as_ref())
    }
}

struct Iter<'a> {
    buffer: &'a [&'a [u8]],
    x: usize,
    y: usize,
}
impl<'a> Iter<'a> {
    fn new(buffer: &'a [&'a [u8]]) -> Self {
        Iter {buffer, x: 0, y: 0,}
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        while self.y < self.buffer.len() {
            let line = self.buffer[self.y];

            while self.x < line.len() {
                if line[self.x] == b'#' {
                    let x = self.x.try_into().unwrap();
                    let y = self.y.try_into().unwrap();
                    self.x += 1;

                    return Some(Point::new(x, y));
                }
                self.x += 1;
            }

            self.y += 1;
            self.x = 0;
        }
        None
    }
}

#[must_use]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl std::ops::Div<i32> for Point {
    type Output = Point;
    fn div(self, rhs: i32) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {x, y}
    }
    fn len_sq(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
    fn normalize (self) -> Self {
        let gcd = gcd(self.x, self.y);
        if gcd == 0 {
            self
        } else {
            self / gcd
        }
    }
    // clockwise from north
    fn angle(self) -> f64 {
        -f64::from(self.x).atan2(f64::from(self.y))
    }
}

pub fn part1(arena: Arena) {
    let mut buffer: HashMap<Point, HashSet<Point>> = HashMap::new();
    for p1 in arena.iter() {
        for p2 in arena.iter() {
            let direction = (p2 - p1).normalize();
            let asteroid_set = buffer.entry(p1).or_insert_with(HashSet::new);
            asteroid_set.insert(direction);
        }
    }

    let max_asteroid = buffer
        .into_iter()
        .max_by_key(|(_k, v)| v.len())
        .map(|(k, v)| (k, v.len() - 1, v));
    
    println!("{:?}", max_asteroid);
}

pub fn part2(arena: Arena) {
    let mut buffer: HashMap<Point, HashMap<Point, Point>> = HashMap::new();
    for p1 in arena.iter() {
        for p2 in arena.iter() {
            if p1 == p2 {
                continue;
            }

            let direction = (p2 - p1).normalize();
            let asteroid_set = buffer.entry(p1).or_insert_with(HashMap::new);

            let closest = asteroid_set.entry(direction).or_insert(p2);
            if (p2 - p1).len_sq() < (*closest - p1).len_sq() {
                *closest = p2;
            }
        }
    }

    let (max_asteroid, surrounds) = buffer
        .into_iter()
        .max_by_key(|(_k, v)| v.len())
        .unwrap();

    let mut surrounding: Vec<_> = surrounds.into_iter().collect();
    surrounding.sort_by(|(d1, _), (d2, _)| {
        d1.angle().partial_cmp(&d2.angle()).unwrap()
    });
    
    println!("{:?}", surrounding[200 - 1].1);
}

pub fn start () {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let arena = arena_p(buffer.as_bytes()).unwrap().1;

    println!("Program Output: {:?}", part2(arena));
}