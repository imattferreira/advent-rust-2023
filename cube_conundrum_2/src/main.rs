/**
 * Results:
 * - Part 1: 2771
 * - Part 2: 70924
 *
 * TODO
 * - work better with u16 and u32 (avoid casting)
 */
use std::fs;

struct GameAttempt {
    red: u16,
    green: u16,
    blue: u16,
}

fn to_usize(num: &str) -> u16 {
    match num.parse::<u16>() {
        Ok(n) => {
            return n;
        }
        Err(_) => {
            return 0;
        }
    }
}

impl GameAttempt {
    fn extract_cube_count(color: &str, cubes: &Vec<&str>) -> u16 {
        match cubes.iter().find(|i| i.contains(color)) {
            Some(red) => to_usize(red.replace(color, "").trim()),
            None => 0,
        }
    }

    pub fn new(line: &str) -> GameAttempt {
        let cubes: Vec<_> = line.split(",").map(str::trim).collect();

        let red = GameAttempt::extract_cube_count("red", &cubes);
        let green = GameAttempt::extract_cube_count("green", &cubes);
        let blue = GameAttempt::extract_cube_count("blue", &cubes);

        GameAttempt { red, green, blue }
    }
}

fn get_games() -> Vec<String> {
    let file_path = "./src/input.txt";
    let content = fs::read_to_string(file_path).expect("should have been able to read the file.");

    content.split('\n').map(str::to_string).collect()
}

fn get_game_id(line: &String) -> u16 {
    to_usize(
        line.split(":").collect::<Vec<_>>()[0]
            .replace("Game ", "")
            .trim(),
    )
}

fn get_game_attempts(line: &String) -> Vec<GameAttempt> {
    let attempts: Vec<_> = line.split(":").collect::<Vec<_>>()[1].split(";").collect();

    attempts
        .iter()
        .map(|attempt| GameAttempt::new(attempt))
        .collect::<Vec<_>>()
}

// Relative to part.1 of day 2
// fn game_can_ran_with_ideal_bag(
//     game_attempts: &Vec<GameAttempt>,
//     ideal_game_bag: &GameAttempt,
// ) -> bool {
//     for attempt in game_attempts {
//         if attempt.red > ideal_game_bag.red
//             || attempt.green > ideal_game_bag.green
//             || attempt.blue > ideal_game_bag.blue
//         {
//             return false;
//         }
//     }

//     true
// }

fn find_higher_count(counts: Vec<u16>) -> u16 {
    let mut result = counts[0];

    for count in counts {
        if result < count {
            result = count;
        }
    }

    result
}

fn find_best_attempt(attempts: &Vec<GameAttempt>) -> GameAttempt {
    let red = find_higher_count(attempts.iter().map(|i| i.red).collect());
    let green = find_higher_count(attempts.iter().map(|i| i.green).collect());
    let blue = find_higher_count(attempts.iter().map(|i| i.blue).collect());

    // println!("red {red}, green {green}, blue {blue}");

    GameAttempt { red, green, blue }
}

// Relative to part.1 of day 2
// fn _find_ideal_games() {
//     let games_lines = get_games();
//     let ideal_game_bag = GameAttempt {
//         red: 12,
//         green: 13,
//         blue: 14,
//     };

//     let games: Vec<_> = games_lines
//         .iter()
//         .map(|game| (get_game_id(game), get_game_attempts(game)))
//         .collect();

//     let mut possible_games_ids: Vec<u16> = vec![];

//     for game in games {
//         let (game_id, attempts) = game;

//         if game_can_ran_with_ideal_bag(&attempts, &ideal_game_bag) {
//             possible_games_ids.push(game_id);
//         }
//     }

//     let sum: u16 = possible_games_ids.iter().sum();

//     println!("sum of all possible games: {sum}");
// }

fn main() {
    let games_lines = get_games();

    let games: Vec<_> = games_lines
        .iter()
        .map(|game| (get_game_id(game), get_game_attempts(game)))
        .collect();

    let games_with_best_attempt: Vec<_> = games
        .iter()
        .map(|game| find_best_attempt(&game.1))
        .collect();

    let power_of_cubes_games: Vec<_> = games_with_best_attempt
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .collect();

    let sum: u32 = power_of_cubes_games
        .iter()
        .map(|i| u32::from(i.clone()))
        .sum();

    println!("sum of power of all games with best attempt is: {sum}");
}
