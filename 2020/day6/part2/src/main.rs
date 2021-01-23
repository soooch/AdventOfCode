use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let sum = buffer.split("\n\n")
        .fold(0, |acc, group| {
            let all_yesses: u32 = group.lines()
                .fold(u32::MAX, |all_yes_acc, line| {
                    let mut line_yesses: u32 = 0;
                    line.bytes().for_each(|q| {
                        line_yesses |= 1 << (q - b'a');
                    });
                    all_yes_acc & line_yesses
                });
            acc + all_yesses.count_ones()
        });

    println!("{}", sum);

    Ok(())
}
