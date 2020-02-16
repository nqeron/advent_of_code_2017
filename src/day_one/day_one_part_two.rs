use std::fs;

static DATA_FILEPATH: &str = "src/data/day_one.txt";

pub fn analyze() -> String{
    let input_number = fs::read_to_string(DATA_FILEPATH).unwrap().trim().to_string();
    let digits: Vec<u32> = input_number.chars().map(|c| {c.to_digit(10).unwrap()}).collect();
    let half = digits.len()/2;
    let sum = digits.iter().enumerate().fold(0, |d, idx| {
        let mut next = idx.0 + half;
        if next >= digits.len() {
            next = next - digits.len();
        }
        if digits[next].eq(idx.1) {
            idx.1 + d
        } else {
            d
        }
    });

    format!("answer: {}", sum)
}