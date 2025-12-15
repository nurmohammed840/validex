use crate::*;
use std::ops::RangeBounds;

pub struct Range<R>(pub R);

impl<R, T> Verify<T> for Range<R>
where
    R: RangeBounds<T> + Clone,
    T: PartialOrd<T> + Clone,
{
    type Error = errors::RangeError<T, R>;

    fn verify(&self, val: &T) -> bool {
        self.0.contains(val)
    }

    fn error(&self, val: &T) -> Self::Error {
        errors::RangeError {
            value: val.clone(),
            range: self.0.clone(),
        }
    }
}

impl<R, T> Check<T> for Range<R>
where
    R: RangeBounds<T> + Clone,
    T: PartialOrd<T> + Clone,
{
    type Error = errors::RangeError<T, R>;
    fn check(&self, val: &T) -> Result<(), Self::Error> {
        Verify::check(self, val)
    }
}

impl<V, T: Clone> Verify<T> for V
where
    V: PartialEq<T> + Clone,
{
    type Error = errors::EquelError<T, V>;

    fn verify(&self, val: &T) -> bool {
        self.eq(val)
    }
    fn error(&self, val: &T) -> Self::Error {
        errors::EquelError(val.clone(), self.clone())
    }
}
