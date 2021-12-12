use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    const YEAR: i32 = 2020;
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let report = buffer.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    
    for &entry in &report {
        if let Some(product) = two_num_sum_product(YEAR - entry, &report) {
            println!("{}", entry * product);
            break;
        }
    }

    Ok(())
}

fn two_num_sum_product(sum: i32, report: &Vec<i32>) -> Option<i32> {
    let mut visited = HashSet::new();
    report.iter()
        .filter_map(|&entry| if entry < sum / 2 {Some(sum - entry)} else if entry > sum {None} else {Some(entry)})
        .find(|&entry| !visited.insert(entry))
        .map(|addend| addend * (sum - addend))
}