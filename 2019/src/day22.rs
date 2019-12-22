use crate::parsers::{byte, chunk, optional, sep_by, take_while_p, ParseResult};

fn positive_number(s: &[u8]) -> ParseResult<i128> {
    let (s, num_str) = take_while_p(|c| c.is_ascii_digit())(s)?;
    let number: i128 = std::str::from_utf8(num_str).unwrap().parse().unwrap();
    Some((s, number))
}
fn number(s: &[u8]) -> ParseResult<i128> {
    let (s, opt_sign) = optional(byte(b'-'))(s)?;
    let (s, mut number) = positive_number(s)?;

    if opt_sign.is_some() {
        number = -number;
    }
    Some((s, number))
}
fn stack(s: &[u8]) -> ParseResult<Shuffle> {
    let (s, _) = chunk(b"deal into new stack")(s)?;
    Some((s, Shuffle::Stack))
}
fn increment(s: &[u8]) -> ParseResult<Shuffle> {
    let (s, _) = chunk(b"deal with increment ")(s)?;
    let (s, num) = positive_number(s)?;
    Some((s, Shuffle::Increment(num)))
}
fn cut(s: &[u8]) -> ParseResult<Shuffle> {
    let (s, _) = chunk(b"cut ")(s)?;
    let (s, num) = number(s)?;
    Some((s, Shuffle::Cut(num)))
}
fn shuffle(s: &[u8]) -> ParseResult<Shuffle> {
    stack(s).or_else(|| increment(s)).or_else(|| cut(s))
}
fn shuffles(s: &[u8]) -> ParseResult<Vec<Shuffle>> {
    sep_by(shuffle, byte(b'\n'))(s)
}

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = gcd_extended(b % a, a);

    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

fn modular_div(num: i128, denom: i128, modulus: i128) -> i128 {
    let (gcd, inverse, _) = gcd_extended(denom, modulus);
    assert!(gcd == 1);
    (num * inverse).rem_euclid(modulus)
}

#[derive(Debug, Clone)]
pub struct Deck {
    len: i128,
    shuffles: Vec<Shuffle>,
}
impl Deck {
    fn from_str(len: i128, s: &str) -> Self {
        Deck {
            len,
            shuffles: shuffles(s.as_bytes()).unwrap().1,
        }
    }
    fn new(len: i128) -> Self {
        Deck {
            len,
            shuffles: Vec::new(),
        }
    }
    fn shuffle(&mut self, shuffle: Shuffle) {
        self.shuffles.push(shuffle)
    }
    fn get(&self, mut position: i128) -> i128 {
        for shuffle in self.shuffles.iter().rev().copied() {
            assert!(position >= 0 && position < self.len);
            match shuffle {
                Shuffle::Stack => position = self.len - position - 1,
                Shuffle::Cut(n) => position = (position + n).rem_euclid(self.len),
                Shuffle::Increment(n) => position = modular_div(position, n, self.len),
            }
        }
        position
    }
    fn position_of_card(&self, mut card: i128) -> i128 {
        for shuffle in self.shuffles.iter().copied() {
            assert!(card >= 0 && card < self.len);
            match shuffle {
                Shuffle::Stack => card = self.len - card - 1,
                Shuffle::Cut(n) => card = (card - n).rem_euclid(self.len),
                Shuffle::Increment(n) => card = (card * n).rem_euclid(self.len),
            }
        }
        card
    }
}

#[derive(Clone, Copy, Debug)]
enum Shuffle {
    Stack,
    Cut(i128),
    Increment(i128),
}

pub fn part1(s: &str) -> i128 {
    let deck = Deck::from_str(10_007, s);
    deck.position_of_card(2019)
}

pub fn part2(mut shuffles: i128, mut position: i128, deck: &Deck) -> i128 {
    let cards = deck.len;
    let forward_b = deck.position_of_card(0);
    let forward_m = deck.position_of_card(1) - forward_b;
    let mut m = modular_div(1, forward_m, cards);
    let mut b = ((cards - forward_b) * m) % cards;

    while shuffles != 0 {
        if shuffles & 1 != 0 {
            position = (m * position + b) % cards;
        }
        shuffles >>= 1;
        b = (m * b + b) % cards;
        m = (m * m) % cards;
    }

    position
}

const STARTING_POSITION: i128 = 2020;
const NUM_CARDS: i128 = 119_315_717_514_047;
const SHUFFLES: i128 = 101_741_582_076_661;

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day22.txt").unwrap();
    let deck = Deck::from_str(NUM_CARDS, &buffer);
    println!(
        "Program Output: {:?}",
        part2(SHUFFLES, STARTING_POSITION, &deck)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stack_shuffle() {
        let mut deck = Deck::new(10);
        for x in 0..10 {
            assert_eq!(x, deck.get(x));
        }

        deck.shuffle(Shuffle::Stack);
        for x in 0..10 {
            assert_eq!(x, deck.get(9 - x));
        }
    }
    #[test]
    fn ex1() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Increment(7));
        deck.shuffle(Shuffle::Stack);
        deck.shuffle(Shuffle::Stack);

        for (ix, x) in [0, 3, 6, 9, 2, 5, 8, 1, 4, 7].iter().enumerate() {
            assert_eq!(*x, deck.get(ix as i128));
        }
    }
    #[test]
    fn ex2() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(6));
        deck.shuffle(Shuffle::Increment(7));
        deck.shuffle(Shuffle::Stack);

        for (ix, x) in [3, 0, 7, 4, 1, 8, 5, 2, 9, 6].iter().enumerate() {
            assert_eq!(*x, deck.get(ix as i128));
        }
    }
    #[test]
    fn ex3() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Increment(7));
        deck.shuffle(Shuffle::Increment(9));
        deck.shuffle(Shuffle::Cut(-2));

        for (ix, x) in [6, 3, 0, 7, 4, 1, 8, 5, 2, 9].iter().enumerate() {
            assert_eq!(*x, deck.get(ix as i128));
        }
    }
    #[test]
    fn ex4() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Stack);
        deck.shuffle(Shuffle::Cut(-2));
        deck.shuffle(Shuffle::Increment(7));
        deck.shuffle(Shuffle::Cut(8));
        deck.shuffle(Shuffle::Cut(-4));
        deck.shuffle(Shuffle::Increment(7));
        deck.shuffle(Shuffle::Cut(3));
        deck.shuffle(Shuffle::Increment(9));
        deck.shuffle(Shuffle::Increment(3));
        deck.shuffle(Shuffle::Cut(-1));

        for (ix, x) in [9, 2, 5, 8, 1, 4, 7, 0, 3, 6].iter().enumerate() {
            assert_eq!(*x, deck.get(ix as i128));
        }

        assert_eq!(deck.position_of_card(8), 3);
    }
    #[test]
    fn div() {
        assert_eq!(modular_div(1, 7, 10), 3);
    }
    #[test]
    fn fast_shuffle() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Stack);
        deck.shuffle(Shuffle::Cut(-2));
        deck.shuffle(Shuffle::Increment(7));
        deck.shuffle(Shuffle::Cut(8));
        deck.shuffle(Shuffle::Cut(-4));
        deck.shuffle(Shuffle::Increment(7));
        deck.shuffle(Shuffle::Cut(3));
        deck.shuffle(Shuffle::Increment(9));
        deck.shuffle(Shuffle::Increment(3));
        deck.shuffle(Shuffle::Cut(-1));

        assert_eq!(part2(0, 0, &deck), 0);
        assert_eq!(part2(1, 0, &deck), 9);
    }
}
