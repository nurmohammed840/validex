mod len;
mod number;

pub mod errors;

pub use len::*;
pub use number::*;
pub use validex_macros::Validate;

pub type DynError<'e> = Box<dyn std::error::Error + Send + Sync + 'e>;
pub type Result<T = (), E = DynError<'static>> = std::result::Result<T, E>;

pub trait Validate<Args: ?Sized> {
    type Error;
    fn validate(&self, _: &Args) -> Result<(), Self::Error>;
}

impl<F, T: ?Sized, E> Validate<T> for F
where
    F: Fn(&T) -> Result<(), E>,
{
    type Error = E;
    fn validate(&self, args: &T) -> Result<(), Self::Error> {
        self(args)
    }
}

pub struct Maybe<T>(pub T);

impl<T, V: Validate<T>> Validate<Option<T>> for Maybe<V> {
    type Error = V::Error;
    #[inline]
    fn validate(&self, val: &Option<T>) -> Result<(), Self::Error> {
        match val {
            None => Ok(()),
            Some(val) => self.0.validate(val),
        }
    }
}
