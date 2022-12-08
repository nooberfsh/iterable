use std::collections::BTreeMap;

use crate::{GrowableProducer, Iterable, IterableMap};

impl<K, V> Iterable for BTreeMap<K, V> {
    type C = Self;
    type CC<U> = Vec<U>;
    // remove below after `associated_type_defaults` stabilized
    type F = Self;
    type CF<U> = Vec<U>;

    fn add_one(mut self, a: Self::Item) -> Self::C
    where
        Self::C: GrowableProducer<Self::Item>,
    {
        self.grow_one(a);
        self
    }
}

impl<'a, K: 'a, V: 'a> Iterable for &'a BTreeMap<K, V> {
    type C = BTreeMap<&'a K, &'a V>;
    type CC<U> = Vec<U>;
    // remove below after `associated_type_defaults` stabilized
    type F = BTreeMap<&'a K, &'a V>;
    type CF<U> = Vec<U>;
}

impl<K, V> IterableMap<K, V> for BTreeMap<K, V> {
    type CCMap<X, Y> = BTreeMap<X, Y>;
}

impl<'a, K: 'a, V: 'a> IterableMap<&'a K, &'a V> for &'a BTreeMap<K, V> {
    type CCMap<X, Y> = BTreeMap<X, Y>;
}

delegate_into_iterator!(BTreeMap<K, V>, impl <K, V>);
delegate_into_iterator!(&'a BTreeMap<K, V>, impl <'a, K: 'a, V: 'a>);

delegate_from_iterator!(BTreeMap<K, V>, (K, V), impl <K: Ord, V>);
delegate_extend!(BTreeMap<K, V>, (K, V), impl <K: Ord, V>);

#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_c() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let res = v.filter(|(i, _)| i > &1);
        assert_eq!(res, btreemap![2=>"b", 3 => "c"]);
    }

    #[test]
    fn test_cc() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let mut res = v.map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_c_r() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let res = (&v).filter(|(i, _)| i > &&1);
        assert_eq!(res, btreemap![&2 => &"b", &3 => &"c"]);
    }

    #[test]
    fn test_cc_r() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let mut res = (&v).map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // tests for IterableMap
    #[test]
    fn test_map_value() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let res = v.map_value(|v| v.to_string());
        assert_eq!(
            res,
            btreemap![1 => "a".to_string(), 2 => "b".to_string(), 3 => "c".to_string()]
        );
    }

    #[test]
    fn test_map_kv() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let res = v.map_kv(|(k, v)| (k + 1, v.to_string()));
        assert_eq!(
            res,
            btreemap![2 => "a".to_string(), 3 => "b".to_string(), 4 => "c".to_string()]
        );
    }

    #[test]
    fn test_map_value_r() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let res = (&v).map_value(|v| v.to_string());
        assert_eq!(
            res,
            btreemap![&1 => "a".to_string(), &2 => "b".to_string(), &3 => "c".to_string()]
        );
    }

    #[test]
    fn test_map_kv_r() {
        let v = btreemap![1 => "a",2 => "b",3 => "c"];
        let res = (&v).map_kv(|(k, v)| (k.to_string(), v.to_string()));
        assert_eq!(
            res,
            btreemap!["1".to_string() => "a".to_string(), "2".to_string() => "b".to_string(), "3".to_string() => "c".to_string()]
        );
    }
}
