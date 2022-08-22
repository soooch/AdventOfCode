//! For when a byte is a bit (or four) too much.

use core::iter::FusedIterator;

/// A Nibble is used to represent a 4 bit value. It is half of a byte.
pub struct Nibble(u8);

impl Nibble {
    /// Returns a nibble from a [u8].
    ///
    /// # Arguments
    /// * `hex` - A u8 value interpreted as an ASCII character
    #[inline]
    pub const fn from_hex_ascii(hex: u8) -> Result<Self, &'static str> {
        match hex {
            c @ b'0'..=b'9' => Ok(Nibble(c - b'0')),
            c @ b'A'..=b'F' => Ok(Nibble(c - b'A' + 10)),
            _ => Err("Can't parse non-hexadecimal character"),
        }
    }

    /// Returns an [Iterator] which iterates over each bit.
    /// Iterates from the most significant bit to the least significant bit
    #[inline]
    pub fn into_bits(self) -> NibbleBits {
        NibbleBits::new(self)
    }
}

/// A struct used to iterate over the bits of a [Nibble].
/// See [Nibble::into_bits]
pub struct NibbleBits {
    nibble: Nibble,
    len: u8,
}

impl NibbleBits {
    #[inline]
    fn new(nibble: Nibble) -> Self {
        Self { len: 4, nibble }
    }

    #[inline]
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
