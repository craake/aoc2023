use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new(id: u32, sets: Vec<Set>) -> Game {
        Game { id, sets }
    }

    fn is_possible(&self) -> bool {
        self.sets
            .iter()
            .map(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
            .all(|x| x)
    }
}

#[derive(Debug)]
struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

impl Set {
    fn new(blue: u32, red: u32, green: u32) -> Set {
        Set { blue, red, green }
    }
}

pub fn run() {
    let games = transform_input(
        fs::read_to_string("src/day2/input.txt")
            .expect("Cannot read input file")
            .lines()
            .map(String::from)
            .collect(),
    );

    let result = calculate_part_1(&games);
    println!("Day 2, part 1: {}", result);
}

fn calculate_part_1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| if game.is_possible() { game.id } else { 0 })
        .sum()
}

fn transform_input(lines: Vec<String>) -> Vec<Game> {
    lines
        .iter()
        .map(|line| {
            let line_parts: Vec<String> = line.split(":").map(String::from).collect();
            let game_id_parts: Vec<String> = line_parts[0].split(" ").map(String::from).collect();
            let game_id = game_id_parts[1].clone().parse::<u32>().unwrap();
            let subsets_strs: Vec<String> = line_parts[1].split(";").map(String::from).collect();
            let subsets: Vec<Vec<String>> = subsets_strs
                .iter()
                .map(|x| {
                    x.split(",")
                        .map(String::from)
                        .map(|x| x.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>();

            let sets = subsets
                .iter()
                .map(|subset_string| {
                    let mut set = Set::new(0, 0, 0);
                    subset_string.iter().for_each(|set_member| {
                        let subset: Vec<String> =
                            set_member.split(" ").map(|x| String::from(x)).collect();

                        match subset[1].clone().as_str() {
                            "red" => set.red = subset[0].parse::<u32>().unwrap(),
                            "blue" => set.blue = subset[0].parse::<u32>().unwrap(),
                            "green" => set.green = subset[0].parse::<u32>().unwrap(),
                            _ => panic!("Invalid color"),
                        };
                    });
                    set
                })
                .collect();
            let game = Game::new(game_id, sets);

            game
        })
        .collect()
}

#[test]
fn test_transform_input() {
    let input_text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

    let input: Vec<String> = input_text.split("\n").map(String::from).collect();
    let res = transform_input(input);
    let g1 = &res[0];
    let g2 = &res[1];

    assert_eq!(g1.sets[0].red, 4);
    assert_eq!(g1.sets[0].blue, 3);
    assert_eq!(g1.sets[0].green, 0);
    assert_eq!(g1.sets[1].red, 1);
    assert_eq!(g1.sets[1].blue, 6);
    assert_eq!(g1.sets[1].green, 2);
    assert_eq!(g1.sets[2].red, 0);
    assert_eq!(g1.sets[2].blue, 0);
    assert_eq!(g1.sets[2].green, 2);

    assert_eq!(g2.sets[0].red, 0);
    assert_eq!(g2.sets[0].blue, 1);
    assert_eq!(g2.sets[0].green, 2);
    assert_eq!(g2.sets[1].red, 1);
    assert_eq!(g2.sets[1].blue, 4);
    assert_eq!(g2.sets[1].green, 3);
    assert_eq!(g2.sets[2].red, 0);
    assert_eq!(g2.sets[2].blue, 1);
    assert_eq!(g2.sets[2].green, 1);
}

#[test]
fn test_calculate_part_1() {
    let input_text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let input: Vec<Game> = transform_input(input_text.split("\n").map(String::from).collect());
    assert_eq!(calculate_part_1(&input), 8);
}
