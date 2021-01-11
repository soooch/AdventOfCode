use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let num_valid = buffer.split("\n\n").fold(0, |count, passport| {
        let mut has_cred: [bool; 7] = [false; 7];
        for cred in passport.split_ascii_whitespace() {
            match &cred[0..3] {
                "byr" => has_cred[0] = true,
                "iyr" => has_cred[1] = true,
                "eyr" => has_cred[2] = true,
                "hgt" => has_cred[3] = true,
                "hcl" => has_cred[4] = true,
                "ecl" => has_cred[5] = true,
                "pid" => has_cred[6] = true,
                _ => (),
            }
        }
        if has_cred.contains(&false) {count} else {count + 1}
    });

    println!("{}", num_valid);

    Ok(())
}

