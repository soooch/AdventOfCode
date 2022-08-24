use day16_part2::nibble::Nibble;
use day16_part2::packet;

pub fn main() {
    let input = include_str!("../../input.txt");

    let mut bits = input
        .trim()
        .bytes()
        .map(|b| Nibble::from_hex_ascii(b).unwrap())
        .flat_map(Nibble::into_bits);

    let solution = packet::compute(&mut bits).unwrap().value;

    println!("{solution}");
}
