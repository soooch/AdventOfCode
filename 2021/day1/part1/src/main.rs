#![feature(array_windows)]

fn main() {
    let input = include_str!("../../input.txt");

    let nums = input
        .lines()
        .filter_map(|num| num.parse().ok())
        .collect::<Vec<u32>>();

    let answer = nums
        .array_windows::<2>()
        .filter(|[a, b]| a < b)
        .count();

    println!("{}", answer);
}

