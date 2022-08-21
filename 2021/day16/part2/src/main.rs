use num_traits::PrimInt;
use std::cmp;
use std::iter::FusedIterator;
use std::mem::MaybeUninit;
use std::ops::{Add, Mul};

fn main() {
    let input = include_str!("../../input.txt");

    let mut bits = input
        .trim()
        .bytes()
        .map(|b| Nibble::from_hex_ascii(b).unwrap())
        .flat_map(Nibble::into_bits);

    let solution = compute_packet(&mut bits).unwrap();

    println!("{solution}");
}

fn compute_packet(bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str> {
    let (_version, operation) = get_packet_header(bits)?;

    use Operation as Op;
    match operation {
        Op::Literal => literal_packet(bits),
        rest => {
            let length_type_id = bits
                .next()
                .ok_or("Expected length type ID, but bit stream ended")?;

            let reduce_packets = move |f: fn(usize, usize) -> usize, bits| match length_type_id {
                false => reduce_packets_t0(f, bits),
                true => reduce_packets_t1(f, bits),
            };

            let compare_packets = move |f: fn(usize, usize) -> bool, bits| match length_type_id {
                false => compare_packets_t0(f, bits),
                true => compare_packets_t1(f, bits),
            };

            match rest {
                Op::Sum => reduce_packets(Add::add, bits),
                Op::Product => reduce_packets(Mul::mul, bits),
                Op::Minimum => reduce_packets(cmp::min, bits),
                Op::Maximim => reduce_packets(cmp::max, bits),
                Op::Greater => compare_packets(|a, b| a > b, bits),
                Op::Less => compare_packets(|a, b| a < b, bits),
                Op::Equal => compare_packets(|a, b| a == b, bits),
                _ => unreachable!(),
            }
        }
    }
}

fn literal_packet(bits: &mut impl Iterator<Item = bool>) -> Result<usize, &'static str> {
    let append_bits = |acc, frag| (acc << 4) | u8::from_bits(frag) as usize;
    let mut lit_acc = 0;
    loop {
        match bits.next_n::<5>() {
            Some([true, frag @ ..]) => lit_acc = append_bits(lit_acc, frag),
            Some([false, frag @ ..]) => {
                let lit_acc = append_bits(lit_acc, frag);
                break Ok(lit_acc);
            }
            None => break Err("Expected literal value, but bit stream ended"),
        }
    }
}

fn reduce_packets_t0<F>(
    mut f: F,
    bits: &mut impl Iterator<Item = bool>,
) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> usize,
{
    let num_bits = get_t0_bit_count(bits)?;

    // type erasure. this line sucks
    let mut bits: Box<dyn Iterator<Item = bool>> = Box::new(bits);

    let mut subpacket_bits = bits.fence(num_bits as usize);

    let mut accum = compute_packet(&mut subpacket_bits)?;

    while subpacket_bits.remaining() > 0 {
        accum = f(accum, compute_packet(&mut subpacket_bits)?);
    }
    Ok(accum)
}

fn reduce_packets_t1<F>(
    mut f: F,
    bits: &mut impl Iterator<Item = bool>,
) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> usize,
{
    let num_packets = get_t1_packet_count(bits)? - 1;

    let mut accum = compute_packet(bits)?;

    for _ in 0..num_packets {
        accum = f(accum, compute_packet(bits)?);
    }

    Ok(accum)
}

fn compare_packets_t0<F>(
    mut f: F,
    bits: &mut impl Iterator<Item = bool>,
) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> bool,
{
    let num_bits = get_t0_bit_count(bits)?;

    // type erasure. this line sucks
    let mut bits: Box<dyn Iterator<Item = bool>> = Box::new(bits);

    let mut subpacket_bits = bits.fence(num_bits as usize);

    let first = compute_packet(&mut subpacket_bits)?;
    let second = compute_packet(&mut subpacket_bits)?;

    if subpacket_bits.remaining() != 0 {
        Err("Comparison operation held more than two subpackets")
    } else {
        Ok(f(first, second) as usize)
    }
}

