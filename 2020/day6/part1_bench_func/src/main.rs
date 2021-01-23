use std::io::{self, Read};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let now= Instant::now();

    let sum = buffer.lines()
        .fold(0, |acc, group| {
            let mut yesses: u32 = 0;
            group.bytes().for_each(|q| {
                if (b'a'..=b'z').contains(&q) {
                    yesses |= 1 << (q - b'a');
                }
            });
            acc + yesses.count_ones()
        });
    
    println!("{}", now.elapsed().as_millis());

    println!("{}", sum);

    Ok(())
}
