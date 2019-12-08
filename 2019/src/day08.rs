use std::io;
use std::io::Read;

use nom::bytes::complete::take_while_m_n;
use nom::multi::many0;
use nom::IResult;

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;
const LAYER_SIZE: usize = LAYER_HEIGHT * LAYER_WIDTH;

fn layer_p(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while_m_n(LAYER_SIZE, LAYER_SIZE, |_| true)(input)
}

fn image_p(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many0(layer_p)(input)
}

pub fn part1 (image: Vec<&[u8]>) -> usize {
    let mut lowest_zeroes = usize::max_value();
    let mut two_by_ones = 0;
    for layer in image {
        let num_zeroes = bytecount::count(layer, b'0');
        if num_zeroes >= lowest_zeroes {
            continue;
        }
        lowest_zeroes = num_zeroes;
        let num_ones = bytecount::count(layer, b'1');
        let num_twos = bytecount::count(layer, b'2');
        two_by_ones = num_twos * num_ones;
    }
    two_by_ones
}

const ANSI_RED: &str = "\x1B[41m";
const ANSI_GREEN: &str = "\x1B[42m";
const ANSI_RESET: &str = "\x1B[0m";

pub fn part2(image: Vec<&[u8]>) {
    let mut result = [2; LAYER_SIZE];
    image.iter().rev().for_each(|layer| {
        for i in 0..LAYER_SIZE {
            if layer[i] == b'2' {
                continue;
            }
            result[i] = layer[i]
        }
    });
    for line in result.chunks(LAYER_WIDTH) {
        for pixel in line {
            match pixel {
                b'1' => print!("{}  {}", ANSI_GREEN, ANSI_RESET),
                b'0' => print!("  "),
                _ => print!("{}  {}", ANSI_RED, ANSI_RESET)
            }
        }
        println!();
    }
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let image = image_p(buffer.as_bytes()).unwrap().1;

    println!("Program Output: {:?}", part2(image));
}