fn compare_packets_t1<F>(
    mut f: F,
    bits: &mut impl Iterator<Item = bool>,
) -> Result<usize, &'static str>
where
    F: FnMut(usize, usize) -> bool,
{
    let num_packets = get_t1_packet_count(bits)?;
    if num_packets != 2 {
        Err("Comparison operation held more than two subpackets")
    } else {
        let first = compute_packet(bits)?;
        let second = compute_packet(bits)?;

        Ok(f(first, second) as usize)
    }
}

fn get_packet_header(
    bits: &mut impl Iterator<Item = bool>,
) -> Result<(u8, Operation), &'static str> {
    let version = bits
        .next_n::<3>()
        .ok_or("Expected packet version, but bit stream ended")?;
    let type_id = bits
        .next_n::<3>()
        .ok_or("Expected packet type ID, but bit stream ended")?;

    let version = u8::from_bits(version);
    let operation = u8::from_bits(type_id).try_into()?;

    Ok((version, operation))
}

fn get_t0_bit_count(bits: &mut impl Iterator<Item = bool>) -> Result<u16, &'static str> {
    let num_bits = bits
        .next_n::<15>()
        .ok_or("Expected type 0 length, but bit stream ended")?;

    Ok(u16::from_bits(num_bits))
}

fn get_t1_packet_count(bits: &mut impl Iterator<Item = bool>) -> Result<u16, &'static str> {
    let num_packets = bits
        .next_n::<11>()
        .ok_or("Expected type 1 length, but bit stream ended")?;

    Ok(u16::from_bits(num_packets))
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

trait FromBits: PrimInt + From<bool> {
    fn from_bits<I>(bits: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        bits.into_iter()
            .fold(Self::zero(), |acc, b| (acc << 1) | b.into())
    }
}

impl FromBits for u8 {}
impl FromBits for u16 {}

// Nibble stuff
struct Nibble(u8);

impl Nibble {
    const fn from_hex_ascii(hex: u8) -> Result<Self, &'static str> {
        match hex {
            c @ b'0'..=b'9' => Ok(Nibble(c - b'0')),
            c @ b'A'..=b'F' => Ok(Nibble(c - b'A' + 10)),
            _ => Err("Can't parse non-hexadecimal character"),
        }
    }

    fn into_bits(self) -> NibbleBits {
        NibbleBits::new(self)
    }
}

struct NibbleBits {
    nibble: Nibble,
    len: u8,
}

impl NibbleBits {
    fn new(nibble: Nibble) -> Self {
        Self { len: 4, nibble }
    }

    fn shift(&mut self) {
        self.nibble.0 <<= 1;
        self.len -= 1;
    }
}

impl Iterator for NibbleBits {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        const MASK: u8 = 0b00010000;
        if self.len > 0 {
            self.shift();
            Some((self.nibble.0 & MASK) != 0)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len as usize, Some(self.len as usize))
    }
}

impl ExactSizeIterator for NibbleBits {}

impl FusedIterator for NibbleBits {}

// next_n stuff
trait NextN: Iterator {
    #[inline]
    fn next_n<const N: usize>(&mut self) -> Option<[Self::Item; N]> {
        #[allow(clippy::uninit_assumed_init)]
        let mut arr: [Self::Item; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for pos in arr.iter_mut() {
            if let Some(v) = self.next() {
                *pos = v;
            } else {
                return None;
            }
        }
        Some(arr)
    }
}

impl<I: Iterator> NextN for I {}

// fence stuff
struct Fence<'a, I> {
    inner: &'a mut I,
    limit: usize,
}

impl<'a, I> Fence<'a, I> {
    fn new(inner: &'a mut I, limit: usize) -> Fence<I> {
        Fence { inner, limit }
    }

    fn remaining(&self) -> usize {
        self.limit
    }
}

impl<'a, I> Iterator for Fence<'a, I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if self.limit != 0 {
            self.limit -= 1;
            self.inner.next()
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.inner.size_hint();
        (
            lower.min(self.limit),
            Some(upper.unwrap_or(self.limit).min(self.limit)),
        )
    }
}

trait Fencable {
    #[inline]
    fn fence(&mut self, limit: usize) -> Fence<Self>
    where
        Self: Sized,
    {
        Fence::new(self, limit)
    }
}

impl<I: Iterator> Fencable for I {}
