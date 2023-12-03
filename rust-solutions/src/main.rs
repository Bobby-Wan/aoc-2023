#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

fn parse_digit(s: &str) -> Option<u32> {
    if s.starts_with("one") { return Some(1); }
    if s.starts_with("two") { return Some(2); }
    if s.starts_with("three") { return Some(3); }
    if s.starts_with("four") { return Some(4); }
    if s.starts_with("five") { return Some(5); }
    if s.starts_with("six") { return Some(6); }
    if s.starts_with("seven") { return Some(7); }
    if s.starts_with("eight") { return Some(8); }
    if s.starts_with("nine") { return Some(9); }

    if s.starts_with("1") { return Some(1); }
    if s.starts_with("2") { return Some(2); }
    if s.starts_with("3") { return Some(3); }
    if s.starts_with("4") { return Some(4); }
    if s.starts_with("5") { return Some(5); }
    if s.starts_with("6") { return Some(6); }
    if s.starts_with("7") { return Some(7); }
    if s.starts_with("8") { return Some(8); }
    if s.starts_with("9") { return Some(9); }

    return None;
}

fn part1() {
    let file = File::open("input1").unwrap();
    let lines = BufReader::new(file).lines();

    let mut sum: u32 = 0;
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut found_first: bool;
    let mut found_second: bool;

    for line in lines {
        found_first = false;
        found_second = false;

        let line = line.expect("Could not parse line.");
        for char in line.chars() {
            if char.is_digit(10) {
                if !found_first {
                    first = char.to_digit(10).unwrap();
                    found_first = true;
                } else {
                    second = char.to_digit(10).unwrap();
                    found_second = true;
                }
            }
        }

        if found_second {
            sum += 10*first + second;
        } else {
            sum += 10*first + first;
        }
    }

    println!("{}", sum);
}

fn part2() {
    let file = File::open("input1").unwrap();
    let lines = BufReader::new(file).lines();

    let mut sum: u32 = 0;
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut found_first: bool;
    let mut found_second: bool;

    for line in lines {
        found_first = false;
        found_second = false;

        let line = line.expect("Could not parse line.");
        for (i, _) in line.chars().enumerate() {
            let digit = parse_digit(&line[i..]);
            match digit {
                Some(digit) => {
                    if !found_first {
                        first = digit;
                        found_first = true;
                    } else {
                        second = digit;
                        found_second = true;
                    }
                }
                None => ()
            }
        }

        let number: u32;
        if found_second {
            number = 10*first + second;
        } else {
            number = 10*first + first;
        }

        sum += number;
    }

    println!("{}", sum);
}

fn main() {
    part2();
}
