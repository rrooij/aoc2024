use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap().replace("\n", " ");
    let mut row_one: Vec<u64> = Vec::new();
    let mut row_two: Vec<u64> = Vec::new();
    let mut occurrences_two: HashMap<u64, u64> = HashMap::new();
    for (idx, content) in contents.split(" ").filter(|x| !x.is_empty()).enumerate() {
        let parsed_number = content.to_string().parse().unwrap();
        let count = match occurrences_two.get(&parsed_number) {
            None => 0,
            Some(v) => *v,
        };
        if idx % 2 == 0 {
            row_one.push(parsed_number);
        } else {
            row_two.push(parsed_number);
            occurrences_two.insert(parsed_number, count + 1);
        }
    }
    row_one.sort();
    row_two.sort();
    let mut total_distance: u64 = 0;
    let mut similarity_score: u64 = 0;
    for idx in 0..row_one.len() {
        let first_number = row_one[idx];
        let second_number = row_two[idx];
        let similarity = first_number
            * match occurrences_two.get(&first_number) {
                None => 0,
                Some(v) => *v,
            };
        similarity_score += similarity;
        if first_number > second_number {
            total_distance += first_number - second_number;
        } else {
            total_distance += second_number - first_number;
        }
    }
    println!("Distance is: {}", total_distance);
    println!("Similarity is: {}", similarity_score);
}
