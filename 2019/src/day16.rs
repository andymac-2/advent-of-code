use std::convert::TryFrom;

use crate::parsers::*;

fn numbers_p(s: &[u8]) -> ParseResult<Vec<i32>> {
    many(token(|c| {
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

fn flawed_ft(input: &[i32]) -> Vec<i32> {
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

fn fast_flawed_ft(input: &[i32], offset: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(input.len());
    let sum_range = SumRange::new(input, offset);

    for ix in 0..input.len() {
        result.push(sum_range.get_digit(ix + offset));
    }

    result
}

struct SumRange {
    sums: Vec<i32>,
    offset: usize,
}
impl SumRange {
    fn new(numbers: &[i32], offset: usize) -> Self {
        let mut sums = Vec::with_capacity(numbers.len() + 1);
        sums.push(0);
        let mut current_sum = 0;

        for number in numbers {
            current_sum += number;
            sums.push(current_sum);
        }

        SumRange { sums, offset }
    }

    fn get_range(&self, lower: usize, upper: usize) -> i32 {
        assert!(lower < self.sums.len() + self.offset);
        assert!(lower <= upper);

        let upper_bound = (upper - self.offset).min(self.sums.len() - 1);
        self.sums[upper_bound] - self.sums[lower - self.offset]
    }

    fn get_digit(&self, position: usize) -> i32 {
        let quarter_wavelength = position + 1;
        let half_wavelength = quarter_wavelength * 2;
        let last = self.sums.len() - 2;

        let mut lower = quarter_wavelength - 1;
        let mut current_sum = 0;
        loop {
            current_sum += self.get_range(lower, lower + quarter_wavelength);

            lower += half_wavelength;
            if lower > last {
                return (current_sum % 10).abs();
            }

            current_sum -= self.get_range(lower, lower + quarter_wavelength);

            lower += half_wavelength;
            if lower > last {
                return (current_sum % 10).abs();
            }
        }
    }
}

pub fn part1(mut signal: Vec<i32>) -> Vec<i32> {
    for _ in 0..100 {
        signal = flawed_ft(&signal);
    }
    signal
}

pub fn part2(signal: Vec<i32>, target: usize) -> Vec<i32> {
    let length = signal.len() * 10_000;

    let mut real_signal: Vec<_> = signal
        .into_iter()
        .cycle()
        .take(length)
        .skip(target)
        .collect();

    for _ in 0..100 {
        real_signal = fast_flawed_ft(&real_signal, target);
    }

    real_signal.iter().take(8).copied().collect()
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day16.txt")
        .expect("Something went wrong reading the file");

    let numbers = numbers_p(buffer.as_bytes()).unwrap().1;

    println!("numbers len: {:?}", numbers.len());
    println!("Program Output: {:?}", &part2(numbers, 5_973_847));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flawed_ft1() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let output = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(flawed_ft(&input), output);
    }

    #[test]
    fn fast_flawed_ft1() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let output = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(fast_flawed_ft(&input, 0), output);
    }

    #[test]
    fn fast_flawed_ft2() {
        let input = [3, 4, 5, 6, 7, 8];
        let output = vec![2, 2, 6, 1, 5, 8];
        assert_eq!(fast_flawed_ft(&input, 2), output);
    }

    #[test]
    fn sum_ranges() {
        let sum_range = SumRange::new(&[1, 2, 3, 4, 5, 6, 7, 8], 0);
        assert_eq!(sum_range.get_range(0, 0), 0);
        assert_eq!(sum_range.get_range(0, 1), 1);
        assert_eq!(sum_range.get_range(0, 100), 36);
        assert_eq!(sum_range.get_range(0, 7), 28);
        assert_eq!(sum_range.get_range(1, 7), 27);
    }

    #[test]
    fn sum_ranges_2() {
        let sum_range = SumRange::new(&[2, 3, 4, 5, 6, 7, 8], 1);
        assert_eq!(sum_range.get_range(1, 1), 0);
        assert_eq!(sum_range.get_range(1, 2), 2);
        assert_eq!(sum_range.get_range(1, 100), 35);
        assert_eq!(sum_range.get_range(1, 7), 27);
        assert_eq!(sum_range.get_range(2, 7), 25);
    }

    #[test]
    fn digits() {
        let sum_range = SumRange::new(&[1, 2, 3, 4, 5, 6, 7, 8], 0);
        assert_eq!(sum_range.get_digit(0), 4);
        assert_eq!(sum_range.get_digit(1), 8);
        assert_eq!(sum_range.get_digit(2), 2);
        assert_eq!(sum_range.get_digit(3), 2);
        assert_eq!(sum_range.get_digit(4), 6);
        assert_eq!(sum_range.get_digit(5), 1);
        assert_eq!(sum_range.get_digit(6), 5);
        assert_eq!(sum_range.get_digit(7), 8);
    }

    #[test]
    fn digits2() {
        let sum_range = SumRange::new(&[2, 3, 4, 5, 6, 7, 8], 1);
        assert_eq!(sum_range.get_digit(1), 8);
        assert_eq!(sum_range.get_digit(2), 2);
        assert_eq!(sum_range.get_digit(3), 2);
        assert_eq!(sum_range.get_digit(4), 6);
        assert_eq!(sum_range.get_digit(5), 1);
        assert_eq!(sum_range.get_digit(6), 5);
        assert_eq!(sum_range.get_digit(7), 8);
    }

    #[test]
    fn part_2_1() {
        let input = "03036732577212944063491565474664";
        let numbers = numbers_p(input.as_bytes()).unwrap().1;
        assert_eq!(part2(numbers, 303_673), vec![8, 4, 4, 6, 2, 0, 2, 6]);
    }
}
