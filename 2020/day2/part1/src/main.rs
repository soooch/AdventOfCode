use std::io::{self, Read};
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let num_valid = buffer.lines()
        .fold(0, |num, line| {
            let mut sections = line.split_ascii_whitespace();
            let mut range = sections.next()
                .expect("can't isolate range")
                .split('-')
                .filter_map(|s| s.parse::<i32>().ok());
            let range = range.next().expect("could not isolate range start") ..= range.last().expect("could not isolate range end");
            let letter = sections.next().expect("can't isolate rule").chars().next().expect("failed to get rule char");
            let password = sections.next().expect("can't isolate password");

            return if range.contains(&(password.matches(letter).count() as i32)) {
                num + 1
            } else {
                num
            }
        });

    println!("{}", num_valid);

    Ok(())
}
