#![feature(iter_map_while)]
#![feature(maybe_uninit_uninit_array)]
#![feature(array_value_iter)]
#![feature(min_const_generics)]
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

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.into_iter().last()
    }

    fn nth(self, n: usize) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.into_iter().nth(n)
    }

    fn step_by(self, step: usize) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().step_by(step))
    }

    fn chain(self, other: impl Iterable<Item = Self::Item>) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().chain(other.into_iter()))
    }

    fn zip<E>(self, other: impl Iterable<Item=E>) -> Self::CC<(Self::Item, E)>
    where
        Self: Sized,
        Self::CC<(Self::Item, E)>: Producer<(Self::Item, E)>,
    {
        Self::CC::<(Self::Item, E)>::from_iter(self.into_iter().zip(other.into_iter()))
    }

    fn map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: Producer<U>,
    {
        Self::CC::<U>::from_iter(self.into_iter().map(f))
    }

    fn foreach(self, f: impl Fn(Self::Item))
    where
        Self: Sized,
    {
        self.into_iter().for_each(f)
    }

    fn filter(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().filter(f))
    }

    fn filter_map<U>(self, f: impl Fn(Self::Item) -> Option<U>) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: Producer<U>,
    {
        Self::CC::<U>::from_iter(self.into_iter().filter_map(f))
    }

    fn enumerate(self) -> Self::CC<(usize, Self::Item)>
    where
        Self: Sized,
        Self::CC<(usize, Self::Item)>: Producer<(usize, Self::Item)>,
    {
        Self::CC::<(usize, Self::Item)>::from_iter(self.into_iter().enumerate())
    }

    fn skip_while(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().skip_while(f))
    }

    fn take_while(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().take_while(f))
    }

    fn map_while<U>(self, f: impl Fn(Self::Item) -> Option<U>) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: Producer<U>,
    {
        Self::CC::<U>::from_iter(self.into_iter().map_while(f))
    }

    fn skip(self, n: usize) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().skip(n))
    }

    fn take(self, n: usize) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().take(n))
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
    fn from_iter<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = A>;
}
