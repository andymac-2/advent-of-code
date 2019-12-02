use std::io;
use std::io::Read;

use nom::character::complete::{digit1, newline, space0};
use nom::multi::separated_list;
use nom::IResult;

fn line_p(input: &str) -> IResult<&str, u64> {
    let (input, _) = space0(input)?;
    let (input, num_str) = digit1(input)?;
    let (input, _) = space0(input)?;
    Ok((input, num_str.parse().unwrap()))
}

fn lines_p(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list(newline, line_p)(input)
}

pub fn part1(masses: Vec<u64>) -> u64 {
    masses.into_iter().fold(0, |acc, mass| acc + (mass / 3) - 2)
}

pub fn part2(masses: Vec<u64>) -> u64 {
    masses.into_iter().fold(0, |mut total_so_far, mut mass| {
        loop {
            mass = (mass / 3).saturating_sub(2);

            if mass == 0 {
                break;
            }

            total_so_far += mass;
        }
        total_so_far
    })
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let masses = lines_p(&buffer).unwrap().1;
    let total_fuel = part2(masses);

    println!("Total fuel: {}", total_fuel);
}
