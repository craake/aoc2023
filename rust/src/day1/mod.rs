use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/day1/input.txt")
        .expect("Cannot read input file")
        .lines()
        .map(String::from)
        .collect();

    let result_1 = calculate_part_1(&input);
    println!("Day 1, part 1: {}", result_1);

    let result_2 = calculate_part_2(&input);
    println!("Day 1, part 2: {}", result_2);
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

fn enumerate_digits(str: &str) -> Vec<u32> {
    let mut result: Vec<u32> = vec![0; str.len()];
    str.chars().enumerate().for_each(|(idx, c)| {
        if c.is_digit(10) {
            result[idx] = c.to_digit(10).unwrap();
        }
    });

    result
}

fn enumerate_word_digits(str: &str) -> Vec<u32> {
    let word_digit_map = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result: Vec<u32> = vec![0; str.len()];

    word_digit_map.iter().enumerate().for_each(|(idx, word)| {
        let mut i = 0;
        while i <= str.len() {
            if i + word.len() > str.len() {
                i += 1;
                continue;
            }

            if str[i..(i + word.len())].contains(word) {
                result[i] = (idx as u32) + 1;
                i += word.len();
            }

            i += 1;
        }
    });

    result
}

fn calculate_part_1(input: &Vec<String>) -> u32 {
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

fn calculate_part_2(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut r = enumerate_digits(&line.as_str());

            enumerate_word_digits(line)
                .iter()
                .enumerate()
                .for_each(|(idx, n)| {
                    let n = n.clone();
                    if n > 0 {
                        r[idx] = n.clone();
                    }
                });

            r
        })
        .map(|line| {
            line.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .concat()
                .replace("0", "")
        })
        .map(|line| {
            let line: Vec<char> = line.chars().collect();
            (if line.len() == 1 {
                format!("{}{}", line[0], line[0])
            } else {
                format!("{}{}", line.first().unwrap(), line.last().unwrap())
            })
            .parse::<u32>()
            .unwrap()
        })
        .sum::<u32>()
}

#[test]
fn test_filter_digits() {
    assert_eq!(vec![6, 9], filter_digits("ab6cdef9ijk"));
    assert_eq!(vec![6, 6, 9, 9], filter_digits("6ab6cdef9ijk9"));
}

#[test]
fn test_enumerate_digits() {
    assert_eq!(
        vec![0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0],
        enumerate_digits("ab6cdef9ijk")
    );
    assert_eq!(
        vec![6, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 9],
        enumerate_digits("6ab6cdef9ijk9")
    );
}

#[test]
fn test_enumerate_word_digits() {
    assert_eq!(
        vec![2, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
        enumerate_word_digits("two6cdeninef9ijk")
    );

    assert_eq!(
        vec![0, 5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0],
        enumerate_word_digits("6fivesixf9ieightk")
    );
}

#[test]
fn test_calculate_part_1() {
    let input_text = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();

    assert_eq!(142, calculate_part_1(&input));
}

#[test]
fn test_calculate_part_2() {
    let input_text = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();

    assert_eq!(281, calculate_part_2(&input));
}
