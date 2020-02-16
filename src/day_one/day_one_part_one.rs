use std::fs;

static DATA_FILEPATH: &str = "src/data/day_one.txt";

pub fn analyze() -> String{
    let input_number = fs::read_to_string(DATA_FILEPATH).unwrap().trim().to_string();
    let digits = input_number.chars();


    let mut first_num: Option<u32> = None;
    let mut prev_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut total = 0;
    for d in digits{
        let num = d.to_digit(10).unwrap();
        if let None = first_num {
            first_num = Some(num);
            prev_digit = Some(num);
        } else{
            if prev_digit.unwrap() == num{
                total += num;
            }
            prev_digit = Some(num);
            last_digit = Some(num);
        }
    }
    if last_digit.unwrap() == first_num.unwrap() {
        total += last_digit.unwrap();
    }


    format!("answer: {}", total)
}