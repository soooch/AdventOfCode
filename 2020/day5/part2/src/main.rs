use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let seats = buffer.lines()
        .filter_map(|bsp| {
            let bsp = bsp.chars()
                .filter_map(|side| {
                    match side {
                        'F' => Some('0'),
                        'L' => Some('0'),
                        'B' => Some('1'),
                        'R' => Some('1'),
                        _ => None,
                    }
                }).collect::<String>();
            i32::from_str_radix(&bsp, 2).ok()
        }).collect::<Vec<i32>>();
    
    let min_seat = seats.iter().min().unwrap().clone();
    let max_seat = seats.iter().max().unwrap().clone();

    let missing = (min_seat..=max_seat).fold(0, |acc, num| acc ^ num) 
                  ^ seats.iter().fold(0, |acc, num| acc ^ num);

    println!("{}", missing);

    Ok(())
}
