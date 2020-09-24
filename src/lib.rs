#![feature(try_trait)]
#![feature(extend_one)]
#![feature(associated_type_defaults)]
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


use std::ops::Try;
use std::cmp::Ordering;
use std::iter::Sum;
use std::iter::Product;
use std::cmp::Ord;
use std::fmt::Display;

use itertools::Itertools;

pub trait Iterable: Consumer {
    type C;
    type CC<U>;
    type F = Self::C;
    type CF<U> = Self::CC<U>;
    type CR<'a> where Self: 'a;
    type FR<'a> where Self: 'a = Self::CR<'a>;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // std combinator

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

    fn chain(self, other: impl Consumer<Item = Self::Item>) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::from_iter(self.into_iter().chain(other.into_iter()))
    }

    fn zip<E>(self, other: impl Consumer<Item=E>) -> Self::CC<(Self::Item, E)>
    where
        Self: Sized,
        Self::CC<(Self::Item, E)>: Producer<(Self::Item, E)>,
    {
        Self::CC::<(Self::Item, E)>::from_iter(self.into_iter().zip(other.into_iter()))
    }

    fn map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CF<U>
    where
        Self: Sized,
        Self::CF<U>: Producer<U>,
    {
        Self::CF::<U>::from_iter(self.into_iter().map(f))
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

    fn enumerate(self) -> Self::CF<(usize, Self::Item)>
    where
        Self: Sized,
        Self::CF<(usize, Self::Item)>: Producer<(usize, Self::Item)>,
    {
        Self::CF::<(usize, Self::Item)>::from_iter(self.into_iter().enumerate())
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

    fn flat_map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CC<U::Item>
    where
        U: Consumer,
        Self: Sized,
        Self::CC<U::Item>: Producer<U::Item>,
    {
        Self::CC::<U::Item>::from_iter(self.into_iter().flat_map(|t| f(t).into_iter()))
    }

    fn flatten(self) -> Self::CC<<Self::Item as Consumer>::Item>
    where
        Self: Sized,
        Self::Item: Consumer,
        Self::CC<<Self::Item as Consumer>::Item>: Producer<<Self::Item as Consumer>::Item>,
    {
        Self::CC::<<Self::Item as Consumer>::Item>::from_iter(self.into_iter().map(|item| item.into_iter()).flatten())
    }

    fn by_ref(&self) -> &Self {
        self
    }

    fn partition(self, f: impl Fn(&Self::Item) -> bool) ->(Self::C, Self::C)
    where
        Self: Sized,
        Self::C: GrowableProducer<Self::Item>,
    {
        let mut l  = <Self::C as GrowableProducer<Self::Item>>::empty();
        let mut r  = <Self::C as GrowableProducer<Self::Item>>::empty();
        for e in self.into_iter() {
            if f(&e) {
                l.add_one(e);
            } else {
                r.add_one(e);
            }
        }
        (l, r)
    }

    fn try_fold<S, R>(self, init: S, f: impl Fn(S, Self::Item) -> R) -> R
    where
        Self: Sized,
        R: Try<Ok = S>,
    {
        self.into_iter().try_fold(init, f)
    }

    fn try_for_each<R>(self, f: impl Fn(Self::Item) -> R) -> R
    where
        Self: Sized,
        R: Try<Ok = ()>,
    {
        self.into_iter().try_for_each(f)
    }

    fn fold<S>(self, init: S, f: impl Fn(S, Self::Item) -> S) -> S
    where
        Self: Sized,
    {
        self.into_iter().fold(init, f)
    }

    fn all(self, f: impl Fn(Self::Item) -> bool) -> bool
    where
        Self: Sized,
    {
        self.into_iter().all(f)
    }

    fn any(self, f: impl Fn(Self::Item) -> bool) -> bool
    where
        Self: Sized,
    {
        self.into_iter().any(f)
    }

    fn find(self, f: impl Fn(&Self::Item) -> bool) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.into_iter().find(f)
    }

    fn find_map<B>(self, f: impl Fn(Self::Item) -> Option<B>) -> Option<B>
    where
        Self: Sized,
    {
        self.into_iter().find_map(f)
    }

    fn position(self, f: impl Fn(Self::Item) -> bool) -> Option<usize>
    where
        Self: Sized,
    {
        self.into_iter().position(f)
    }

    fn rposition(self, f: impl Fn(Self::Item) -> bool) -> Option<usize>
    where
        Self: Sized,
        Self::IntoIter: ExactSizeIterator + DoubleEndedIterator,
    {
        self.into_iter().rposition(f)
    }

    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.into_iter().max()
    }

    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.into_iter().min()
    }

    fn max_by_key<B>(self, f: impl Fn(&Self::Item) -> B) -> Option<Self::Item>
    where
        Self: Sized,
        B: Ord,
    {
        self.into_iter().max_by_key(f)
    }

    fn max_by(self, f: impl Fn(&Self::Item, &Self::Item) -> Ordering) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.into_iter().max_by(f)
    }

    fn min_by_key<B>(self, f: impl Fn(&Self::Item) -> B) -> Option<Self::Item>
    where
        Self: Sized,
        B: Ord,
    {
        self.into_iter().min_by_key(f)
    }

    fn min_by(self, f: impl Fn(&Self::Item, &Self::Item) -> Ordering) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.into_iter().min_by(f)
    }

    fn rev(self) -> Self::F
    where
        Self: Sized,
        Self::F: Producer<Self::Item>,
        Self::IntoIter: DoubleEndedIterator,
    {
        Self::F::from_iter(self.into_iter().rev())
    }

    fn unzip<A, B>(self) -> (Self::CF<A>, Self::CF<B>)
    where
        Self: Sized,
        Self: Consumer<Item=(A, B)>,
        Self::CF<A>: GrowableProducer<A>,
        Self::CF<B>: GrowableProducer<B>,
    {
        let mut l  = <Self::CF<A> as GrowableProducer<A>>::empty();
        let mut r  = <Self::CF<B> as GrowableProducer<B>>::empty();
        for (a, b) in self.into_iter() {
            l.add_one(a);
            r.add_one(b);
        }
        (l, r)
    }

    fn copied<'a, T>(self) -> Self::CF<T>
    where
        T: 'a + Copy,
        Self: Sized,
        Self: Consumer<Item = &'a T>,
        Self::CF<T>: Producer<T>,
    {
        Self::CF::<T>::from_iter(self.into_iter().copied())
    }

    fn cloned<'a, T>(self) -> Self::CF<T>
    where
        T: 'a + Clone,
        Self: Sized,
        Self: Consumer<Item = &'a T>,
        Self::CF<T>: Producer<T>,
    {
        Self::CF::<T>::from_iter(self.into_iter().cloned())
    }

    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: Sum<Self::Item>,
    {
        self.into_iter().sum()
    }

    fn product<S>(self) -> S
    where
        Self: Sized,
        S: Product<Self::Item>,
    {
        self.into_iter().product()
    }

    fn cmp<I>(self, other: I) -> Ordering
    where
        I: Consumer<Item = Self::Item>,
        Self: Sized,
        Self::Item: Ord,
    {
        self.into_iter().cmp(other.into_iter())
    }

    fn partial_cmp<I>(self, other: I) -> Option<Ordering>
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.into_iter().partial_cmp(other.into_iter())
    }

    fn eq<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialEq<<I as Consumer>::Item>,
    {
        self.into_iter().eq(other.into_iter())
    }

    fn ne<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialEq<<I as Consumer>::Item>,
    {
        self.into_iter().ne(other.into_iter())
    }

    fn lt<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.into_iter().lt(other.into_iter())
    }

    fn le<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.into_iter().le(other.into_iter())
    }

    fn gt<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.into_iter().gt(other.into_iter())
    }

    fn ge<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.into_iter().ge(other.into_iter())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // itertools combinator

    fn join(self, sep: &str) -> String
    where
        Self: Sized,
        Self::Item: Display,
    {
        self.into_iter().join(sep)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // custom lazy combinator

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

pub trait GrowableProducer<A>: Producer<A> {
    fn empty() -> Self;
    fn add_one(&mut self, a: A);
}
