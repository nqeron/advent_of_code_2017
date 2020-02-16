mod day_two_part_one;
mod day_two_part_two;

static DATA_FILEPATH: &str = "src/data/day_two.txt";
pub fn part_one() -> String {
    format!("{}",day_two_part_one::analyze(DATA_FILEPATH))
}

pub fn part_two() -> String {
    format!("part two: {}",day_two_part_two::analyze(DATA_FILEPATH))
}