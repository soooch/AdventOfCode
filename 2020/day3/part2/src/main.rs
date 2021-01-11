use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let normal_part = (1..=7).step_by(2).fold(1, |product, slope| {
        product * buffer.lines().enumerate()
        .fold(0, |count, (line_num, tree_row)| {
            let check_idx = (line_num * slope) % tree_row.len();
            return if tree_row.chars().nth(check_idx) == Some('#') {count + 1} else {count}
        })
    });

    let annoying_part  = buffer.lines().step_by(2).enumerate()
        .fold(0, |count, (line_num, tree_row)| {
            return if tree_row.chars().nth(line_num % tree_row.len()) == Some('#') {count + 1} else {count}
        });

    println!("{}", normal_part * annoying_part);

    Ok(())
}
