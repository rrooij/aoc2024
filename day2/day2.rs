use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let contents = input.lines();
    let mut safe_reports = 0;
    for line in contents {
        let mut safe_score = 1;
        let mut increasing = true; // We assume increase as the initial state
        let mut previous_number = 0;
        for (idx, number) in line.split(" ").enumerate() {
            let number: i64 = number.parse().unwrap();
            // There are no 0s in the input, so 0 is always the initial state
            if previous_number == 0 {
                previous_number = number;
                continue;
            }
            let difference = (number - previous_number).abs();
            if difference == 0 || difference > 3 {
                safe_score = 0;
                break;
            }
            if number > previous_number && idx == 1 {
                increasing = true;
            } else if number < previous_number && idx == 1 {
                increasing = false;
            } else if (number > previous_number && !increasing)
                || (number < previous_number && increasing)
            {
                safe_score = 0;
                break;
            }
            previous_number = number;
        }
        safe_reports += safe_score;
    }
    println!("Safe reports {}", safe_reports);
}
