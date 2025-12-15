use crate::DynError;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct RangeError<T, R> {
    pub value: T,
    pub range: R,
}

impl<T: Debug, R: Debug> Error for RangeError<T, R> {}
impl<T: Debug, R: Debug> Display for RangeError<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected value {:?} in {:?}", self.value, self.range)
    }
}

#[derive(Debug)]
pub struct EquelError<A, B>(pub A, pub B);
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

#[derive(Debug)]
pub struct Unexpected<E>(pub E);
impl<E: Display + Debug> Error for Unexpected<E> {}
impl<E: Display> Display for Unexpected<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Not: ")?;
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug)]
pub struct LengthError<R> {
    pub len: usize,
    pub range: R,
}
impl<R: Debug> Error for LengthError<R> {}
impl<R: Debug> Display for LengthError<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected length {} in {:?}", self.len, self.range)
    }
}

#[derive(Debug)]
pub struct FieldError {
    pub key: &'static str,
    pub error: DynError,
}
impl FieldError {
    pub fn new(key: &'static str, err: impl Into<DynError>) -> FieldError {
        FieldError {
            key,
            error: err.into(),
        }
    }
}
impl Error for FieldError {}
impl Display for FieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.key, self.error)
    }
}

#[derive(Debug)]
pub struct Errors(pub Box<[DynError]>);
impl Error for Errors {}
impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for err in &self.0 {
            Display::fmt(err, f)?;
            f.write_str("; ")?;
        }
        Ok(())
    }
}
