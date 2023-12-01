use std::fs;

fn get_calibrations() -> Vec<String> {
    let file_path = "./src/input.txt";

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file.");

    return content.split('\n').map(str::to_string).collect();
}

fn extract_numbers_from_calibration(calibration: &String) -> Vec<u16> {
    let chars: Vec<&str> = calibration.split("").collect();
    let mut result: Vec<u16> = vec![];

    for char in chars.iter() {
        match char.parse::<u16>() {
            Ok(n) => {
                result.push(n);
            },
            Err(_) => {
                // do nothing
            },
        }
    }

    return result;
}

fn main() {
    let calibrations = get_calibrations();
    let mut calibrations_numbers: Vec<u16> = vec![];

    for calibration in calibrations.iter() {
        let nums = extract_numbers_from_calibration(calibration);

        if nums.len() == 0 {
            continue;
        }

        let doubled_num = format!("{}{}", nums[0], nums[nums.len() - 1]);

        match doubled_num.parse::<u16>() {
            Ok (n) => {
                calibrations_numbers.push(n);
            },
            Err(_) => {
                // do nothing
            },
        };
    }

    let result = calibrations_numbers.iter().sum::<u16>();

    print!("sum of calibrations: {result}")
}
