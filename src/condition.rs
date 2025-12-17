use crate::*;

/// A condition that requires all sub-conditions to be met.
///
/// ## Example
///
/// ```rust
/// # use validex::*;
/// let condition = All((Range(10..=20), Not(15)));
/// assert!(condition.verify(&12));
/// assert!(!condition.verify(&42));
/// ```
pub struct All<V>(pub V);

/// A condition that requires any sub-condition to be met.
///
/// ## Example
///
/// ```rust
/// # use validex::*;
/// let condition = Any((Range(15..=20), 33, 42));
/// assert!(!condition.verify(&12));
/// assert!(condition.verify(&17));
/// assert!(condition.verify(&42));
/// ```
pub struct Any<V>(pub V);

/// Inverts the result of a sub-condition.
///
/// ## Example
///
/// ```rust
/// # use validex::*;
/// let condition = Not(Range(10..=20));
/// assert!(condition.verify(&5));
/// assert!(!condition.verify(&15));
/// ```
///
/// ### De Morgan's laws
///
/// - when applied with [`Any`](crate::Any):
///
/// ```text
/// Not(Any((c1, c2, ...))) -> All((Not(c1), Not(c2), ...))
/// ```
///
/// when applied with [`All`](crate::All):
///
/// ```text
/// Not(All((c1, c2, ...))) -> Any((Not(c1), Not(c2), ...))
/// ```
pub struct Not<V>(pub V);

/// The `Maybe` combinator allows a validation field to be optional ([`Option`](std::option::Option))
///
/// ## Example
///
/// ```rust
/// # use validex::*;
/// #[derive(Check)]
/// struct Input {
///   #[check(Maybe((Range(10..=20))))]
///   value: Option<u32>,
/// }
/// ```
pub struct Maybe<T>(pub T);

impl<T, V> Verify<T> for Not<V>
where
    V: Verify<T>,
{
    type Error = errors::Unexpected<V::Error>;
    #[inline]
    fn verify(&self, val: T) -> bool {
        !self.0.verify(val)
    }
    #[inline]
    fn error(&self, val: T) -> Self::Error {
        errors::Unexpected(self.0.error(val))
    }
}

impl<'a, V, T> Check<&'a T> for Not<V>
where
    T: ?Sized,
    V: Verify<&'a T>,
{
    type Error = errors::Unexpected<V::Error>;
    fn check(&self, val: &'a T) -> Result<(), Self::Error> {
        check(self, val)
    }
}

impl<'a, T, V> Check<&'a Option<T>> for Maybe<V>
where
    V: Check<&'a T>,
{
    type Error = V::Error;
    #[inline]
    fn check(&self, val: &'a Option<T>) -> Result<(), Self::Error> {
        match val {
            None => Ok(()),
            Some(val) => self.0.check(val),
        }
    }
}

