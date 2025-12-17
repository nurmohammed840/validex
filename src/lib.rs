#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod condition;
mod len;
mod number;

/// Error types used by the validation checks.
pub mod errors;

pub use condition::*;
pub use len::*;
pub use number::*;
pub use validex_macros::Check;

/// A dynamic error type.
pub type DynError<'err> = Box<dyn std::error::Error + Send + Sync + 'err>;

/// A trait that powers conditional combinators [`Not`], [`All`], [`Any`]
pub trait Verify<Args> {
    /// The error type produced when verify fails.
    type Error;

    /// Verifies the given arguments.
    fn verify(&self, _: Args) -> bool;

    /// Produces an error for the given arguments.
    fn error(&self, _: Args) -> Self::Error;
}

/// Trait for performing a check on a field.
///
/// This is the heart of the `validex` library.
///
/// Any types that implement [`Check`] trait can be used to validate fields using
/// `#[check(...)]` attribute.
///
/// Every function `Fn(&T) -> Result<(), E>` automatically implements this trait.
///
/// ### Example
///
/// ```rust
/// # use validex::{Check, Length};
/// #[derive(Check)]
/// struct Input {
///     #[check(is_even)]
///     id: i32,
///     #[check(Length(3..=64), is_ascii)]
///     name: String,
/// }
///
/// fn is_even(&num: &i32) -> Result<(), String> {
///     if num % 2 != 0 {
///         return Err(format!("{} is not even", num));
///     }
///     Ok(())
/// }
///
/// fn is_ascii(string: &dyn AsRef<str>) -> Result<(), String> {
///     let name = string.as_ref();
///     if !name.chars().all(|c| c.is_ascii()) {
///         return Err(format!("{} contains non-ascii characters", name));
///     }
///     Ok(())
/// }
/// ```
pub trait Check<Args> {
    /// The type of error returned if the check fails.
    type Error;

    /// Performs the check on the given input.
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
