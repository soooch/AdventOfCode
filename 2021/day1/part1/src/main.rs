#![feature(array_windows)]

fn main() {
    let input = include_str!("../../input.txt");

    let nums = input
        .lines()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u32>>();

    let answer = nums
        .array_windows()
        .filter(|[a, b]| a < b)
        .count();

    println!("{}", answer);
}

