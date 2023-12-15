#![allow(unused, dead_code)]

use std::{fs, str::Chars};

#[derive(Debug)]
struct EngineNumber {
    position: Option<usize>,
    number: Option<u32>,
    length: Option<usize>,
}

pub fn run() {
    let input: Vec<String> = fs::read_to_string("src/day3/input.txt")
        .expect("Cannot read input file")
        .lines()
        .map(String::from)
        .collect();

    let result_part_1 = calculate_part_1(input);
    println!("Day 3, part 1: {}", result_part_1);
}

fn calculate_part_1(lines: Vec<String>) -> u32 {
    let num_of_lines = lines.len();
    let engine_numbers_per_line = transform_input_to_engine_numbers(&lines);
    let mut result = 0;
    let symbols = vec!['*', '#', '+', '$', '@', '&', '=', '/', '-', '%'];

    engine_numbers_per_line
        .iter()
        .enumerate()
        .for_each(|(line_num, ens_per_line)| {
            let current_line_chars: Vec<char> = lines[line_num].chars().collect();
            let mut previous_line_chars: Option<Vec<char>> = None;
            let mut next_line_chars: Option<Vec<char>> = None;

            if line_num > 0 {
                previous_line_chars = match lines.get(line_num - 1) {
                    Some(line) => Some(line.chars().collect()),
                    None => None,
                };
            }

            if line_num < num_of_lines {
                next_line_chars = match lines.get(line_num + 1) {
                    Some(line) => Some(line.chars().collect()),
                    None => None,
                };
            }

            ens_per_line.iter().for_each(|en| {
                let current_en_position = en.position.unwrap();
                let mut neighbours: Vec<char> = vec![];

                if current_en_position > 0 {
                    neighbours.push(current_line_chars[current_en_position - 1]);
                }

                if current_en_position + en.length.unwrap() < current_line_chars.len() - 1 {
                    neighbours.push(current_line_chars[current_en_position + en.length.unwrap()]);
                }

                match previous_line_chars.clone() {
                    Some(pl) => {
                        neighbours.push(pl[current_en_position]);
                        if current_en_position > 0 {
                            neighbours.push(pl[current_en_position - 1]);
                        }

                        if current_en_position + en.length.unwrap() < current_line_chars.len() {
                            neighbours.push(pl[current_en_position + en.length.unwrap()]);
                        }

                        pl[current_en_position..current_en_position + en.length.unwrap()]
                            .into_iter()
                            .for_each(|&x| neighbours.push(x))
                    }
                    None => (),
                }

                match next_line_chars.clone() {
                    Some(nl) => {
                        neighbours.push(nl[en.position.unwrap()]);
                        if current_en_position > 0 {
                            neighbours.push(nl[en.position.unwrap() - 1]);
                        }

                        if current_en_position + en.length.unwrap() < current_line_chars.len() {
                            neighbours.push(nl[en.position.unwrap() + en.length.unwrap()]);
                        }

                        nl[current_en_position..current_en_position + en.length.unwrap()]
                            .into_iter()
                            .for_each(|&x| neighbours.push(x))
                    }
                    None => (),
                }

                if neighbours
                    .iter()
                    .map(|&n| !n.is_digit(10) && n != '.')
                    .any(|n| n == true)
                {
                    result += en.number.unwrap();
                }
            });
        });

    result
}

fn transform_input_to_engine_numbers(input: &Vec<String>) -> Vec<Vec<EngineNumber>> {
    input
        .iter()
        .map(|line| {
            let mut tmp_idx = 0;
            let chars = line.chars().collect::<Vec<char>>();
            let mut num = String::from("");
            let mut inner_engine_numbers: Vec<EngineNumber> = vec![];

            for (char_idx, c) in line.chars().enumerate() {
                if char_idx < tmp_idx {
                    continue;
                }

                if c.is_digit(10) {
                    let mut num = String::new();
                    let mut engine_number = EngineNumber {
                        position: Some(char_idx),
                        number: None,
                        length: None,
                    };
                    num.push(c);
                    tmp_idx = char_idx + 1;

                    loop {
                        if tmp_idx >= chars.len() || !chars[tmp_idx].is_digit(10) {
                            break;
                        }

                        num.push(chars[tmp_idx]);
                        tmp_idx += 1;
                    }

                    engine_number.number = Some(num.parse::<u32>().unwrap());
                    engine_number.length = Some(num.len());
                    inner_engine_numbers.push(engine_number);
                }
            }

            inner_engine_numbers
        })
        .collect()
}

#[test]
fn test_calculate_part_1() {
    let input_text = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();
    assert_eq!(4361, calculate_part_1(input));
}

#[test]
fn test_again_calculate_part_1() {
    let input_text = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();
    assert_eq!(413, calculate_part_1(input));
}
