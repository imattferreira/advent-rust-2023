/**
 * Results:
 * - Part 1: 2771
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

fn game_can_ran_with_ideal_bag(
    game_attempts: &Vec<GameAttempt>,
    ideal_game_bag: &GameAttempt,
) -> bool {
    for attempt in game_attempts {
        if attempt.red > ideal_game_bag.red
            || attempt.green > ideal_game_bag.green
            || attempt.blue > ideal_game_bag.blue
        {
            return false;
        }
    }

    true
}

fn main() {
    let games_lines = get_games();
    let ideal_game_bag = GameAttempt {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Vec<_> = games_lines
        .iter()
        .map(|game| (get_game_id(game), get_game_attempts(game)))
        .collect();

    let mut possible_games_ids: Vec<u16> = vec![];

    for game in games {
        let (game_id, attempts) = game;

        if game_can_ran_with_ideal_bag(&attempts, &ideal_game_bag) {
            possible_games_ids.push(game_id);
        }
    }

    let sum: u16 = possible_games_ids.iter().sum();

    println!("sum of all possible games: {sum}");
}
