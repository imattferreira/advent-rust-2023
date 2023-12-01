use std::fs;

fn get_calibrations() -> Vec<String> {
    let file_path = "./src/input.txt";
    let content = fs::read_to_string(file_path).expect("should have been able to read the file.");

    content.split('\n').map(str::to_string).collect()
}

fn extract_numbers_from_calibration(calibration: &String) -> Vec<u16> {
    let chars: Vec<&str> = calibration.split("").collect();
    let mut result: Vec<u16> = vec![];

    for char in chars.iter() {
        match char.parse::<u16>() {
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

fn extract_numbers_from_calibrations(calibrations: Vec<String>) -> Vec<Vec<u16>> {
    let mut result: Vec<Vec<u16>> = vec![];

    for calibration in calibrations.iter() {
        let nums = extract_numbers_from_calibration(calibration);

        if nums.len() == 0 {
            continue;
        }

        result.push(nums);
    }

    result
}

fn concat_vec_nums(vector: &Vec<u16>) -> String {
    format!("{}{}", vector[0], vector[vector.len() - 1])
}

fn main() {
    let calibrations = get_calibrations();
    let calibrations_numbers = extract_numbers_from_calibrations(calibrations);

    let converted_calibrations: Vec<u16> = calibrations_numbers
        .iter()
        .map(concat_vec_nums)
        .map(|s| {
            match s.parse::<u16>() {
                Ok(n) => return n,
                Err(_) => return 0,
            };
        })
        .collect();

    let result = converted_calibrations.iter().sum::<u16>();

    print!("sum of calibrations: {result}")
}
