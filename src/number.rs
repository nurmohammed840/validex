use crate::*;
use std::{fmt, ops::RangeBounds};

pub struct Range<R>(pub R);

pub struct ExcludeRange<R>(pub R);

impl<R, T> Validate<T> for Range<R>
where
    R: RangeBounds<T> + fmt::Debug + Clone,
    T: PartialOrd<T> + fmt::Debug + Clone,
{
    type Error = errors::RangeError<T, R>;
    fn validate(&self, val: &T) -> Result<(), Self::Error> {
        if !self.0.contains(val) {
            return Err(errors::RangeError {
                value: val.clone(),
                range: self.0.clone(),
            });
        }
        Ok(())
    }
}

impl<R, T> Validate<T> for ExcludeRange<R>
where
    R: RangeBounds<T> + fmt::Debug + Clone,
    T: PartialOrd<T> + fmt::Debug + Clone,
{
    type Error = errors::ExcludeRangeError<T, R>;
    fn validate(&self, val: &T) -> Result<(), Self::Error> {
        if self.0.contains(val) {
            return Err(errors::ExcludeRangeError {
                value: val.clone(),
                range: self.0.clone(),
            });
        }
        Ok(())
    }
}
