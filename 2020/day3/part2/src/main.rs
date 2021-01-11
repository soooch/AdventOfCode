use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    // TODO: (not that I ever will) 
    // could maybe speed up for large cases by mapping string into vec of bools first 
    // (then iterating over chunks instead of lines)

    let trees_product = slopes.iter().fold(1, |product, (right, down)| {
        product * buffer.lines().step_by(*down).enumerate()
            .fold(0, |count, (line_num, tree_row)| {
                let check_idx = (line_num * right) % tree_row.len();
                if tree_row.chars().nth(check_idx) == Some('#') {count + 1} else {count}
            })
    });

    println!("{}", trees_product);

    Ok(())
}
