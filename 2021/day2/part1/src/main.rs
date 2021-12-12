fn main() {
    let input = include_str!("../../input.txt");

    let (x, y) = input
        .lines()
        .filter_map(|step| step_from_str(step).ok())
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();

    println!("{}", x * y);

}

fn step_from_str(input: &str) -> Result<(i32, i32), String> {
    if let Some((dir, dist)) = input.split_once(' ') {
        if let Ok(dist) = dist.parse() {
            match dir {
                "forward" => Ok((dist, 0)),
                "down" => Ok((0, dist)),
                "up" => Ok((0, -dist)),
                d => Err(format!("Could not parse direction: {}", d))
            }
        } else {
            Err(format!("Could not parse distance: {}", dist))
        }
    } else {
        Err(format!("Could not divide direction and distance"))
    }
}

