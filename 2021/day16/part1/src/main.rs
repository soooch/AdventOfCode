use num_traits::PrimInt;
use std::iter::FusedIterator;
use std::mem::MaybeUninit;

fn main() {
    let input = include_str!("../../input.txt");

    let mut bits = input
        .trim()
        .bytes()
        .map(|b| Nibble::from_hex_ascii(b).unwrap())
        .flat_map(Nibble::into_bits);

    let packet = Packet::from_bits(&mut bits).unwrap();

    println!("{packet:?}");

    let version_sum = sum_of_versions(packet);

    println!("{version_sum}");
}

fn sum_of_versions(p: Packet) -> usize {
    let vec_sum = |sp: Vec<Packet>| sp.into_iter().map(sum_of_versions).sum();

    p.version as usize
        + match p.body {
            PacketBody::Literal(_) => 0,
            PacketBody::Sum(sp) => vec_sum(sp),
            PacketBody::Product(sp) => vec_sum(sp),
            PacketBody::Minimum(sp) => vec_sum(sp),
            PacketBody::Maximum(sp) => vec_sum(sp),
            PacketBody::Greater(sp) => vec_sum(sp),
            PacketBody::Less(sp) => vec_sum(sp),
            PacketBody::Equal(sp) => vec_sum(sp),
        }
}

// parse 15 bits into a num
// then parse that number of bits into packets
fn packets_from_bits_t0(
    bits: &mut impl Iterator<Item = bool>,
) -> Result<Vec<Packet>, &'static str> {
    let num_bits = bits
        .next_n::<15>()
        .ok_or("Expected type 0 length, but bit stream ended")?;
    let num_bits = u16::from_bits(num_bits);

    // type erasure. this line sucks
    let mut bits: Box<dyn Iterator<Item = bool>> = Box::new(bits);

    let mut subpacket_bits = bits.fence(num_bits as usize);

    let mut subpackets = Vec::new();
    while subpacket_bits.remaining() > 0 {
        subpackets.push(Packet::from_bits(&mut subpacket_bits)?);
    }

    Ok(subpackets)
}

// parse 11 bits into a num
// then parse that many packets
fn packets_from_bits_t1(
    bits: &mut impl Iterator<Item = bool>,
) -> Result<Vec<Packet>, &'static str> {
    let num_packets = bits
        .next_n::<11>()
        .ok_or("Expected type 1 length, but bit stream ended")?;
    let num_packets = u16::from_bits(num_packets);

    let mut subpackets = Vec::with_capacity(num_packets as usize);
    for _ in 0..num_packets {
        subpackets.push(Packet::from_bits(bits)?)
    }

    Ok(subpackets)
}

#[derive(Default, Debug)]
struct Packet {
    version: u8,
    body: PacketBody,
}

impl Packet {
    fn from_bits(bits: &mut impl Iterator<Item = bool>) -> Result<Self, &'static str> {
        let version = bits
            .next_n::<3>()
            .ok_or("Expected packet version, but bit stream ended")?;
        let version = u8::from_bits(version);

        let body = PacketBody::from_bits(bits)?;

        Ok(Packet { version, body })
    }
}

#[derive(Debug)]
enum PacketBody {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Greater(Vec<Packet>),
    Less(Vec<Packet>),
    Equal(Vec<Packet>),
}

impl PacketBody {
    fn from_bits(bits: &mut impl Iterator<Item = bool>) -> Result<Self, &'static str> {
        let type_id = bits
            .next_n::<3>()
            .ok_or("Expected packet type ID, but bit stream ended")?;
        let type_id = u8::from_bits(type_id);

        match type_id {
            4 => Self::literal_from_bits(bits),
            op => {
                let length_type = bits
                    .next()
                    .ok_or("Expected length type ID, but bit stream ended")?;

                let mut get_vec_packets = || match length_type {
                    false => packets_from_bits_t0(bits),
                    true => packets_from_bits_t1(bits),
                };

                let body = match op {
                    0 => Self::Sum(get_vec_packets()?),
                    1 => Self::Product(get_vec_packets()?),
                    2 => Self::Minimum(get_vec_packets()?),
                    3 => Self::Maximum(get_vec_packets()?),
                    5 => Self::Greater(get_vec_packets()?),
                    6 => Self::Less(get_vec_packets()?),
                    7 => Self::Equal(get_vec_packets()?),
                    _ => Err("Unrecognized Operation")?,
                };

                Ok(body)
            }
        }
    }
    fn literal_from_bits(bits: &mut impl Iterator<Item = bool>) -> Result<Self, &'static str> {
        let append_bits = |acc, frag| (acc << 4) | u8::from_bits(frag) as usize;
        let mut lit_acc = 0;
        loop {
            match bits.next_n::<5>() {
                Some([true, frag @ ..]) => lit_acc = append_bits(lit_acc, frag),
                Some([false, frag @ ..]) => {
                    let lit_acc = append_bits(lit_acc, frag);
                    break Ok(Self::Literal(lit_acc));
                }
                None => break Err("Expected literal value, but bit stream ended"),
            }
        }
    }
}

impl Default for PacketBody {
    fn default() -> Self {
        PacketBody::Literal(0)
    }
}

// packet header
// first 3 bits : version
// next 3 bits  : type ID

// literal packet
// ???_100_1????_1????_0????_000

// operator packet
// ???_???_0_{15?->n}_{n?}
// ???_???_1_{11?->n}_{n[]}

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
