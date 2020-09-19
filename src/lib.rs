#![feature(generic_associated_types)]
#![allow(incomplete_features)]

#[macro_use]
mod delegate;
mod impls;

pub use impls::*;

pub trait Iterable: Consumer {
    type C;
    type CC<U>;
    type CR<'a> where Self: 'a;

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.into_iter().count()
    }

    fn filter_map<U>(self, f: impl Fn(Self::Item) -> Option<U>) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: Producer<U>,
    {
        Self::CC::<U>::from_iter(self.into_iter().filter_map(f))
    }

    fn filter(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().filter(f))
    }

    fn map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: Producer<U>,
    {
        Self::CC::<U>::from_iter(self.into_iter().map(f))
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
        Self::CCMap<K, U>: Producer<(K, U)>,
    {
        Self::CCMap::<K, U>::from_iter(self.into_iter().map(|(k, v)| (k, f(v))))
    }

    fn map_kv<X, Y>(self, f: impl Fn((K, V)) -> (X, Y)) -> Self::CCMap<X, Y>
    where
        Self: Sized,
        Self::CCMap<X, Y>: Producer<(X, Y)>,
    {
        Self::CCMap::<X, Y>::from_iter(self.into_iter().map(f))
    }
}

pub trait Consumer {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait Producer<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}
