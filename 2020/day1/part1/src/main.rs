use std::io::{self, Read};
use std::collections::HashSet;
fn main() -> io::Result<()> {
    const YEAR: i32 = 2020;
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut visited = HashSet::new();
    let report = buffer.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|entry| if entry < YEAR / 2 {YEAR - entry} else {entry})
        .find(|&entry| !visited.insert(entry))
        .map(|entry| entry * (YEAR - entry));
    
    if let Some(answer) = report {
       println!("{}", answer);
    }
    else {
       println!("could not find two numbers that add to {}", YEAR);
    }

    Ok(())
}
