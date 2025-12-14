mod condition;
mod len;
mod number;

pub mod errors;

pub use condition::*;
pub use len::*;
pub use number::*;
pub use validex_macros::Validate;

#[cfg(feature = "anyhow")]
pub type DynError = anyhow::Error;
#[cfg(not(feature = "anyhow"))]
pub type DynError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T = (), E = DynError> = std::result::Result<T, E>;

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

#[doc(hidden)]
pub fn __field<V, Args>(key: &'static str, v: &V, args: &Args) -> Result<(), errors::FieldError>
where
    Args: ?Sized,
    V: Validate<Args> + ?Sized,
    V::Error: Into<DynError>,
{
    Validate::validate(v, args).map_err(|err| errors::FieldError {
        key,
        error: err.into(),
    })
}
