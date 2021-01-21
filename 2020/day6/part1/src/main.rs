use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let sum = buffer.split("\n\n")
        .fold(0, |acc, group| {
            let mut yesses: u32 = 0;
            group.chars().for_each(|q| {
                match q {
                    'a'..='z' => yesses |= 1 << (q as u32 - 'a' as u32),
                    _ => (),
                }
            });
            acc + yesses.count_ones()
        });

    println!("{}", sum);

    Ok(())
}
