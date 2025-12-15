use super::*;

pub struct Any<V>(pub V);

pub struct Maybe<T>(pub T);

pub struct Not<V>(pub V);

impl<T, V> Verify<T> for Not<V>
where
    V: Verify<T>,
{
    type Error = errors::Unexpected<V::Error>;
    #[inline]
    fn verify(&self, val: &T) -> bool {
        !self.0.verify(val)
    }

    fn error(&self, val: &T) -> Self::Error {
        errors::Unexpected(self.0.error(val))
    }
}

impl<V, T> Check<T> for Not<V>
where
    V: Verify<T>,
{
    type Error = errors::Unexpected<V::Error>;
    fn check(&self, val: &T) -> Result<(), Self::Error> {
        Verify::check(self, val)
    }
}

impl<T, V: Check<T>> Check<Option<T>> for Maybe<V> {
    type Error = V::Error;
    #[inline]
    fn check(&self, val: &Option<T>) -> Result<(), Self::Error> {
        match val {
            None => Ok(()),
            Some(val) => self.0.check(val),
        }
    }
}

macro_rules! t {
    [$($ty:tt: $idx:tt)*] => [
        impl<T, $($ty: Check<T>),*> Check<T> for ($($ty,)*)
        where $($ty::Error: Into<DynError>,)* {
            type Error = DynError;
            fn check(&self, val: &T) -> Result<(), Self::Error> {
                $(self.$idx.check(val).map_err($ty::Error::into)?;)*
                Ok(())
            }
        }

        impl<T, $($ty: Verify<T>),*> Verify<T> for Any<($($ty,)*)>
        where $( $ty::Error: Into<DynError>, )* {
            type Error = errors::Errors;
            fn verify(&self, val: &T) -> bool {
                $( self.0.$idx.verify(val) )||*
            }
            fn error(&self, val: &T) -> Self::Error {
                errors::Errors(Box::new([
                    $(self.0.$idx.error(val).into(),)*
                ]))
            }
        }

        impl<T, $($ty: Verify<T>),*> Check<T> for Any<($($ty,)*)>
        where $( $ty::Error: Into<DynError>, )* {
            type Error = errors::Errors;
            #[allow(non_snake_case)]
            fn check(&self, val: &T) -> Result<(), Self::Error> {
                Verify::check(self, val)
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
