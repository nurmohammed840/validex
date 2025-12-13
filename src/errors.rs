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
        write!(f, "value {:?} is out of range {:?}", self.value, self.range)
    }
}
