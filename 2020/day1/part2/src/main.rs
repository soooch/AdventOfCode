use std::io::{self, Read};
use std::collections::HashSet;
fn main() -> io::Result<()> {
    const YEAR: i32 = 2020;
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let report = buffer.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    
    for entry in report.iter() {
        if let Some(product) = two_sum(YEAR - entry, report.clone()) {
            println!("{}", entry * product);
            break;
        }
    }

    Ok(())
}

fn two_sum(sum: i32, report: Vec<i32>) -> Option<i32> {
    let report = report.iter()
        .filter_map(|&entry| if entry < sum / 2 {Some(sum - entry)} else if entry > sum {None} else {Some(entry)});

    let mut visited = HashSet::new();

    for entry in report {
        if !visited.insert(entry) {
            return Some(entry * (sum - entry));
        }
    }

    None
}