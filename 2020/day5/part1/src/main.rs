use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let highest_id = buffer.lines()
        .filter_map(|bsp| {
            let seat = bsp.chars()
                .filter_map(|side| {
                    match side {
                        'F' => Some('0'),
                        'L' => Some('0'),
                        'B' => Some('1'),
                        'R' => Some('1'),
                        _ => None,
                    }
                }).collect::<String>();
            i32::from_str_radix(&seat, 2).ok()
        }).max();
    
    println!("{}", highest_id.unwrap());

    Ok(())
}
