use std::io;
use std::io::Read;
use std::convert::TryFrom;

use crate::parsers::*;

fn numbers_p (s: &[u8]) -> ParseResult<Vec<i32>> {
    take_while(token(|c| {
        char::from(c)
            .to_digit(10)
            .map(|n| i32::try_from(n).unwrap())
    }))(s)
}

/// period of 1: 0, 1, 0, -1 ...
/// period of 2: 0, 0, 1, 1, 0, 0, ...
fn flawed_sin(period: usize, position: usize) -> i32 {
    [0, 1, 0, -1][(position / period) % 4]
}

/// period of 1: 0, 1, 0, -1 ...
/// period of 2: 0, 0, 1, 1, 0, 0, ...
fn flawed_cos(period: usize, position: usize) -> i32 {
    [1, 0, -1, 0][(position / period) % 4]
}

fn flawed_ft (input: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for output_ix in 0..input.len() {
        let mut sum = 0;

        for (input_ix, input) in input.iter().enumerate() {
            sum += flawed_sin(output_ix + 1, input_ix + 1) * input
        }

        result.push((sum % 10).abs())
    }

    result
}

fn sum_digit_10k (position: usize, period: usize, length: usize, digit: i32) -> i32 {
    let original_index = (position / period) % 4;
    let offset_index = (length / period) % 4;

    unreachable!()
}

pub fn part1(mut signal: Vec<i32>) -> Vec<i32> {
    for _ in 0..100 {
        signal = flawed_ft(&signal);
    }
    signal
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let numbers = numbers_p(buffer.as_bytes()).unwrap().1;

    println!("numbers len: {:?}", numbers.len());
    println!("Program Output: {:?}", part1(numbers));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flawed_ft1 () {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let output = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(flawed_ft(&input), output);
    }
}