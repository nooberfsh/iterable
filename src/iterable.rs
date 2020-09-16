use std::iter::FromIterator;
use std::collections::HashSet;

pub trait Iterable: IntoIterator {
    type Collection<U>;

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.into_iter().count()
    }

    fn filter_map<U>(self, f: impl Fn(Self::Item) -> Option<U>) -> Self::Collection<U>
    where
        Self: Sized,
        Self::Collection<U>: FromIterator<U>
    {
        self.into_iter().filter_map(f).collect()
    }

    // reduction
    fn filter(self, f: impl Fn(&Self::Item) -> bool) -> Self::Collection<Self::Item>
    where
        Self: Sized,
        Self::Collection<Self::Item>: FromIterator<Self::Item>
    {
        self.into_iter().filter(f).collect()
    }

    // transformation
    fn map<U>(self, f: impl Fn(Self::Item) -> U) -> Self::Collection<U>
    where
        Self: Sized,
        Self::Collection<U>: FromIterator<U>
    {
        self.into_iter().map(f).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// iterable implementations

impl<T> Iterable for Vec<T> {
    type Collection<U> = Vec<U>;
}

impl<'a, T: 'a> Iterable for &[T] {
    type Collection<U> = Vec<U>;
}

impl<T> Iterable for HashSet<T> {
    type Collection<U> = HashSet<T>;
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// iterable implementations for reference

impl<'a, I> Iterable for &'a I
where
    I: Iterable,
    &'a I: IntoIterator<Item = &'a I::Item>,
{
    type Collection<U> = I::Collection<U>;
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
