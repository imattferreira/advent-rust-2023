/**
 * Part 1 - 54953
 * Part 2 - 53868
 */
use std::{fs, vec};

fn get_calibrations() -> Vec<String> {
    let file_path = "./src/input.txt";
    let content = fs::read_to_string(file_path).expect("should have been able to read the file.");

    content.split('\n').map(str::to_string).collect()
}

fn sort_by_position<'a>(vector: Vec<(&'a str, u16)>) -> Vec<&'a str> {
    let mut positions: Vec<_> = vector.iter().map(|(_, pos)| pos).collect();
    let mut result: Vec<&str> = vec![];

    positions.sort();

    for position in positions {
        match vector.iter().find(|(_, pos)| pos == position) {
            Some(i) => {
                result.push(i.0);
            }
            None => {
                // do nothing
            }
        }
    }

    result
}

fn extract_digits_from_calibration(calibration: &String) -> Vec<u16> {
    let digits = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let extensive_digits = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut extracted: Vec<(&str, u16)> = vec![];

    for digit in digits.iter() {
        let matched: Vec<_> = calibration.match_indices(digit).collect();

        matched.iter().for_each(|i| {
            extracted.push((digit, i.0 as u16));
        });
    }

    for (i, digit) in extensive_digits.iter().enumerate() {
        let matched: Vec<_> = calibration.match_indices(digit).collect();

        matched.iter().for_each(|m| {
            extracted.push((digits[i], m.0 as u16));
        });
    }

    let digits = sort_by_position(extracted);
    let mut result: Vec<u16> = vec![];

    for digit in digits {
        match digit.parse::<u16>() {
            Ok(n) => {
                result.push(n);
            }
            Err(_) => {
                // do nothing
            }
        }
    }

    result
}

fn extract_digits(calibrations: Vec<String>) -> Vec<Vec<u16>> {
    let mut result: Vec<Vec<u16>> = vec![];

    for calibration in calibrations.iter() {
        let digits = extract_digits_from_calibration(calibration);

        if digits.len() == 0 {
            continue;
        }

        result.push(digits);
    }

    result
}

fn concat_digits(digits: &Vec<u16>) -> String {
    format!("{}{}", digits[0], digits[digits.len() - 1])
}

fn unify_digits(digits: &Vec<u16>) -> u16 {
    match concat_digits(digits).parse::<u16>() {
        Ok(n) => return n,
        Err(_) => return 0,
    }
}

fn main() {
    let calibrations = get_calibrations();
    let digits = extract_digits(calibrations);
    let sum = digits.iter().map(unify_digits).sum::<u16>().clone();

    println!("sum of calibrations: {sum}")
}
