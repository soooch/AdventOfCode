#![feature(portable_simd)]
#![feature(stdsimd)]
#![feature(array_windows)]

use std::simd::*;
use std::arch::x86_64::*;

const WORD_LEN: usize = 12;

fn main() {
    let input = include_str!("../../input.txt");
    let input = [input, "000"].concat();

    let (num_words, sums) = input
        .as_bytes()
        .array_windows()
        .step_by(WORD_LEN + 1)
        .map(|&word| u8x16::from(word))
        .map(|word| unsafe { _mm256_cvtepu8_epi16(word.into()) })
        .map(|word| u16x16::from(word) - u16x16::splat(b'0'.into()))
        .fold((0, u16x16::splat(0)), |(count, sum), word| (count + 1, sum + word));

    let gamma = unsafe {
        let gamma = _mm256_cmpgt_epi16(sums.into(), u16x16::splat(num_words / 2).into());
        let gamma_lower = _mm256_extracti128_si256::<0>(gamma);
        let gamma_upper = _mm256_extracti128_si256::<1>(gamma);
        let gamma = _mm_packs_epi16(gamma_lower, gamma_upper);
        let gamma = u8x16::from(gamma).reverse();
        _mm_movemask_epi8(gamma.into()) >> 4
    };

    let epsilon = !gamma & 0x0FFF;

    println!("{}", gamma * epsilon);
}

