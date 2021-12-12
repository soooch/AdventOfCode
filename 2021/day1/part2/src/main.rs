#![feature(array_windows)]

fn main() {
    let input = include_str!("../../input.txt");

    let depths = input
        .lines()
        .filter_map(|num| num.parse().ok())
        .collect::<Vec<u32>>();

    let windows = depths
        .array_windows::<3>()
        .map(|a| a.iter().sum())
        .collect::<Vec<u32>>();

    let answer = windows
        .array_windows()
        .filter(|[a, b]| a < b)
        .count();

    println!("{}", answer);

}
