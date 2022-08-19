use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt").trim();

    let mut crab_pos = input
        .split(',')
        .map(|pos| u16::from_str(pos).unwrap())
        .collect::<Vec<_>>();

    crab_pos.sort_unstable();

    let midpoint = crab_pos.len() / 2;
    let median = crab_pos[midpoint];
    let (lo_pos, hi_pos) = crab_pos.split_at(midpoint);

    let lo_cost = lo_pos.iter().map(|pos| (median - pos) as u32).sum::<u32>();
    let hi_cost = hi_pos.iter().map(|pos| (pos - median) as u32).sum::<u32>();

    println!("{}", lo_cost + hi_cost);
}
