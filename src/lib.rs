mod condition;
mod len;
mod number;

pub mod errors;

pub use condition::*;
pub use len::*;
pub use number::*;
pub use validex_macros::Check;

pub type DynError<'err> = Box<dyn std::error::Error + Send + Sync + 'err>;

pub trait Verify<Args> {
    type Error;
    fn verify(&self, _: Args) -> bool;
    fn error(&self, _: Args) -> Self::Error;
}

pub trait Check<Args> {
    type Error;
    fn check(&self, _: Args) -> Result<(), Self::Error>;
}

impl<F, T, E> Check<T> for F
where
    F: Fn(T) -> Result<(), E>,
{
    type Error = E;
    #[inline]
    fn check(&self, args: T) -> Result<(), Self::Error> {
        self(args)
    }
}

pub(crate) fn check<'a, T, V>(this: &V, val: &'a T) -> Result<(), V::Error>
where
    T: ?Sized,
    V: Verify<&'a T>,
{
    if !this.verify(val) {
        return Err(this.error(val));
    }
    Ok(())
}

#[doc(hidden)]
pub fn __field<'e, V, T>(key: &'static str, this: &V, val: T) -> Result<(), errors::FieldError<'e>>
where
    V: Check<T>,
    V::Error: Into<DynError<'e>>,
{
    Check::check(this, val).map_err(|err| errors::FieldError::new(key, err))
}
