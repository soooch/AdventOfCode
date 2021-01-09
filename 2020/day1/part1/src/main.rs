use std::io::{self, Read};
use std::collections::HashSet;
fn main() -> io::Result<()> {
    const YEAR: i32 = 2020;
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let report = buffer.lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|entry| if entry < YEAR / 2 {YEAR - entry} else {entry});
    
    let mut visited = HashSet::new();

    for entry in report {
        if !visited.insert(entry) {
            println!("{}", entry * (YEAR - entry));
            break;
        }
    }

    Ok(())
}
