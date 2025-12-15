use super::*;

pub struct All<V>(pub V);

pub struct Any<V>(pub V);

pub struct Maybe<T>(pub T);

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

macro_rules! all {
    [$($ty:tt: $idx:tt)*] => [
        impl<T, $($ty),*> Check<T> for All<($($ty,)*)>
        where $(
            $ty: Check<T>,
            $ty::Error: Into<DynError>,
        )* {
            type Error = DynError;
            fn check(&self, val: &T) -> Result<(), Self::Error> {
                $(self.0.$idx.check(val).map_err($ty::Error::into)?;)*
                Ok(())
            }
        }

        impl<T, $($ty),*> Check<T> for Any<($($ty,)*)>
        where $(
            $ty: Check<T>,
            $ty::Error: Into<DynError>,
        )* {
            type Error = errors::Errors;
            #[allow(non_snake_case)]
            fn check(&self, val: &T) -> Result<(), Self::Error> {
                $(let Err($ty) = self.0.$idx.check(val) else { return Ok(()) };)*
                Err(errors::Errors(Box::new([
                    $($ty.into(),)*
                ])))
            }
        }
    ]
}

all! { V0:0 }
all! { V0:0 V1:1 }
all! { V0:0 V1:1 V2:2 }
all! { V0:0 V1:1 V2:2 V3:3 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 V14:14 }
all! { V0:0 V1:1 V2:2 V3:3 V4:4 V5:5 V6:6 V7:7 V8:8 V9:9 V10:10 V11:11 V12:12 V13:13 V14:14 V15:15 }
