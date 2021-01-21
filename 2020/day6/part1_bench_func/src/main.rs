use std::io::{self, Read};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    
    let now = Instant::now();

    let sum = buffer.split("\n\n")
        .fold(0, |acc, group| {
            let mut yesses: u32 = 0;
            group.chars().for_each(|q| {
                match q {
                    'a'..='z' => yesses |= 1 << (q as u8 - 'a' as u8),
                    _ => (),
                }
            });
            acc + yesses.count_ones()
        });

    println!("{}", now.elapsed().as_millis());

    println!("{}", sum);

    Ok(())
}
