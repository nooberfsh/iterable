use std::iter::FromIterator;

use crate::WithFilter;

pub trait Iterable: IntoIterator {
    type C;
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

    fn filter(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: FromIterator<Self::Item>,
    {
        self.into_iter().filter(f).collect()
    }

    fn map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: FromIterator<U>,
    {
        self.into_iter().map(f).collect()
    }

    fn with_filter<F: Fn(&Self::Item) -> bool>(self, f: F) -> WithFilter<Self, F>
    where
        Self: Sized,
    {
        WithFilter { iterable: self, f }
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

