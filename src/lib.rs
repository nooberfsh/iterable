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
mod lazy;

pub use impls::*;
pub use lazy::*;

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

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // from std

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.consume().count()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.consume().last()
    }

    fn nth(self, n: usize) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.consume().nth(n)
    }

    fn step_by(self, step: usize) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::produce(self.consume().step_by(step))
    }

    fn chain(self, other: impl Consumer<Item = Self::Item>) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::produce(self.consume().chain(other.consume()))
    }

    fn zip<E>(self, other: impl Consumer<Item=E>) -> Self::CC<(Self::Item, E)>
    where
        Self: Sized,
        Self::CC<(Self::Item, E)>: Producer<(Self::Item, E)>,
    {
        Self::CC::<(Self::Item, E)>::produce(self.consume().zip(other.consume()))
    }

    fn map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CF<U>
    where
        Self: Sized,
        Self::CF<U>: Producer<U>,
    {
        Self::CF::<U>::produce(self.consume().map(f))
    }

    fn foreach(self, f: impl Fn(Self::Item))
    where
        Self: Sized,
    {
        self.consume().for_each(f)
    }

    fn filter(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::produce(self.consume().filter(f))
    }

    fn filter_map<U>(self, f: impl Fn(Self::Item) -> Option<U>) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: Producer<U>,
    {
        Self::CC::<U>::produce(self.consume().filter_map(f))
    }

    fn enumerate(self) -> Self::CF<(usize, Self::Item)>
    where
        Self: Sized,
        Self::CF<(usize, Self::Item)>: Producer<(usize, Self::Item)>,
    {
        Self::CF::<(usize, Self::Item)>::produce(self.consume().enumerate())
    }

    fn skip_while(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::produce(self.consume().skip_while(f))
    }

    fn take_while(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::produce(self.consume().take_while(f))
    }

    fn map_while<U>(self, f: impl Fn(Self::Item) -> Option<U>) -> Self::CC<U>
    where
        Self: Sized,
        Self::CC<U>: Producer<U>,
    {
        Self::CC::<U>::produce(self.consume().map_while(f))
    }

    fn skip(self, n: usize) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::produce(self.consume().skip(n))
    }

    fn take(self, n: usize) -> Self::C
    where
        Self: Sized,
        Self::C: Producer<Self::Item>,
    {
        Self::C::produce(self.consume().take(n))
    }

    fn flat_map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::CC<U::Item>
    where
        U: Consumer,
        Self: Sized,
        Self::CC<U::Item>: Producer<U::Item>,
    {
        Self::CC::<U::Item>::produce(self.consume().flat_map(|t| f(t).consume()))
    }

    fn flatten(self) -> Self::CC<<Self::Item as Consumer>::Item>
    where
        Self: Sized,
        Self::Item: Consumer,
        Self::CC<<Self::Item as Consumer>::Item>: Producer<<Self::Item as Consumer>::Item>,
    {
        Self::CC::<<Self::Item as Consumer>::Item>::produce(self.consume().map(|item| item.consume()).flatten())
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
        for e in self.consume() {
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
        self.consume().try_fold(init, f)
    }

    fn try_for_each<R>(self, f: impl Fn(Self::Item) -> R) -> R
    where
        Self: Sized,
        R: Try<Ok = ()>,
    {
        self.consume().try_for_each(f)
    }

    fn fold<S>(self, init: S, f: impl Fn(S, Self::Item) -> S) -> S
    where
        Self: Sized,
    {
        self.consume().fold(init, f)
    }

    fn all(self, f: impl Fn(Self::Item) -> bool) -> bool
    where
        Self: Sized,
    {
        self.consume().all(f)
    }

    fn any(self, f: impl Fn(Self::Item) -> bool) -> bool
    where
        Self: Sized,
    {
        self.consume().any(f)
    }

    fn find(self, f: impl Fn(&Self::Item) -> bool) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.consume().find(f)
    }

    fn find_map<B>(self, f: impl Fn(Self::Item) -> Option<B>) -> Option<B>
    where
        Self: Sized,
    {
        self.consume().find_map(f)
    }

    fn position(self, f: impl Fn(Self::Item) -> bool) -> Option<usize>
    where
        Self: Sized,
    {
        self.consume().position(f)
    }

    fn rposition(self, f: impl Fn(Self::Item) -> bool) -> Option<usize>
    where
        Self: Sized,
        Self::IntoIter: ExactSizeIterator + DoubleEndedIterator,
    {
        self.consume().rposition(f)
    }

    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.consume().max()
    }

    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.consume().min()
    }

    fn max_by_key<B>(self, f: impl Fn(&Self::Item) -> B) -> Option<Self::Item>
    where
        Self: Sized,
        B: Ord,
    {
        self.consume().max_by_key(f)
    }

    fn max_by(self, f: impl Fn(&Self::Item, &Self::Item) -> Ordering) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.consume().max_by(f)
    }

    fn min_by_key<B>(self, f: impl Fn(&Self::Item) -> B) -> Option<Self::Item>
    where
        Self: Sized,
        B: Ord,
    {
        self.consume().min_by_key(f)
    }

    fn min_by(self, f: impl Fn(&Self::Item, &Self::Item) -> Ordering) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.consume().min_by(f)
    }

    fn rev(self) -> Self::F
    where
        Self: Sized,
        Self::F: Producer<Self::Item>,
        Self::IntoIter: DoubleEndedIterator,
    {
        Self::F::produce(self.consume().rev())
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
        for (a, b) in self.consume() {
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
        Self::CF::<T>::produce(self.consume().copied())
    }

    fn cloned<'a, T>(self) -> Self::CF<T>
    where
        T: 'a + Clone,
        Self: Sized,
        Self: Consumer<Item = &'a T>,
        Self::CF<T>: Producer<T>,
    {
        Self::CF::<T>::produce(self.consume().cloned())
    }

    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: Sum<Self::Item>,
    {
        self.consume().sum()
    }

    fn product<S>(self) -> S
    where
        Self: Sized,
        S: Product<Self::Item>,
    {
        self.consume().product()
    }

    fn cmp<I>(self, other: I) -> Ordering
    where
        I: Consumer<Item = Self::Item>,
        Self: Sized,
        Self::Item: Ord,
    {
        self.consume().cmp(other.consume())
    }

    fn partial_cmp<I>(self, other: I) -> Option<Ordering>
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.consume().partial_cmp(other.consume())
    }

    fn eq<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialEq<<I as Consumer>::Item>,
    {
        self.consume().eq(other.consume())
    }

    fn ne<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialEq<<I as Consumer>::Item>,
    {
        self.consume().ne(other.consume())
    }

    fn lt<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.consume().lt(other.consume())
    }

    fn le<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.consume().le(other.consume())
    }

    fn gt<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.consume().gt(other.consume())
    }

    fn ge<I>(self, other: I) -> bool
    where
        I: Consumer,
        Self: Sized,
        Self::Item: PartialOrd<<I as Consumer>::Item>,
    {
        self.consume().ge(other.consume())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // from itertools

    fn join(self, sep: &str) -> String
    where
        Self: Sized,
        Self::Item: Display,
    {
        self.consume().join(sep)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // lazy combinator

    fn lazy_step_by(self, step: usize) -> LazyStepBy<Self>
    where
        Self: Sized,
    {
        LazyStepBy { iterable: self, step }
    }

    fn lazy_zip<C: Consumer>(self, c: C) -> LazyZip<Self, C>
    where
        Self: Sized,
    {
        LazyZip { iterable: self, c }
    }

    fn lazy_filter<F: Fn(&Self::Item) -> bool>(self, f: F) -> LazyFilter<Self, F>
    where
        Self: Sized,
    {
        LazyFilter { iterable: self, f }
    }

    fn lazy_map<T, F: Fn(Self::Item) -> T>(self, f: F) -> LazyMap<Self, F>
    where
        Self: Sized,
    {
        LazyMap { iterable: self, f }
    }

    fn lazy_filter_map<T, F: Fn(Self::Item) -> Option<T>>(self, f: F) -> LazyFilterMap<Self, F>
    where
        Self: Sized,
    {
        LazyFilterMap { iterable: self, f }
    }

    fn lazy_enumerate(self) -> LazyEnumerate<Self>
    where
        Self: Sized,
    {
        LazyEnumerate { iterable: self }
    }

    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::IntoIter: Clone,
    {
        LazyCycle { iterable: self }
    }
}

pub trait IterableMap<K, V>: Iterable<Item = (K, V)> {
    type CCMap<X, Y>;

    fn map_value<U>(self, f: impl Fn(V) -> U) -> Self::CCMap<K, U>
    where
        Self: Sized,
        Self::CCMap<K, U>: Producer<(K, U)>,
    {
        Self::CCMap::<K, U>::produce(self.consume().map(|(k, v)| (k, f(v))))
    }

    fn map_kv<X, Y>(self, f: impl Fn((K, V)) -> (X, Y)) -> Self::CCMap<X, Y>
    where
        Self: Sized,
        Self::CCMap<X, Y>: Producer<(X, Y)>,
    {
        Self::CCMap::<X, Y>::produce(self.consume().map(f))
    }
}

pub trait Consumer {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn consume(self) -> Self::IntoIter;
}

pub trait Producer<A> {
    fn produce<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = A>;
}

pub trait GrowableProducer<A>: Producer<A> {
    fn empty() -> Self;
    fn add_one(&mut self, a: A);
}

#[cfg(test)]
fn assert_type<T>(_t: T) {}