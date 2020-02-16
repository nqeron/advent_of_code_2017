use std::fs;

pub fn analyze(filepath: &str) -> String {
    let input_matrix = fs::read_to_string(filepath).unwrap().trim().to_string();
    let lines = input_matrix.lines();
    let checksum = lines.fold(0,|sum,l| {
        let nums_line = l.split_ascii_whitespace().map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let max = nums_line.iter().max().unwrap();
        let min = nums_line.iter().min().unwrap();
        max - min + sum
    });

    format!("{}", checksum)
}