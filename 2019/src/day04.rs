use std::convert::TryFrom;

fn digits(mut number: u32) -> [u8; 6] {
    let mut result = [0; 6];
    for digit in result.iter_mut().rev() {
        *digit = u8::try_from(number % 10).unwrap();
        number /= 10;
    }
    assert!(number == 0);
    result
}

fn has_repeating_digit(number: [u8; 6]) -> bool {
    for i in 0..5 {
        if number[i] == number[i + 1] {
            return true;
        }
    }
    false
}

fn has_increasing_digits(number: [u8; 6]) -> bool {
    for i in 0..5 {
        if number[i] > number[i + 1] {
            return false;
        }
    }
    true
}

fn has_pair(number: [u8; 6]) -> bool {
    match number {
        [b, c, d, _, _, _] if b == c && c != d => true,
        [a, b, c, d, _, _] if b == c && a != b && c != d => true,
        [_, a, b, c, d, _] if b == c && a != b && c != d => true,
        [_, _, a, b, c, d] if b == c && a != b && c != d => true,
        [_, _, _, a, b, c] if b == c && a != b => true,
        _ => false,
    }
}

pub fn part1() -> usize {
    let mut total = 0;
    for x in 130_254..678_275 {
        let number = digits(x);
        if has_increasing_digits(number) && has_repeating_digit(number) {
            total += 1;
        }
    }
    total
}

pub fn part2() -> usize {
    let mut total = 0;
    for x in 130_254..678_275 {
        let number = digits(x);
        if has_increasing_digits(number) && has_pair(number) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_numbers() {
        assert_eq!(digits(123_456), [1, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn valid_numbers() {
        assert!(has_increasing_digits(digits(111_111)));
        assert!(has_repeating_digit(digits(111_111)));

        assert!(!has_increasing_digits(digits(223_450)));
        assert!(has_repeating_digit(digits(223_450)));

        assert!(has_increasing_digits(digits(123_789)));
        assert!(!has_repeating_digit(digits(123_789)));
    }
}
