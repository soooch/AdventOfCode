#![feature(portable_simd)]
#![feature(stdsimd)]
#![feature(array_windows)]

#![feature(array_zip)]

use std::simd::*;
use std::arch::x86_64::*;

use align_data::{include_aligned, Align16};

const LINE_LEN: usize = 13;
const HW_LANES: usize = 16;

fn main() {
    let input = include_aligned!(Align16, "../../input.txt");
    let num_words = input.len() as u16 / LINE_LEN as u16;

    let (pre, aligned, unaligned) = unsafe {
        input.align_to::<[Simd<u8, HW_LANES>; LINE_LEN]>()
    };
    // start should be aligned
    assert_eq!(pre.len(), 0);

    let aligned = aligned
        .iter()
        .map(|words| words.map(|word| unsafe { _mm256_cvtepu8_epi16(word.into()) }))
        .map(|words| words.map(|word| u16x16::from(word)))
        .reduce(|acc, word| acc.zip(word).map(|(a, w)| a + w))
        .unwrap();

    // TODO: maybe try to keep these inside registers and shift
    // instead of storing/loading
    let (_, aligned, _) = unsafe { aligned.align_to::<u16>() };

    // three bytes of extra padding will allow me to not have to handle
    // the last line special case.
    // This is not great, but probably fine. I'm just reading from the stack,
    // so it won't fail, unless I'm already super close to an overflow
    let aligned = unsafe {
        std::slice::from_raw_parts(
            aligned.as_ptr(),
            aligned.len() + (HW_LANES - LINE_LEN)
        )
    };

    let aligned_sums = aligned
        .array_windows()
        .step_by(LINE_LEN)
        .map(|&word| u16x16::from(word))
        .sum::<Simd<u16, HW_LANES>>();

    let sums = if unaligned.len() > 0 {
        // horrible memory safety time bomb. (this could be on the heap).
        // but so much cleaner to assume I can read from the 3 bytes after
        // without segfaulting
        // should probably load last 16 elements, then lane shift,
        // but that would be somewhat annoying and ugly and I'm done.
        let unaligned = unsafe {
            std::slice::from_raw_parts(
                unaligned.as_ptr(),
                unaligned.len() + (HW_LANES - LINE_LEN)
            )
        };

        let unaligned_sums = unaligned
            .array_windows()
            .step_by(LINE_LEN)
            .map(|&word| Simd::from(word))
            .map(|word| unsafe { _mm256_cvtepu8_epi16(word.into()) })
            .map(|word| Simd::from(word))
            .sum::<Simd<u16, HW_LANES>>();

        aligned_sums + unaligned_sums
    } else {
        aligned_sums
    };

    let midpoint = b'0' as u16 * num_words + num_words / 2;

    let gamma = unsafe {
        let midpoint = Simd::splat(midpoint);
        let gamma = _mm256_cmpgt_epi16(sums.into(), midpoint.into());
        let gamma_lower = _mm256_extracti128_si256::<0>(gamma);
        let gamma_upper = _mm256_extracti128_si256::<1>(gamma);
        let gamma = _mm_packs_epi16(gamma_lower, gamma_upper);
        let gamma = u8x16::from(gamma).reverse();
        _mm_movemask_epi8(gamma.into()) as u32 >> 4
    };

    let epsilon = !gamma & 0x0FFF;

    println!("{}", gamma * epsilon);
}

