use std::collections::{HashMap, HashSet};

use crate::{Iterable, IterableMap};

impl<T> Iterable for Vec<T> {
    type C = Self;
    type CC<U> = Vec<U>;
}

impl<T> Iterable for HashSet<T> {
    type C = Self;
    type CC<U> = HashSet<U>;
}

impl<K, V> Iterable for HashMap<K, V> {
    type C = Self;
    type CC<U> = Vec<U>;
}

impl<K, V> IterableMap<K, V> for HashMap<K, V> {
    type CCMap<X, Y> = HashMap<X, Y>;
}

#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3];

        let res = v.clone().map(|i| i.to_string());
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);

        let res = v.filter(|i| i > &1);
        assert_eq!(res, vec![2, 3]);
    }

    #[test]
    fn test_hash_set() {
        let v = hashset![1, 2, 3];

        let res = v.clone().map(|i| i.to_string());
        assert_eq!(
            res,
            hashset!["1".to_string(), "2".to_string(), "3".to_string()]
        );

        let res = v.filter(|i| i > &1);
        assert_eq!(res, hashset![2, 3]);
    }

    #[test]
    fn test_hash_map() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];

        let mut res = v.clone().map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);

        let res = v.filter(|(i, _)| i > &1);
        assert_eq!(res, hashmap![2=>"b", 3 => "c"]);
    }

    #[test]
    fn test_hash_map2() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];

        let res = v.clone().map_value(|v| v.to_string());
        assert_eq!(
            res,
            hashmap![1 => "a".to_string(), 2 => "b".to_string(), 3 => "c".to_string()]
        );

        let res = v.clone().map_kv(|(k, v)| (k + 1, v.to_string()));
        assert_eq!(
            res,
            hashmap![2 => "a".to_string(), 3 => "b".to_string(), 4 => "c".to_string()]
        );
    }
}
