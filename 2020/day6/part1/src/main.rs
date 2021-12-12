fn main() {
    let sum = include_str!("../../input.txt")
        .split("\n\n")
        .map(|group| group
            .bytes()
            .filter(|q| (b'a'..=b'z').contains(&q))
            .map(|q| 1u32 << (q - b'a'))
            .fold(0, std::ops::BitOr::bitor)
            .count_ones())
        .sum::<u32>();

    println!("{}", sum);
}