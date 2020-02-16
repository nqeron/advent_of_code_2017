use std::fs;

pub fn analyze(filepath: &str) -> String {
    let input_matrix = fs::read_to_string(filepath).unwrap().trim().to_string();
    let lines = input_matrix.lines();
    let checksum = lines.fold(0,|sum,l| {
        let nums_line = l.split_ascii_whitespace().map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let vars = nums_line.iter().flat_map(|n|
                                           nums_line.iter().filter(
                                               move |i| *i != n && (*i % n == 0 || n % *i == 0))).collect::<Vec<&u32>>();
        let div = *vars.iter().max().unwrap() / *vars.iter().min().unwrap();
        div + sum
    });

    format!("{}", checksum)
}