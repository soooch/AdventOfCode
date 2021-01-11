use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    const SLOPE: usize = 3;

    let num_trees = buffer.lines().enumerate()
        .fold(0, |count, (line_num, tree_row)| {
            let check_idx = (line_num * SLOPE) % tree_row.len();
            return if tree_row.chars().nth(check_idx) == Some('#') {count + 1} else {count}
        });

    println!("{}", num_trees);

    Ok(())
}
