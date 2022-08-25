//! Logic for parsing and computing the "packet" language in day 16 of Advent of Code 2021

use crate::util::{CountIter, Countable, FromBits};
use core::cmp;
use core::ops::{Add, Mul};

pub fn solve(bits: impl Iterator<Item = bool>) -> ComputationResult {
    compute(&mut bits.counted())
}

/// Returns either the solution to the given packet or an error.
///
/// # Arguments
/// * `bits` - a mutable reference to an [Iterator] over the bits of the packet.
fn compute<I>(bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    let (_version, operation) = get_header(bits)?;

    use Operation as Op;
    match operation {
        Op::Sum => reduce(Add::add, bits),
        Op::Product => reduce(Mul::mul, bits),
        Op::Minimum => reduce(cmp::min, bits),
        Op::Maximim => reduce(cmp::max, bits),
        Op::Literal => literal(bits),
        Op::Greater => compare(|a, b| a > b, bits),
        Op::Less => compare(|a, b| a < b, bits),
        Op::Equal => compare(|a, b| a == b, bits),
    }
}

fn literal<I>(bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    Ok(usize::from_bits(&mut LiteralBits::new(bits)))
}

#[inline]
fn reduce<I>(f: fn(usize, usize) -> usize, bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    let length_type_id = get_length_type(bits)?;
    match length_type_id {
        false => reduce_t0(f, bits),
        true => reduce_t1(f, bits),
    }
}

fn reduce_t0<I>(f: fn(usize, usize) -> usize, bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    let num_bits = get_length_t0(bits)?;

    let final_bits_read = bits.iter_count() + num_bits as usize;

    let mut accum = compute(bits)?;

    while bits.iter_count() != final_bits_read {
        let subpacket = compute(bits)?;
        accum = f(accum, subpacket);
    }

    Ok(accum)
}

fn reduce_t1<I>(f: fn(usize, usize) -> usize, bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    let num_packets = get_length_t1(bits)?;

    let mut accum = compute(bits)?;
    let num_packets = num_packets - 1;

    for _ in 0..num_packets {
        let subpacket = compute(bits)?;
        accum = f(accum, subpacket);
    }

    Ok(accum)
}

#[inline]
fn compare<I>(f: fn(usize, usize) -> bool, bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    let length_type_id = get_length_type(bits)?;
    match length_type_id {
        false => compare_t0(f, bits),
        true => compare_t1(f, bits),
    }
}

fn compare_t0<I>(f: fn(usize, usize) -> bool, bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    let num_bits = get_length_t0(bits)?;

    let bits_read = bits.iter_count();

    let first = compute(bits)?;
    let second = compute(bits)?;

    let packet_bits_read = bits.iter_count() - bits_read;

    if packet_bits_read != num_bits as usize {
        Err("Comparison operation length field did not match exactly two subpackets")
    } else {
        Ok(f(first, second) as usize)
    }
}

fn compare_t1<I>(f: fn(usize, usize) -> bool, bits: &mut CountIter<I>) -> ComputationResult
where
    I: Iterator<Item = bool>,
{
    let num_packets = get_length_t1(bits)?;
    if num_packets != 2 {
        Err("Comparison operation length field did not match exactly two subpackets")
    } else {
        let first = compute(bits)?;
        let second = compute(bits)?;

        Ok(f(first, second) as usize)
    }
}

#[inline]
fn get_header<I>(bits: &mut CountIter<I>) -> Result<(u8, Operation), ComputeError>
where
    I: Iterator<Item = bool>,
{
    const VERSION_FIELD_SIZE: usize = 3;
    const TYPE_ID_FIELD_SIZE: usize = 3;
    let version = try_read_bits(bits, VERSION_FIELD_SIZE)
        .ok_or("Expected packet version, but bit stream ended")?;
    let operation = try_read_bits::<u8, _>(bits, TYPE_ID_FIELD_SIZE)
        .ok_or("Expected packet type ID, but bit stream ended")?
        .try_into()?;

    Ok((version, operation))
}

#[inline]
fn get_length_type<I>(bits: &mut CountIter<I>) -> Result<bool, ComputeError>
where
    I: Iterator<Item = bool>,
{
    //const LENGTH_TYPE_FIELD_SIZE: usize = 1;
    bits.next()
        .ok_or("Expected length type ID, but bit stream ended")
}

#[inline]
fn get_length_t0<I>(bits: &mut CountIter<I>) -> Result<u16, ComputeError>
where
    I: Iterator<Item = bool>,
{
    const T0_LEN_FIELD_SIZE: usize = 15;
    try_read_bits(bits, T0_LEN_FIELD_SIZE).ok_or("Expected type 0 length, but bit stream ended")
}

#[inline]
fn get_length_t1<I>(bits: &mut CountIter<I>) -> Result<u16, ComputeError>
where
    I: Iterator<Item = bool>,
{
    const T1_LEN_FIELD_SIZE: usize = 11;
    try_read_bits(bits, T1_LEN_FIELD_SIZE).ok_or("Expected type 1 length, but bit stream ended")
}

#[inline(always)]
fn try_read_bits<T, I>(bits: &mut CountIter<I>, n: usize) -> Option<T>
where
    I: Iterator<Item = bool>,
    T: FromBits,
{
    let bits_read = bits.iter_count();
    let mut number_bits = bits.take(n);
    let number = T::from_bits(&mut number_bits);
    let bits_read = bits.iter_count() - bits_read;
    if bits_read == n {
        Some(number)
    } else {
        None
    }
}

type ComputeError = &'static str;

type ComputationResult = Result<usize, ComputeError>;

enum Operation {
    Sum,
    Product,
    Minimum,
    Maximim,
    Literal,
    Greater,
    Less,
    Equal,
}

impl TryFrom<u8> for Operation {
    type Error = ComputeError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximim),
            4 => Ok(Self::Literal),
            5 => Ok(Self::Greater),
            6 => Ok(Self::Less),
            7 => Ok(Self::Equal),
            _ => Err("Unrecognized operation type"),
        }
    }
}

struct LiteralBits<'a, I> {
    inner: &'a mut I,
    state: u8,
    last: bool,
}

impl<'a, I> LiteralBits<'a, I> {
    #[inline]
    fn new(inner: &'a mut I) -> Self {
        LiteralBits {
            inner,
            state: 0,
            last: false,
        }
    }
}

impl<'a, I> Iterator for LiteralBits<'a, I>
where
    I: Iterator<Item = bool>,
{
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.state == 0 {
            if self.last {
                return None;
            } else {
                let group_id = self.inner.next()?;
                if !group_id {
                    self.last = true;
                }
                self.state = 4;
            }
        }
        self.state -= 1;
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bits_test_u8() {
        let num = u8::from_bits([1, 0, 1, 1, 0, 1, 1, 1].into_iter().map(|b| {
            if b == 1 {
                true
            } else {
                false
            }
        }));

        assert_eq!(num, 0b10110111);

        let num =
            u8::from_bits([1, 0, 1, 1, 0, 1].into_iter().map(
                |b| {
                    if b == 1 {
                        true
                    } else {
                        false
                    }
                },
            ));

        assert_eq!(num, 0b101101);
    }

    #[test]
    fn from_bits_test_u16() {
        let num = u16::from_bits(
            [1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]
                .into_iter()
                .map(|b| if b == 1 { true } else { false }),
        );

        assert_eq!(num, 0b1011011101010111);
    }
}
