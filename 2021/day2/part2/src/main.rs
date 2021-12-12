use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");

    let (_, horiz, depth) = input
        .lines()
        .map(|step| step.parse().unwrap())
        .fold((0, 0, 0), |(aim, horiz, depth), cmd| match cmd {
            Command::Thrust(dist) => (aim, horiz + dist, depth + (aim * dist)),
            Command::Pitch(tilt) => (aim + tilt, horiz, depth)
        });

    println!("{}", horiz * depth);

}

enum Command {
    Thrust(i32),
    Pitch(i32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        if let Some((dir, dist)) = input.split_once(' ') {
            if let Ok(dist) = dist.parse() {
                match dir {
                    "forward" => Ok(Command::Thrust(dist)),
                    "down" => Ok(Command::Pitch(dist)),
                    "up" => Ok(Command::Pitch(-dist)),
                    d => Err(format!("Could not parse direction {} in input {}", d, input))
                }
            } else {
                Err(format!("Could not parse distance {} in input {}", dist, input))
            }
        } else {
            Err(format!("Could not divide direction and distance in input {}", input))
        }
    }
}

