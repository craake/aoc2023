#![allow(dead_code, unused)]

use std::{env::current_dir, fs, path};

pub fn run() {
    let input = fs::read_to_string("src/day1/input.txt")
        .expect("Cannot read input file")
        .lines()
        .map(String::from)
        .collect();
    let result = calculate(input);

    println!("Day 1, part 1: {}", result);
}

fn filter_digits(str: &str) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    str.chars().for_each(|c| {
        if c.is_digit(10) {
            result.push(c.to_digit(10).unwrap());
        }
    });

    result
}

fn calculate(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| filter_digits(&line.as_str()))
        .map(|line| {
            (if line.len() == 1 {
                format!("{}{}", line[0], line[0])
            } else {
                format!("{}{}", line.first().unwrap(), line.last().unwrap())
            })
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}

#[test]
fn test_pick_numbers() {
    assert_eq!(vec![6, 9], filter_digits("ab6cdef9ijk"));
    assert_eq!(vec![6, 6, 9, 9], filter_digits("6ab6cdef9ijk9"));
}

#[test]
fn test_calculate() {
    let input_text = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();

    assert_eq!(142, calculate(input));
}
