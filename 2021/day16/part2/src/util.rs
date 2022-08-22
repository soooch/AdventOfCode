//! A few small and unrelated utility things.

use num_traits::PrimInt;
use std::mem::MaybeUninit;

/// A trait providing the [from_bits](FromBits::from_bits) method for integers
pub trait FromBits: PrimInt + From<bool> {
    /// Contructs `Self` from an iterator over bits from MSB to LSB
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

/// An [Iterator] which limits iteration of the [Iterator] it wraps.
/// See the [fence](Fencable::fence) method.
pub struct Fence<'a, I> {
    inner: &'a mut I,
    limit: usize,
}

impl<'a, I> Fence<'a, I> {
    pub fn new(inner: &'a mut I, limit: usize) -> Fence<I> {
        Fence { inner, limit }
    }

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
