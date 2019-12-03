use std::cmp::{max, min};

use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res, value};
use nom::multi::separated_list;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct WireRel {
    length: i32,
    direction: Direction,
}

#[derive(Clone, Copy, Debug)]
struct WireAbs {
    x: i32,
    y: i32,
    wire: WireRel,
}
impl WireAbs {
    fn start(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    fn end(&self) -> (i32, i32) {
        match self.wire.direction {
            Direction::Up => (self.x, self.y + self.wire.length),
            Direction::Down => (self.x, self.y - self.wire.length),
            Direction::Left => (self.x - self.wire.length, self.y),
            Direction::Right => (self.x + self.wire.length, self.y),
        }
    }
    fn is_vertical(&self) -> bool {
        match self.wire.direction {
            Direction::Down | Direction::Up => true,
            Direction::Left | Direction::Right => false,
        }
    }
    fn collides(&self, other: &WireAbs) -> Option<(i32, i32)> {
        let (horizontal, vertical) = match (self.is_vertical(), other.is_vertical()) {
            (true, true) | (false, false) => return None,
            (true, false) => (other, self),
            (false, true) => (self, other),
        };

        let collision = (vertical.x, horizontal.y);
        if between(collision.0, horizontal.start().0, horizontal.end().0)
            && between(collision.1, vertical.start().1, vertical.end().1)
        {
            Some(collision)
        } else {
            None
        }
    }
}

fn number_p(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn direction_p(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, char('U')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
        value(Direction::Down, char('D')),
    ))(input)
}

fn wire_segment_p(input: &str) -> IResult<&str, WireRel> {
    map(tuple((direction_p, number_p)), |(direction, length)| {
        WireRel { length, direction }
    })(input)
}

fn full_wire_p(input: &str) -> IResult<&str, Vec<WireRel>> {
    separated_list(char(','), wire_segment_p)(input)
}

fn wires_p(input: &str) -> IResult<&str, (Vec<WireRel>, Vec<WireRel>)> {
    map(
        tuple((full_wire_p, char('\n'), full_wire_p)),
        |(wire1, _, wire2)| (wire1, wire2),
    )(input)
}

fn into_wire_abs(wire_rels: &[WireRel]) -> Vec<WireAbs> {
    let (mut x, mut y) = (0, 0);
    let mut result = Vec::new();
    for wire in wire_rels {
        result.push(WireAbs { x, y, wire: *wire });

        match wire.direction {
            Direction::Up => y += wire.length,
            Direction::Left => x -= wire.length,
            Direction::Down => y -= wire.length,
            Direction::Right => x += wire.length,
        }
    }
    result
}

fn between(num: i32, bound1: i32, bound2: i32) -> bool {
    num >= min(bound1, bound2) && num <= max(bound1, bound2)
}

fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn part1(buffer: &str) -> i32 {
    let (wire1, wire2) = wires_p(&buffer).unwrap().1;
    let (wire1, wire2) = (into_wire_abs(&wire1), into_wire_abs(&wire2));

    let mut min_so_far = i32::max_value();
    for wire_seg1 in &wire1 {
        for wire_seg2 in &wire2 {
            if let Some((x, y)) = wire_seg1.collides(&wire_seg2) {
                if x != 0 || y != 0 {
                    min_so_far = min(min_so_far, x.abs() + y.abs())
                }
            };
        }
    }
    min_so_far
}

pub fn part2(buffer: &str) -> i32 {
    let (wire1, wire2) = wires_p(&buffer).unwrap().1;
    let (wire1, wire2) = (into_wire_abs(&wire1), into_wire_abs(&wire2));

    let mut min_so_far = i32::max_value();
    let mut wire_1_dist = 0;

    for wire_seg1 in &wire1 {
        wire_1_dist += wire_seg1.wire.length;

        let mut wire_2_dist = 0;
        for wire_seg2 in &wire2 {
            wire_2_dist += wire_seg2.wire.length;

            if let Some(collision) = wire_seg1.collides(&wire_seg2) {
                if collision.0 != 0 || collision.1 != 0 {
                    let total_dist = wire_1_dist + wire_2_dist
                        - dist(collision, wire_seg1.end())
                        - dist(collision, wire_seg2.end());
                    min_so_far = min(min_so_far, total_dist)
                }
            };
        }
    }
    min_so_far
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    }
    #[test]
    fn example2() {
        assert_eq!(
            part1(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                 U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                 U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn example4() {
        assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
    }
    #[test]
    fn example5() {
        assert_eq!(
            part2(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                 U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
    }
    #[test]
    fn example6() {
        assert_eq!(
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                 U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