macro_rules! t {
    [$($ty:tt: $idx:tt)*] => [
        #[doc(hidden)]
        impl<'a, T, $($ty,)*> Check<&'a T> for ($($ty,)*)
        where
            $($ty: Check<&'a T>,)*
            $($ty::Error: Into<DynError<'a>>,)*
        {
            type Error = DynError<'a>;
            fn check(&self, val: &'a T) -> Result<(), Self::Error> {
                $(self.$idx.check(val).map_err($ty::Error::into)?;)*
                Ok(())
            }
        }

        #[doc(hidden)]
        impl<'a, T, $($ty),*> Verify<&'a T> for Any<($($ty,)*)>
        where
            $($ty: Verify<&'a T>,)*
            $($ty::Error: Into<DynError<'a>>,)*
        {
            type Error = errors::Errors<'a>;
            fn verify(&self, val: &'a T) -> bool {
                $( self.0.$idx.verify(val) )||*
            }
            fn error(&self, val: &'a T) -> Self::Error {
                errors::Errors(Box::new([
                    $(self.0.$idx.error(val).into(),)*
                ]))
            }
        }

        #[doc(hidden)]
        impl<'a, T, $($ty),*> Check<&'a T> for Any<($($ty,)*)>
        where
            $($ty: Verify<&'a T>,)*
            $($ty::Error: Into<DynError<'a>>,)*
        {
            type Error = errors::Errors<'a>;
            fn check(&self, val: &'a T) -> Result<(), Self::Error> {
                check(self, val)
            }
        }
    ]
}

macro_rules! all {
    [$last_ty:tt : $last_idx:tt => $($ty:tt : $idx:tt)*] => [
        #[doc(hidden)]
        impl<'a, T, $($ty,)* $last_ty> Verify<&'a T> for All<($($ty,)* $last_ty,)>
        where
            $($ty: Verify<&'a T>,)*
            $last_ty: Verify<&'a T>,
            $( $ty::Error: Into<DynError<'a>>, )*
            $last_ty::Error: Into<DynError<'a>>
        {
            type Error = DynError<'a>;
            fn verify(&self, val: &'a T) -> bool {
                $( self.0.$idx.verify(val) &&)* self.0.$last_idx.verify(val)
            }
            fn error(&self, val: &'a T) -> Self::Error {
                $( if !self.0.$idx.verify(val) { return self.0.$idx.error(val).into(); } )*
                self.0.$last_idx.error(val).into()
            }
        }
    ]
}

t! { V0:0 V1:1 }
t! { V0:0 V1:1 V2:2 }
t! { V0:0 V1:1 V2:2 V3:3 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 V14:14 }
t! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 V14:14 V15:15 }

all! { V1  :1 => V0:0 }
all! { V2  :2 => V0:0 V1:1 }
all! { V3  :3 => V0:0 V1:1 V2:2 }
all! { V4  :4 => V0:0 V1:1 V2:2 V3:3 }
all! { V5  :5 => V0:0 V1:1 V2:2 V3:3 V4:4 }
all! { V6  :6 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 }
all! { V7  :7 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 }
all! { V8  :8 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 }
all! { V9  :9 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 }
all! { V10:10 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 }
all! { V11:11 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 }
all! { V12:12 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 }
all! { V13:13 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 }
all! { V14:14 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 }
all! { V15:15 => V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 V14:14 }

// #[cfg_attr(docsrs, doc(fake_variadic))]
#[doc = "This trait is implemented for tuples up to 16 items long."]
impl<T, V0> Check<T> for (V0,)
where
    V0: Check<T>,
{
    type Error = V0::Error;
    #[inline]
    fn check(&self, val: T) -> Result<(), Self::Error> {
        self.0.check(val)
    }
}

// #[cfg_attr(docsrs, doc(fake_variadic))]
#[doc = "This trait is implemented for tuples up to 16 items long."]
impl<T, V0> Verify<T> for All<(V0,)>
where
    V0: Verify<T>,
{
    type Error = V0::Error;
    #[inline]
    fn verify(&self, val: T) -> bool {
        self.0.0.verify(val)
    }
    #[inline]
    fn error(&self, val: T) -> Self::Error {
        self.0.0.error(val)
    }
}

// #[cfg_attr(docsrs, doc(fake_variadic))]
#[doc = "This trait is implemented for tuples up to 16 items long."]
impl<T, V0> Verify<T> for Any<(V0,)>
where
    V0: Verify<T>,
{
    type Error = V0::Error;

    #[inline]
    fn verify(&self, val: T) -> bool {
        self.0.0.verify(val)
    }
    #[inline]
    fn error(&self, val: T) -> Self::Error {
        self.0.0.error(val)
    }
}

// #[cfg_attr(docsrs, doc(fake_variadic))]
#[doc = "This trait is implemented for tuples up to 16 items long."]
impl<'a, T, V0> Check<&'a T> for Any<(V0,)>
where
    T: ?Sized,
    V0: Verify<&'a T>,
{
    type Error = V0::Error;
    #[inline]
    fn check(&self, val: &'a T) -> Result<(), Self::Error> {
        check(self, val)
    }
}
