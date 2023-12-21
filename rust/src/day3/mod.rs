use std::fs;

#[derive(Debug)]
struct EnginePart<T> {
    row: usize,
    position: Option<usize>,
    value: Option<T>,
    length: Option<usize>,
}

impl<T> EnginePart<T> {
    fn new_from_chars(y: usize, x: usize, row: &Vec<char>) -> EnginePart<u32> {
        let mut engine_part = EnginePart::<u32> {
            row: y,
            position: Some(x),
            value: None,
            length: None,
        };

        let mut next_idx = x + 1;
        let mut num = String::new();
        num.push(row[x]);

        loop {
            if next_idx >= row.len() || !row[next_idx].is_digit(10) {
                break;
            }

            num.push(row[next_idx]);
            next_idx += 1;
        }

        engine_part.value = Some(
            num.parse::<u32>()
                .expect(format!("Cannot parse u32 with {}", num).as_str()),
        );
        engine_part.length = Some(num.len());
        engine_part
    }

    fn char_neighbours(
        &self,
        current_row_chars: &Vec<char>,
        next_row_chars: &Option<Vec<char>>,
        previous_row_chars: &Option<Vec<char>>,
    ) -> Vec<char> {
        let current_position = self.position.unwrap();
        let current_length = self.length.unwrap();
        let mut neighbours: Vec<char> = vec![];

        neighbours.push(current_row_chars[current_position - 1]);
        neighbours.push(current_row_chars[current_position + current_length]);

        match previous_row_chars.clone() {
            Some(p) => {
                neighbours.push(p[current_position]);
                neighbours.push(p[current_position - 1]);
                neighbours.push(p[current_position + current_length]);
                p[current_position..current_position + current_length]
                    .into_iter()
                    .for_each(|&x| neighbours.push(x));
            }
            None => (),
        }

        match next_row_chars.clone() {
            Some(n) => {
                neighbours.push(n[current_position]);
                neighbours.push(n[current_position - 1]);
                neighbours.push(n[current_position + current_length]);
                n[current_position..current_position + current_length]
                    .into_iter()
                    .for_each(|&x| neighbours.push(x));
            }
            None => (),
        }

        neighbours
    }

    fn number_neighbours<'a>(
        &'a self,
        numbers_rows: &'a Vec<Vec<EnginePart<u32>>>,
    ) -> Vec<&EnginePart<u32>> {
        let mut rows_to_search: Vec<usize> = vec![self.row];
        let mut found: Vec<&EnginePart<u32>> = vec![];

        rows_to_search.push(self.row - 1);
        rows_to_search.push(self.row + 1);

        rows_to_search.iter().for_each(|row_checking| {
            numbers_rows[*row_checking].iter().for_each(|num| {
                if num.row == row_checking.to_owned() {
                    if ((num.position.unwrap() - 1)
                        ..(num.position.unwrap() + num.length.unwrap() + 1))
                        .collect::<Vec<usize>>()
                        .contains(&self.position.unwrap())
                    {
                        found.push(&num);
                    }
                }
            });
        });

        found
    }
}

pub fn run() {
    let input: Vec<String> = fs::read_to_string("src/day3/input.txt")
        .expect("Cannot read input file")
        .lines()
        .map(String::from)
        .map(|line| format!(".{}.", line)) // Add . at the start and end ;)
        .collect();

    let result_part_1 = calculate_part_1(&input);
    println!("Day 3, part 1: {}", result_part_1);

    let result_part_2 = calculate_part_2(&input);
    println!("Day 3, part 2: {}", result_part_2);
}

