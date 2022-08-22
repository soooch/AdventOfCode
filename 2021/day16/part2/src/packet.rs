//! Logic for parsing and computing the "packet" language in day 16 of Advent of Code 2021

use crate::util::{Fencable, FromBits};
use std::cmp;
use std::ops::{Add, Mul};

/// Returns either the solution to the given packet or an error.
///
/// # Arguments
/// * `bits` - a mutable reference to an [Iterator] over the bits of the packet.
pub fn compute(bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str> {
    let (_version, operation) = get_header(bits)?;

    use Operation as Op;
    match operation {
        Op::Literal => literal(bits),
        rest => {
            let length_type_id = bits
                .next()
                .ok_or("Expected length type ID, but bit stream ended")?;

            let reduce = move |f: fn(usize, usize) -> usize, bits| match length_type_id {
                false => reduce_t0(f, bits),
                true => reduce_t1(f, bits),
            };

            let compare = move |f: fn(usize, usize) -> bool, bits| match length_type_id {
                false => compare_t0(f, bits),
                true => compare_t1(f, bits),
            };

            match rest {
                Op::Sum => reduce(Add::add, bits),
                Op::Product => reduce(Mul::mul, bits),
                Op::Minimum => reduce(cmp::min, bits),
                Op::Maximim => reduce(cmp::max, bits),
                Op::Greater => compare(|a, b| a > b, bits),
                Op::Less => compare(|a, b| a < b, bits),
                Op::Equal => compare(|a, b| a == b, bits),
                _ => unsafe { std::hint::unreachable_unchecked() },
            }
        }
    }
}

fn literal(bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str> {
    Ok(usize::from_bits(&mut LiteralBits::new(bits)))
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
            if self.last == true {
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

fn reduce_t0<F>(mut f: F, bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> usize,
{
    let num_bits = get_length_t0(bits)?;

    // TODO: try to remove this line
    let mut bits: Box<dyn Iterator<Item = bool>> = Box::new(bits);

    let mut subpacket_bits = bits.fence(num_bits as usize);

    let mut accum = compute(&mut subpacket_bits)?;

    while subpacket_bits.remaining() > 0 {
        accum = f(accum, compute(&mut subpacket_bits)?);
    }
    Ok(accum)
}

fn reduce_t1<F>(mut f: F, bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> usize,
{
    let num_packets = get_length_t1(bits)? - 1;

    let mut accum = compute(bits)?;

    for _ in 0..num_packets {
        accum = f(accum, compute(bits)?);
    }

    Ok(accum)
}

fn compare_t0<F>(mut f: F, bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> bool,
{
    let num_bits = get_length_t0(bits)?;

    // TODO: try to remove this line
    let mut bits: Box<dyn Iterator<Item = bool>> = Box::new(bits);

    let mut subpacket_bits = bits.fence(num_bits as usize);

    let first = compute(&mut subpacket_bits)?;
    let second = compute(&mut subpacket_bits)?;

    if subpacket_bits.remaining() != 0 {
        Err("Comparison operation held more than two subpackets")
    } else {
        Ok(f(first, second) as usize)
    }
}

fn compare_t1<F>(mut f: F, bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> bool,
{
    let num_packets = get_length_t1(bits)?;
    if num_packets != 2 {
        Err("Comparison operation held more than two subpackets")
    } else {
        let first = compute(bits)?;
        let second = compute(bits)?;

        Ok(f(first, second) as usize)
    }
}

#[inline]
fn get_header(bits: &mut impl Iterator<Item = bool>) -> Result<(u8, Operation), &'static str> {
    let mut version_bits = bits.fence(3);
    let version = u8::from_bits(&mut version_bits);
    if version_bits.remaining() != 0 {
        return Err("Expected packet version, but bit stream ended");
    }
    let mut operation_bits = bits.fence(3);
    let operation = u8::from_bits(&mut operation_bits).try_into()?;
    if operation_bits.remaining() != 0 {
        return Err("Expected packet type ID, but bit stream ended");
    }

    Ok((version, operation))
}

#[inline]
fn get_length_t0(bits: &mut impl Iterator<Item = bool>) -> Result<u16, &'static str> {
    let mut length_bits = bits.fence(15);
    let num_bits = u16::from_bits(&mut length_bits);
    if length_bits.remaining() != 0 {
        return Err("Expected type 0 length, but bit stream ended");
    }
    Ok(num_bits)
}

#[inline]
fn get_length_t1(bits: &mut impl Iterator<Item = bool>) -> Result<u16, &'static str> {
    let mut length_bits = bits.fence(11);
    let num_packets = u16::from_bits(&mut length_bits);
    if length_bits.remaining() != 0 {
        return Err("Expected type 1 length, but bit stream ended");
    }
    Ok(num_packets)
}

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
    type Error = &'static str;

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
