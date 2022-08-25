use criterion::{criterion_group, criterion_main, Criterion};
use day16_part2::{nibble::Nibble, packet};

fn bench_compute() -> usize {
    let input = include_str!("../../input.txt");

    let bits = input
        .trim()
        .bytes()
        .map(|b| Nibble::from_hex_ascii(b).unwrap())
        .flat_map(Nibble::into_bits);

    packet::solve(bits).unwrap()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(|| bench_compute()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
