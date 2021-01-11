use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let num_valid = buffer.split("\n\n").fold(0, |count, passport| {
        let mut has_cred: [bool; 7] = [false; 7];
        for cred in passport.split_ascii_whitespace() {
            let (field, data) = cred.split_at(4);
            match field {
                "byr:" => has_cred[0] = validate_byr(data),
                "iyr:" => has_cred[1] = validate_iyr(data),
                "eyr:" => has_cred[2] = validate_eyr(data),
                "hgt:" => has_cred[3] = validate_hgt(data),
                "hcl:" => has_cred[4] = validate_hcl(data),
                "ecl:" => has_cred[5] = validate_ecl(data),
                "pid:" => has_cred[6] = validate_pid(data),
                _ => (),
            }
        }
        if has_cred.contains(&false) {count} else {count + 1}
    });

    println!("{}", num_valid);

    Ok(())
}

fn validate_byr(byr: &str) -> bool {
    num_str_in_range(byr, 1920, 2002)
}

fn validate_iyr(iyr: &str) -> bool {
    num_str_in_range(iyr, 2010, 2020)
}

fn validate_eyr(eyr: &str) -> bool {
    num_str_in_range(eyr, 2020, 2030)
}

fn validate_hgt(hgt: &str) -> bool {
    if hgt.ends_with("cm") {
        num_str_in_range(hgt.trim_end_matches("cm"), 150, 193)
    } else if hgt.ends_with("in") {
        num_str_in_range(hgt.trim_end_matches("in"), 59, 76)
    } else {false}
}

fn num_str_in_range(num_str: &str, start: i32, end: i32) -> bool {
    (start..=end).contains(&num_str.parse().unwrap_or(end + 1))
}

fn validate_hcl(hcl: &str) -> bool {
    if hcl.starts_with('#') {
        if hcl.len() == 7 {
            return !hcl.trim_start_matches('#').chars().any(|c| !c.is_ascii_hexdigit());
        }
    }
    false
}

fn validate_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn validate_pid(pid: &str) -> bool {
    if pid.len() == 9 {
        pid.parse::<i32>().is_ok()
    }
    else {false}
}