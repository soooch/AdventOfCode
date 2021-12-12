use std::io::{self, Read};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let now = Instant::now();
    let sum: u32 = buffer
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.bytes().fold(0, |map, q| map | 1 << (q - b'a')))
                .fold(u32::MAX, |map, line| map & line)
                .count_ones()
        })
        .sum();

    println!("{}", now.elapsed().as_millis());

    println!("{}", sum);

    Ok(())
}
