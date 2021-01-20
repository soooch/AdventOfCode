use std::io::{self, Read};

fn main() -> io::Result<()> {
    const A: u32 = 1;
    const B: u32 = 2;
    const C: u32 = 4;
    const D: u32 = 8;
    const E: u32 = 16;
    const F: u32 = 32;
    const G: u32 = 64;
    const H: u32 = 128;
    const I: u32 = 256;
    const J: u32 = 512;
    const K: u32 = 1024;
    const L: u32 = 2048;
    const M: u32 = 4096;
    const N: u32 = 8192;
    const O: u32 = 16384;
    const P: u32 = 32768;
    const Q: u32 = 65536;
    const R: u32 = 131072;
    const S: u32 = 262144;
    const T: u32 = 524288;
    const U: u32 = 1048576;
    const V: u32 = 2097152;
    const W: u32 = 4194304;
    const X: u32 = 8388608;
    const Y: u32 = 16777216;
    const Z: u32 = 33554432;

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let sum = buffer.split("\n\n")
        .fold(0, |sum_acc, group| {
            let mut yesses = 0;
            group.chars().for_each(|q| {
                match q {
                    'a' => yesses |= A,
                    'b' => yesses |= B,
                    'c' => yesses |= C,
                    'd' => yesses |= D,
                    'e' => yesses |= E,
                    'f' => yesses |= F,
                    'g' => yesses |= G,
                    'h' => yesses |= H,
                    'i' => yesses |= I,
                    'j' => yesses |= J,
                    'k' => yesses |= K,
                    'l' => yesses |= L,
                    'm' => yesses |= M,
                    'n' => yesses |= N,
                    'o' => yesses |= O,
                    'p' => yesses |= P,
                    'q' => yesses |= Q,
                    'r' => yesses |= R,
                    's' => yesses |= S,
                    't' => yesses |= T,
                    'u' => yesses |= U,
                    'v' => yesses |= V,
                    'w' => yesses |= W,
                    'x' => yesses |= X,
                    'y' => yesses |= Y,
                    'z' => yesses |= Z,
                    _ => (),
                }
            });
            sum_acc + yesses.count_ones()
        });

    println!("{}", sum);

    Ok(())
}
