use crate::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::ops::RangeBounds;

pub struct Length<T>(pub T);

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

impl<T, const N: usize> GetLen for [T; N] {
    #[inline]
    fn get_len(&self) -> Option<usize> {
        Some(N)
    }
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

macro_rules! len {
    [$($ty:ty)*] => [$(
        impl GetLen for $ty {
            #[inline] fn get_len(&self) -> Option<usize> { Some(self.len()) }
        }
    )*];
    [@deref $($ty: ty)*] => [$(
        impl<T: GetLen> GetLen for $ty {
            #[inline] fn get_len(&self) -> Option<usize> { T::get_len(self) }
        }
    )*];
    [@collection $(<$($p:tt),*> => $ty:ty)*] => [$(
       impl<$($p),*> GetLen for $ty {
           #[inline] fn get_len(&self) -> Option<usize> { Some(self.len()) }
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
