use std::io::{self, Read};
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let num_valid = buffer.lines()
        .fold(0, |num, line| {
            let mut sections = line.split_ascii_whitespace();
            let indices = sections.next()
                .expect("can't isolate range")
                .split('-')
                .filter_map(|s| s.parse::<usize>().ok())
                .map(|index| index - 1)
                .collect::<Vec<usize>>();
            let letter = sections.next().expect("can't isolate rule").chars().next().expect("failed to get rule char");
            let password = sections.next().expect("can't isolate password");

            return if 
            (password.chars().nth(indices[0]).expect("could not check password for first rule character") == letter) 
            ^ 
            (password.chars().nth(indices[1]).expect("could not check password for second rule character") == letter) 
            {
                num + 1
            } 
            else 
            {
                num
            }
        });

    println!("{}", num_valid);

    Ok(())
}
