use crate::DynError;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// Error when a value is out of [`Range`](crate::Range)
#[derive(Debug)]
pub struct RangeError<T, R> {
    /// The actual input that caused the error.
    pub value: T,
    /// The expected range.
    pub range: R,
}

impl<T: Debug, R: Debug> Error for RangeError<T, R> {}
impl<T: Debug, R: Debug> Display for RangeError<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected {:?} in {:?}", self.value, self.range)
    }
}

/// Error returned when values do not match.
///
/// # Example
///
/// ```rust
/// # use validex::*;
/// #[derive(Check)]
/// struct Input {
///   #[check(Not(42))]
///   value: i32,
/// }
///
/// # fn main() {
/// let input = Input { value: 42 };
/// let err = input.check().unwrap_err();
/// println!("{err}"); // Not: expected 42
/// # }
/// ```
#[derive(Debug)]
pub struct EquelError<T, B>(pub T, pub B);
impl<A: Debug, B: Debug> Error for EquelError<A, B> {}
impl<A: Debug, B: Debug> Display for EquelError<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "expected {:?}, found {:?}", self.1, self.0)
        } else {
            write!(f, "expected {:?}", self.1)
        }
    }
}

/// Error returned when [`Not`](crate::Not) check fails.
#[derive(Debug)]
pub struct Unexpected<E>(pub E);
impl<E: Display + Debug> Error for Unexpected<E> {}
impl<E: Display> Display for Unexpected<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Not: ")?;
        Display::fmt(&self.0, f)
    }
}

/// Error for input length outside [`Length`](crate::Length) range.
#[derive(Debug)]
pub struct LengthError<R> {
    /// The actual length of the input.
    pub len: usize,
    /// The expected range for the length.
    pub range: R,
}
impl<R: Debug> Error for LengthError<R> {}
impl<R: Debug> Display for LengthError<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected length {} in {:?}", self.len, self.range)
    }
}

/// Reports an error when a field check fails.
#[derive(Debug)]
pub struct FieldError<'err> {
    /// The name of the field that caused the error.
    pub key: &'static str,
    /// The underlying error.
    pub error: DynError<'err>,
}
impl<'err> FieldError<'err> {
    /// Create a new [`FieldError`] for a given field key and error.
    pub fn new(key: &'static str, error: impl Into<DynError<'err>>) -> FieldError<'err> {
        FieldError {
            key,
            error: error.into(),
        }
    }
}
impl<'err> Error for FieldError<'err> {}
impl<'err> Display for FieldError<'err> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.key, f)?;
        f.write_str(" -> ")?;
        Display::fmt(&self.error, f)
    }
}

/// A list of errors returned when multiple [`Any`](crate::Any) checks fail.
#[derive(Debug)]
pub struct Errors<'err>(pub Box<[DynError<'err>]>);
impl<'err> Error for Errors<'err> {}
impl<'err> Display for Errors<'err> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for err in &self.0 {
            Display::fmt(err, f)?;
            f.write_str("; ")?;
        }
        Ok(())
    }
}
