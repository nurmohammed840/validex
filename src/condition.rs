use crate::*;

pub struct Any<V>(pub V);

pub struct Maybe<T>(pub T);

pub struct Not<V>(pub V);

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
        impl<'a, T, $($ty: Check<&'a T>),*> Check<&'a T> for ($($ty,)*)
        where $($ty::Error: Into<DynError<'a>>,)* {
            type Error = DynError<'a>;
            fn check(&self, val: &'a T) -> Result<(), Self::Error> {
                $(self.$idx.check(val).map_err($ty::Error::into)?;)*
                Ok(())
            }
        }

        impl<'a, T, $($ty: Verify<&'a T>),*> Verify<&'a T> for Any<($($ty,)*)>
        where $( $ty::Error: Into<DynError<'a>>, )* {
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

        impl<'a, T, $($ty: Verify<&'a T>),*> Check<&'a T> for Any<($($ty,)*)>
        where $( $ty::Error: Into<DynError<'a>>,)* {
            type Error = errors::Errors<'a>;
            #[allow(non_snake_case)]
            fn check(&self, val: &'a T) -> Result<(), Self::Error> {
                check(self, val)
            }
        }
    ]
}

t! { V0:0 }
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
