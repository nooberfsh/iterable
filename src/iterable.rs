use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub trait Iterable: IntoIterator {
    type C: FromIterator<Self::Item> = Self;
    type CC<U>;

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.into_iter().count()
    }

    fn filter_map<U>(self, f: impl Fn(Self::Item) -> Option<U>) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: FromIterator<U>,
    {
        self.into_iter().filter_map(f).collect()
    }

    // reduction
    fn filter(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
    {
        self.into_iter().filter(f).collect()
    }

    // transformation
    fn map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: FromIterator<U>,
    {
        self.into_iter().map(f).collect()
    }

}

pub trait IterableMap<K, V>: Iterable<Item = (K, V)> {
    type CCMap<X, Y>;

    fn map_value<U>(self, f: impl Fn(V) -> U) -> Self::CCMap<K, U>
    where
        Self: Sized,
        Self::CCMap<K, U>: FromIterator<(K, U)>,
    {
        self.into_iter().map(|(k, v)| (k, f(v))).collect()
    }

    fn map_kv<X, Y>(self, f: impl Fn((K, V)) -> (X, Y)) -> Self::CCMap<X, Y>
    where
        Self: Sized,
        Self::CCMap<X, Y>: FromIterator<(X, Y)>,
    {
        self.into_iter().map(f).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// iterable implementations

impl<T> Iterable for Vec<T> {
    type CC<U> = Vec<U>;
}

impl<'a, T: 'a> Iterable for &[T] {
    type CC<U> = Vec<U>;
}

impl<T> Iterable for HashSet<T> {
    type CC<U> = HashSet<T>;
}

impl<K, V> Iterable for HashMap<K, V> {
    type CC<U> = Vec<U>;
}

impl<K, V> IterableMap<K, V> for HashMap<K, V> {
    type CCMap<X, Y> = HashMap<X, Y>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// iterable implementations for reference

impl<'a, I> Iterable for &'a I
where
    I: Iterable,
    &'a I: IntoIterator<Item = &'a I::Item>,
{
    type CC<U> = I::CC<U>;
}

impl<'a, K, V, IM> IterableMap<&'a K, &'a V> for &'a IM
    where
        IM: IterableMap<K, V>,
        &'a IM: Iterable<Item = (&'a K, &'a V)>,
{
    type CCMap<X, Y> = IM::CCMap<X, Y>;
}

// pub struct IterableWrap<'a, I: Iterable + 'a> {
//     iterable: &'a I
// }
//
// impl<'a, I: Iterable + 'a> IntoIterator for IterableWrap<'a, I>
//     where &'a I: Iterable
// {
//     type Item = <&'a I as IntoIterator>::Item;
//     type IntoIter = <&'a I as IntoIterator>::IntoIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.iterable.into_iter()
//     }
// }
//
// impl<'a, I> Iterable for IterableWrap<'a, I>
//     where I: Iterable ,
//           &'a I : IntoIterator<Item = &'a I::Item>
// {
//     type Collection<U> = I::Collection<U>;
// }
