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
                .filter_map(|s| s.parse::<usize>().ok());
            let letter = sections.next().expect("can't isolate rule")
                .chars().next().expect("failed to get rule char");
            let password = sections.next().expect("can't isolate password");

            let indices = indices
                .map(|index| {
                    password.chars()
                        .nth(index - 1)
                        .expect("could not check password for rule char") == letter
                })
                .fold(false, |acc, x| acc ^ x);
            

            return if indices {num + 1} else {num}
        });

    println!("{}", num_valid);

    Ok(())
}
