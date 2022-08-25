//! A few small and unrelated utility things.

use core::iter::FusedIterator;
use core::mem::MaybeUninit;
use num_traits::PrimInt;

/// A trait providing the [next_n](NextN::next_n) method for Iterators
pub trait NextN: Iterator {
    /// Akin to the familiar [Iterator::next], but returns N elements
    ///
    /// N is a const generic parameter, so [next_n](NextN::next_n) is able to return an array
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

pub struct CountIter<I> {
    inner: I,
    count: usize,
}

impl<I> CountIter<I> {
    #[inline]
    pub fn new(inner: I) -> CountIter<I> {
        CountIter { inner, count: 0 }
    }

    #[inline]
    pub fn iter_count(&self) -> usize {
        self.count
    }
}

impl<I> Iterator for CountIter<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for CountIter<I> {}
impl<I: FusedIterator> FusedIterator for CountIter<I> {}

pub trait Countable {
    #[inline]
    fn counted(self) -> CountIter<Self>
    where
        Self: Sized,
    {
        CountIter::new(self)
    }
}

impl<I: Iterator> Countable for I {}

/// A trait providing the [from_bits](FromBits::from_bits) method for integers
pub trait FromBits: PrimInt + From<bool> {
    /// Contructs `Self` from an iterator over bits from MSB to LSB
    /// Also counts the number of bits consumed for type 0 packets
    #[inline]
    fn from_bits<I>(bits: I) -> Self
    where
        I: Iterator<Item = bool>,
    {
        bits.fold(Self::zero(), |acc, b| (acc << 1) | b.into())
    }
}

impl FromBits for u8 {}
impl FromBits for u16 {}
impl FromBits for usize {}

/// An [Iterator] which limits iteration of the [Iterator] it wraps.
/// See the [fence](Fencable::fence) method.
pub struct Fence<'a, I> {
    inner: &'a mut I,
    limit: usize,
}

impl<'a, I> Fence<'a, I> {
    #[inline]
    pub fn new(inner: &'a mut I, limit: usize) -> Fence<I> {
        Fence { inner, limit }
    }

    #[inline]
    pub fn remaining(&self) -> usize {
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

/// A trait providing the [fence](Fencable::fence) method for Iterators
pub trait Fencable {
    /// Returns an [Iterator] which restricts iteration of the wrapped [Iterator]
    /// Similar to [Iterator::take], but borrows its inner iterator instead of moving
    #[inline]
    fn fence(&mut self, limit: usize) -> Fence<Self>
    where
        Self: Sized,
    {
        Fence::new(self, limit)
    }
}

impl<I: Iterator> Fencable for I {}

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
