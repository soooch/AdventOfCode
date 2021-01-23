use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let sum = buffer.split("\n\n")
        .fold(0, |acc, group| {
            let mut yesses: u32 = 0;
            group.bytes().for_each(|q| {
                if (b'a'..=b'z').contains(&q) {
                    yesses |= 1 << (q - b'a');
                }
            });
            acc + yesses.count_ones()
        });

    println!("{}", sum);

    Ok(())
}
