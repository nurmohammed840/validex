use crate::*;
use std::ops::RangeBounds;

pub struct Range<R>(pub R);

impl<'a, R, T> Verify<&'a T> for Range<R>
where
    T: PartialOrd<T>,
    R: RangeBounds<T> + Clone,
{
    type Error = errors::RangeError<&'a T, R>;
    #[inline]
    fn verify(&self, val: &T) -> bool {
        self.0.contains(val)
    }
    #[inline]
    fn error(&self, value: &'a T) -> Self::Error {
        errors::RangeError {
            value,
            range: self.0.clone(),
        }
    }
}

impl<'a, R, T> Check<&'a T> for Range<R>
where
    T: PartialOrd<T>,
    R: RangeBounds<T> + Clone,
{
    type Error = errors::RangeError<&'a T, R>;
    fn check(&self, val: &'a T) -> Result<(), Self::Error> {
        check(self, val)
    }
}

impl<'a, V, T> Verify<&'a T> for V
where
    V: PartialEq<T> + Clone,
{
    type Error = errors::EquelError<&'a T, V>;
    #[inline]
    fn verify(&self, val: &T) -> bool {
        self.eq(val)
    }
    #[inline]
    fn error(&self, val: &'a T) -> Self::Error {
        errors::EquelError(val, self.clone())
    }
}