fn calculate_part_1(rows: &Vec<String>) -> u32 {
    let num_of_rows = rows.len();
    let engine_part_numbers = transform_input_to_engine_part_numbers(&rows);
    let mut result = 0;

    engine_part_numbers
        .iter()
        .enumerate()
        .for_each(|(row_num, row_nums)| {
            let current_row_chars: Vec<char> = rows[row_num].chars().collect();
            let not_first_row = row_num > 0;
            let not_last_row = row_num < num_of_rows;
            let mut previous_row_chars: Option<Vec<char>> = None;
            let mut next_row_chars: Option<Vec<char>> = None;

            if not_first_row {
                previous_row_chars = match rows.get(row_num - 1) {
                    Some(row) => Some(row.chars().collect()),
                    None => None,
                };
            }

            if not_last_row {
                next_row_chars = match rows.get(row_num + 1) {
                    Some(row) => Some(row.chars().collect()),
                    None => None,
                };
            }

            row_nums.iter().for_each(|num| {
                let neighbours =
                    num.char_neighbours(&current_row_chars, &next_row_chars, &previous_row_chars);

                if neighbours
                    .iter()
                    .map(|&n| !n.is_digit(10) && n.clone() != '.')
                    .any(|n| n == true)
                {
                    result += num.value.unwrap();
                }
            });
        });

    result
}

fn calculate_part_2(rows: &Vec<String>) -> u32 {
    let rows_numbers = transform_input_to_engine_part_numbers(&rows);
    let rows_gears = transform_input_to_engine_part_gears(&rows);
    let mut result = 0;

    rows_gears
        .iter()
        .enumerate()
        .for_each(|(_row_num, gears_per_row)| {
            gears_per_row.iter().for_each(|gear| {
                let neighbours = gear.number_neighbours(&rows_numbers);
                if neighbours.len() == 2 {
                    result += neighbours[0].value.unwrap() * neighbours[1].value.unwrap();
                }
            });
        });

    result
}

fn transform_input_to_engine_part_numbers(input: &Vec<String>) -> Vec<Vec<EnginePart<u32>>> {
    input
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            let mut cursor_at = 0;
            let chars = row.chars().collect::<Vec<char>>();
            let mut engine_parts: Vec<EnginePart<u32>> = vec![];

            for (char_idx, c) in row.chars().enumerate() {
                if cursor_at > char_idx {
                    continue;
                }

                if c.is_digit(10) {
                    cursor_at = char_idx;
                    let engine_part = EnginePart::<u32>::new_from_chars(row_num, cursor_at, &chars);
                    cursor_at += engine_part.length.unwrap();
                    engine_parts.push(engine_part);
                }
            }

            engine_parts
        })
        .collect()
}

fn transform_input_to_engine_part_gears(input: &Vec<String>) -> Vec<Vec<EnginePart<char>>> {
    input
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            let mut engine_parts: Vec<EnginePart<char>> = vec![];

            for (char_idx, c) in row.chars().enumerate() {
                if c == '*' {
                    let engine_part = EnginePart::<char> {
                        row: row_num,
                        position: Some(char_idx),
                        value: Some('*'),
                        length: Some(1),
                    };
                    engine_parts.push(engine_part);
                }
            }

            engine_parts
        })
        .collect()
}

#[test]
fn test_calculate_part_1() {
    let input_text = ".467..114..
....*......
...35..633.
.......#...
.617*......
......+.58.
...592.....
.......755.
....$.*....
..664.598..";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();
    assert_eq!(4361, calculate_part_1(&input));
}

#[test]
fn test_again_calculate_part_1() {
    let input_text = ".12.......*...
.+.........34.
........-12...
...78.........
...*....60....
.78...........
........23....
.....90*12....
..............
.2.2......12..
..*.........*.
.1.1.......56.";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();
    assert_eq!(413, calculate_part_1(&input));
}

#[test]
fn test_calculate_part_2() {
    let input_text = ".467..114..
....*......
...35..633.
.......#...
.617*......
......+.58.
...592.....
.......755.
....$.*....
..664.598..";
    let input: Vec<String> = input_text.split("\n").map(String::from).collect();
    assert_eq!(467835, calculate_part_2(&input));
}
