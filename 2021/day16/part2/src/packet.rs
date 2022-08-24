//! Logic for parsing and computing the "packet" language in day 16 of Advent of Code 2021

use core::cmp;
use core::ops::{Add, Mul};
use num_traits::PrimInt;

/// Returns either the solution to the given packet or an error.
///
/// # Arguments
/// * `bits` - a mutable reference to an [Iterator] over the bits of the packet.
pub fn compute(bits: &mut impl Iterator<Item = bool>) -> ComputationResult {
    let ((_version, operation), header_bits_read) = get_header(bits)?;

    use Operation as Op;
    let mut operation = match operation {
        Op::Sum => reduce(Add::add, bits),
        Op::Product => reduce(Mul::mul, bits),
        Op::Minimum => reduce(cmp::min, bits),
        Op::Maximim => reduce(cmp::max, bits),
        Op::Literal => literal(bits),
        Op::Greater => compare(|a, b| a > b, bits),
        Op::Less => compare(|a, b| a < b, bits),
        Op::Equal => compare(|a, b| a == b, bits),
    }?;
    operation.bits_read += header_bits_read;
    Ok(operation)
}

fn literal(bits: &mut impl Iterator<Item = bool>) -> ComputationResult {
    let (literal, bits_read) = usize::from_bits(&mut LiteralBits::new(bits));
    // 5 bits read from bit stream per nibble of literal
    let bits_read = (bits_read / 4) * 5;
    Ok(Computation::new(literal, bits_read))
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

#[inline]
fn reduce(
    f: fn(usize, usize) -> usize,
    bits: &mut impl Iterator<Item = bool>,
) -> ComputationResult {
    let (length_type_id, length_type_bits_read) = get_length_type(bits)?;
    let mut reduction = match length_type_id {
        false => reduce_t0(f, bits),
        true => reduce_t1(f, bits),
    }?;
    reduction.bits_read += length_type_bits_read;
    Ok(reduction)
}

fn reduce_t0(
    f: fn(usize, usize) -> usize,
    bits: &mut impl Iterator<Item = bool>,
) -> ComputationResult {
    let (num_bits, num_bits_bits_read) = get_length_t0(bits)?;
    let num_bits = num_bits as usize;

    let mut accum = compute(bits)?;

    while accum.bits_read != num_bits {
        let subpacket = compute(bits)?;
        accum.value = f(accum.value, subpacket.value);
        accum.bits_read += subpacket.bits_read;
    }
    accum.bits_read += num_bits_bits_read;

    Ok(accum)
}

fn reduce_t1(
    f: fn(usize, usize) -> usize,
    bits: &mut impl Iterator<Item = bool>,
) -> ComputationResult {
    let (num_packets, num_packets_bits_read) = get_length_t1(bits)?;

    let mut accum = compute(bits)?;
    let num_packets = num_packets - 1;

    for _ in 0..num_packets {
        let subpacket = compute(bits)?;
        accum.value = f(accum.value, subpacket.value);
        accum.bits_read += subpacket.bits_read;
    }
    accum.bits_read += num_packets_bits_read;

    Ok(accum)
}

#[inline]
fn compare(
    f: fn(usize, usize) -> bool,
    bits: &mut impl Iterator<Item = bool>,
) -> ComputationResult {
    let (length_type_id, length_type_bits_read) = get_length_type(bits)?;
    let mut comparison = match length_type_id {
        false => compare_t0(f, bits),
        true => compare_t1(f, bits),
    }?;
    comparison.bits_read += length_type_bits_read;
    Ok(comparison)
}

fn compare_t0(
    f: fn(usize, usize) -> bool,
    bits: &mut impl Iterator<Item = bool>,
) -> ComputationResult {
    let (num_bits, num_bits_bits_read) = get_length_t0(bits)?;

    let first = compute(bits)?;
    let second = compute(bits)?;

    let packet_bits_read = first.bits_read + second.bits_read;

    if packet_bits_read != num_bits as usize {
        Err("Comparison operation length field did not match exactly two subpackets")
    } else {
        let computation = Computation {
            value: f(first.value, second.value) as usize,
            bits_read: num_bits_bits_read + packet_bits_read,
        };

        Ok(computation)
    }
}

fn compare_t1(
    f: fn(usize, usize) -> bool,
    bits: &mut impl Iterator<Item = bool>,
) -> ComputationResult {
    let (num_packets, num_packets_bits_read) = get_length_t1(bits)?;
    if num_packets != 2 {
        Err("Comparison operation length field did not match exactly two subpackets")
    } else {
        let first = compute(bits)?;
        let second = compute(bits)?;

        let computation = Computation {
            value: f(first.value, second.value) as usize,
            bits_read: num_packets_bits_read + first.bits_read + second.bits_read,
        };

        Ok(computation)
    }
}

#[inline]
fn get_header(
    bits: &mut impl Iterator<Item = bool>,
) -> Result<((u8, Operation), usize), ComputeError> {
    const VERSION_FIELD_SIZE: usize = 3;
    const TYPE_ID_FIELD_SIZE: usize = 3;
    let version = try_read_bits(bits, VERSION_FIELD_SIZE)
        .ok_or("Expected packet version, but bit stream ended")?;
    let operation = try_read_bits::<u8>(bits, TYPE_ID_FIELD_SIZE)
        .ok_or("Expected packet type ID, but bit stream ended")?
        .try_into()?;

    Ok((
        (version, operation),
        VERSION_FIELD_SIZE + TYPE_ID_FIELD_SIZE,
    ))
}

#[inline]
fn get_length_type(bits: &mut impl Iterator<Item = bool>) -> Result<(bool, usize), ComputeError> {
    const LENGTH_TYPE_FIELD_SIZE: usize = 1;
    let length_type_id = bits
        .next()
        .ok_or("Expected length type ID, but bit stream ended")?;
    Ok((length_type_id, LENGTH_TYPE_FIELD_SIZE))
}

#[inline]
fn get_length_t0(bits: &mut impl Iterator<Item = bool>) -> Result<(u16, usize), ComputeError> {
    const T0_LEN_FIELD_SIZE: usize = 15;
    Ok((
        try_read_bits(bits, T0_LEN_FIELD_SIZE)
            .ok_or("Expected type 0 length, but bit stream ended")?,
        T0_LEN_FIELD_SIZE,
    ))
}

#[inline]
fn get_length_t1(bits: &mut impl Iterator<Item = bool>) -> Result<(u16, usize), ComputeError> {
    const T1_LEN_FIELD_SIZE: usize = 11;
    Ok((
        try_read_bits(bits, T1_LEN_FIELD_SIZE)
            .ok_or("Expected type 1 length, but bit stream ended")?,
        T1_LEN_FIELD_SIZE,
    ))
}

#[inline(always)]
fn try_read_bits<T: FromBits>(bits: &mut impl Iterator<Item = bool>, n: usize) -> Option<T> {
    let mut number_bits = bits.take(n);
    let (number, bits_read) = T::from_bits(&mut number_bits);
    if bits_read != n {
        return None;
    }
    Some(number)
}

/// A struct for representing the final and intermediate results of packet computation
/// `bits_read` is needed to handle length type 0 packets
#[derive(Clone, Copy)]
pub struct Computation {
    pub value: usize,
    pub bits_read: usize,
}

impl Computation {
    #[inline]
    fn new(value: usize, bits_read: usize) -> Self {
        Computation { value, bits_read }
    }
}

type ComputeError = &'static str;

type ComputationResult = Result<Computation, ComputeError>;

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

/// A trait providing the [from_bits](FromBits::from_bits) method for integers
trait FromBits: PrimInt + From<bool> {
    /// Contructs `Self` from an iterator over bits from MSB to LSB
    /// Also counts the number of bits consumed for type 0 packets
    #[inline]
    fn from_bits<I>(bits: I) -> (Self, usize)
    where
        I: Iterator<Item = bool>,
    {
        bits.fold((Self::zero(), 0), |(acc, count), b| {
            ((acc << 1) | b.into(), count + 1)
        })
    }
}

impl FromBits for u8 {}
impl FromBits for u16 {}
impl FromBits for usize {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bits_test_u8() {
        let (num, count) = u8::from_bits([1, 0, 1, 1, 0, 1, 1, 1].into_iter().map(|b| {
            if b == 1 {
                true
            } else {
                false
            }
        }));

        assert_eq!(num, 0b10110111);
        assert_eq!(count, 8);

        let (num, count) =
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
        assert_eq!(count, 6);
    }

    #[test]
    fn from_bits_test_u16() {
        let (num, count) = u16::from_bits(
            [1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]
                .into_iter()
                .map(|b| if b == 1 { true } else { false }),
        );

        assert_eq!(num, 0b1011011101010111);
        assert_eq!(count, 16);
    }
}
