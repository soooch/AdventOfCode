//! A few small and unrelated utility things.

use core::mem::MaybeUninit;

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
