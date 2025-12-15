mod condition;
mod len;
mod number;

pub mod errors;

pub use condition::*;
pub use len::*;
pub use number::*;
pub use validex_macros::Check;

#[cfg(feature = "anyhow")]
pub type DynError = anyhow::Error;
#[cfg(not(feature = "anyhow"))]
pub type DynError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T = (), E = DynError> = std::result::Result<T, E>;

pub trait Check<Args: ?Sized> {
    type Error;
    fn check(&self, _: &Args) -> Result<(), Self::Error>;
}

impl<F, T: ?Sized, E> Check<T> for F
where
    F: Fn(&T) -> Result<(), E>,
{
    type Error = E;
    fn check(&self, args: &T) -> Result<(), Self::Error> {
        self(args)
    }
}

#[doc(hidden)]
pub fn __field<V, Args>(key: &'static str, v: &V, args: &Args) -> Result<(), errors::FieldError>
where
    Args: ?Sized,
    V: Check<Args> + ?Sized,
    V::Error: Into<DynError>,
{
    Check::check(v, args).map_err(|err| errors::FieldError {
        key,
        error: err.into(),
    })
}
