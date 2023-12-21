use std::fs;

pub fn run() {
    let input: Vec<String> = fs::read_to_string("src/day4/input.txt")
        .expect("Cannot read input file")
        .lines()
        .map(String::from)
        .collect();

    let result_part_1 = calculate_part_1(&input);
    println!("Day 4, part 1: {}", result_part_1);

    let result_part_2 = calculate_part_2(&input);
    println!("Day 4, part 2: {}", result_part_2);
}

fn parse_card(card: String) -> (Vec<u32>, Vec<u32>) {
    let parts: Vec<String> = card.split('|').map(String::from).collect();
    let winning_numbers: Vec<u32> = parts
        .get(0)
        .expect("Winning numbers are missing")
        .split(':')
        .skip(1)
        .flat_map(|x| {
            x.split(' ')
                .filter(|s| !s.is_empty())
                .map(|x| x.parse::<u32>().expect("Cannot parse u32"))
                .collect::<Vec<u32>>()
        })
        .collect();
    let my_numbers = parts
        .get(1)
        .expect("My numbers are missing")
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().expect("Cannot parse u32"))
        .collect();

    (winning_numbers, my_numbers)
}

fn calculate_part_1(cards: &Vec<String>) -> u32 {
    cards
        .into_iter()
        .map(|card| {
            let (winning_numbers, my_numbers) = parse_card(card.to_string());
            let mut hits = 0;
            my_numbers.iter().for_each(|x| {
                if winning_numbers.contains(&x) {
                    hits = match hits {
                        0 => 1,
                        _ => hits * 2,
                    }
                }
            });
            hits
        })
        .sum()
}

fn calculate_part_2(cards: &Vec<String>) -> u32 {
    let mut prizes = vec![1; cards.len()];

    cards.into_iter().enumerate().for_each(|(n, card)| {
        let (winning_numbers, my_numbers) = parse_card(card.to_string());
        let mut hits = 0;

        my_numbers.iter().for_each(|x| {
            if winning_numbers.contains(&x) {
                hits += 1
            }
        });

        if hits > 0 {
            for x in n + 1..=n + hits {
                for _ in 0..prizes[n] {
                    prizes[x] += 1;
                }
            }
        }
    });

    prizes.iter().sum()
}

#[test]
fn test_calculate_part_1() {
    let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let input = input_text
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    assert_eq!(13, calculate_part_1(&input));
}

#[test]
fn test_calculate_part_2() {
    let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let input = input_text
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    assert_eq!(30, calculate_part_2(&input));
}
