use std::collections::HashMap;

use crate::{Iterable, IterableMap};

impl<K, V> Iterable for HashMap<K, V> {
    type C = Self;
    type CC<U> = Vec<U>;
    type CR<'a> where K: 'a, V: 'a = HashMap<&'a K, &'a V>;
}

impl<K, V> IterableMap<K, V> for HashMap<K, V> {
    type CCMap<X, Y> = HashMap<X, Y>;
}

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
