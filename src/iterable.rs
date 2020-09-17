use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::WithFilter;

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

    fn filter(self, f: impl Fn(&Self::Item) -> bool) -> Self::C
    where
        Self: Sized,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// iterable implementations

impl<T> Iterable for Vec<T> {
    type CC<U> = Vec<U>;
}

impl<'a, T: 'a> Iterable for &[T] {
    type CC<U> = Vec<U>;
}

impl<T> Iterable for HashSet<T> {
    type CC<U> = HashSet<U>;
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
    &'a I: IntoIterator,
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

#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3];
        let expected = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        let res = (&v).map(|i| i.to_string());
        assert_eq!(res, expected);

        let res = v.map(|i| i.to_string());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_slice() {
        let v: &[i32] = &[1, 2, 3];
        let expected = vec![2, 3, 4];
        let res = v.map(|i| i + 1);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_hash_set() {
        let v = hashset![1, 2, 3];
        let expected = hashset!["1".to_string(), "2".to_string(), "3".to_string()];

        let res = (&v).map(|i| i.to_string());
        assert_eq!(res, expected);

        let res = v.map(|i| i.to_string());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_hash_map() {
        // test Iterable
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let expected = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        let mut res = (&v).map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, expected);

        let mut res = v.clone().map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, expected);

        // test IterableMap
        let expected =
            hashmap![&1 => "a".to_string(), &2 => "b".to_string(), &3 => "c".to_string()];
        let res = (&v).map_value(|v| v.to_string());
        assert_eq!(res, expected);

        let expected = hashmap![1 => "a".to_string(), 2 => "b".to_string(), 3 => "c".to_string()];
        let res = v.clone().map_value(|v| v.to_string());
        assert_eq!(res, expected);

        let expected = hashmap!["1".to_string() => "a".to_string(), "2".to_string() => "b".to_string(), "3".to_string() => "c".to_string()];
        let res = (&v).map_kv(|(k, v)| (k.to_string(), v.to_string()));
        assert_eq!(res, expected);

        let expected = hashmap![2 => "a".to_string(), 3 => "b".to_string(), 4 => "c".to_string()];
        let res = v.clone().map_kv(|(k, v)| (k + 1, v.to_string()));
        assert_eq!(res, expected);
    }
}
