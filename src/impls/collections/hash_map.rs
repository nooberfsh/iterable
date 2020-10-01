use std::collections::HashMap;
use std::hash::Hash;

use crate::{Iterable, IterableMap, GrowableProducer};

impl<K, V> Iterable for HashMap<K, V> {
    type C = Self;
    type CC<U> = Vec<U>;

    fn add_one(mut self, a: Self::Item) -> Self::C
    where
        Self::C: GrowableProducer<Self::Item>
    {
        self.grow_one(a);
        self
    }
}

impl<'a, K: 'a, V: 'a> Iterable for &'a HashMap<K, V> {
    type C = HashMap<&'a K, &'a V>;
    type CC<U> = Vec<U>;
}

impl<K, V> IterableMap<K, V> for HashMap<K, V> {
    type CCMap<X, Y> = HashMap<X, Y>;
}

impl<'a, K: 'a, V: 'a> IterableMap<&'a K, &'a V> for &'a HashMap<K, V> {
    type CCMap<X, Y> = HashMap<X, Y>;
}

delegate_into_iterator!(HashMap<K, V>, impl <K, V>);
delegate_into_iterator!(&'a HashMap<K, V>, impl <'a, K: 'a, V: 'a>);

delegate_from_iterator!(HashMap<K, V>, (K, V), impl <K: Eq + Hash, V>);
delegate_extend!(HashMap<K, V>, (K, V), impl <K: Eq + Hash, V>);

#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_c() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = v.filter(|(i, _)| i > &1);
        assert_eq!(res, hashmap![2=>"b", 3 => "c"]);
    }

    #[test]
    fn test_cc() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let mut res = v.map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_c_r() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = (&v).filter(|(i, _)| i > &&1);
        assert_eq!(res, hashmap![&2 => &"b", &3 => &"c"]);
    }

    #[test]
    fn test_cc_r() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let mut res = (&v).map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // tests for IterableMap
    #[test]
    fn test_map_value() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = v.map_value(|v| v.to_string());
        assert_eq!(
            res,
            hashmap![1 => "a".to_string(), 2 => "b".to_string(), 3 => "c".to_string()]
        );
    }

    #[test]
    fn test_map_kv() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = v.map_kv(|(k, v)| (k + 1, v.to_string()));
        assert_eq!(
            res,
            hashmap![2 => "a".to_string(), 3 => "b".to_string(), 4 => "c".to_string()]
        );
    }

    #[test]
    fn test_map_value_r() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = (&v).map_value(|v| v.to_string());
        assert_eq!(
            res,
            hashmap![&1 => "a".to_string(), &2 => "b".to_string(), &3 => "c".to_string()]
        );
    }

    #[test]
    fn test_map_kv_r() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = (&v).map_kv(|(k, v)| (k.to_string(), v.to_string()));
        assert_eq!(
            res,
            hashmap!["1".to_string() => "a".to_string(), "2".to_string() => "b".to_string(), "3".to_string() => "c".to_string()]
        );
    }
}
