#![feature(str_split_once)]
use std::io::{self, Read};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("could not read input");
    let now = Instant::now();
    
    let bags = buffer.lines().enumerate()
        .filter_map(|(idx, line)| {
            if let Some((bag, contents)) = line.split_once(" bags contain ") {
                return Some((bag, (idx, contents)));
            }
            None
        })
        .collect::<HashMap<_, _>>();
    
    let mut adj_mat = vec![0; bags.len() * bags.len()];

    for (idx, interior) in bags.values() {
        for bag in interior.split(", ") {
            if let Some((num, color)) = bag.split_once(' ') {
                if let Ok(num) = num.parse::<u32>() {
                    if let Some((color, _)) = color.rsplit_once(' ') {
                        adj_mat[bags[color].0 * bags.len() + idx] = num;
                    }
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();
    
    to_visit.push(bags["shiny gold"].0);

    while !to_visit.is_empty() {
        let bag = to_visit.pop().unwrap();
        visited.insert(bag);
        for (container, amount) in adj_mat[bag * bags.len()..bag * bags.len() + bags.len()].iter().enumerate() {
            if !visited.contains(&container) {
                if amount > &0 {
                    to_visit.push(container);
                }
            }
        }
    }
    println!("{}", now.elapsed().as_micros());

    println!("{}", visited.len() - 1);
    
}
