mod number;
mod string;

pub mod errors;
pub use number::*;
pub use string::*;

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
