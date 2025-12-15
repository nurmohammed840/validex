use crate::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::ops::RangeBounds;

pub struct Length<T>(pub T);

impl<R, T> Verify<T> for Length<R>
where
    R: RangeBounds<usize> + Clone,
    T: Count,
{
    type Error = errors::LengthError<R>;
    fn verify(&self, val: &T) -> bool {
        self.0.contains(&val.count())
    }

    fn error(&self, val: &T) -> Self::Error {
        errors::LengthError {
            len: val.count(),
            range: self.0.clone(),
        }
    }
}

impl<R, T> Check<T> for Length<R>
where
    R: RangeBounds<usize> + Clone,
    T: GetLen,
{
    type Error = errors::LengthError<R>;

    fn check(&self, val: &T) -> Result<(), Self::Error> {
        let Some(len) = val.get_len() else {
            return Ok(());
        };
        if !self.0.contains(&len) {
            return Err(errors::LengthError {
                len,
                range: self.0.clone(),
            });
        }
        Ok(())
    }
}

trait GetLen {
    fn get_len(&self) -> Option<usize>;
}

impl<T: GetLen> GetLen for Option<T> {
    #[inline]
    fn get_len(&self) -> Option<usize> {
        match self {
            Some(s) => GetLen::get_len(s),
            None => None,
        }
    }
}

impl<T: Count> GetLen for T {
    #[inline]
    fn get_len(&self) -> Option<usize> {
        Some(T::count(self))
    }
}

trait Count {
    fn count(&self) -> usize;
}

impl<T, const N: usize> Count for [T; N] {
    #[inline]
    fn count(&self) -> usize {
        N
    }
}

macro_rules! len {
    [$($ty:ty)*] => [$(
        impl Count for $ty {
            #[inline] fn count(&self) -> usize { self.len() }
        }
    )*];
    [@deref $($ty: ty)*] => [$(
        impl<T: Count> Count for $ty {
            #[inline] fn count(&self) -> usize { T::count(self) }
        }
    )*];
    [@collection $(<$($p:tt),*> => $ty:ty)*] => [$(
       impl<$($p),*> Count for $ty {
           #[inline] fn count(&self) -> usize { self.len() }
       }
    )*]
}

len! { String str }
len! {
    @deref
    &T
    Box<T>
    std::rc::Rc<T>
    std::sync::Arc<T>
}
len! {
    @collection
    <T> => [T]
    <T> => Vec<T>
    <T> => VecDeque<T>
    <K> => BTreeSet<K>
    <K> => HashSet<K>
    <K, V> => BTreeMap<K, V>
    <K, V> => HashMap<K, V>
}
