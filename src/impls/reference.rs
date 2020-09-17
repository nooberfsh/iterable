use crate::{Iterable, IterableMap};

impl<'a, I> Iterable for &'a I
    where
        I: Iterable,
        &'a I: IntoIterator,
{
    type C = I::CC<<&'a I as IntoIterator>::Item>;
    type CC<U> = I::CC<U>;
}

impl<'a, K, V, IM> IterableMap<&'a K, &'a V> for &'a IM
    where
        IM: IterableMap<K, V>,
        &'a IM: Iterable<Item = (&'a K, &'a V)>,
{
    type CCMap<X, Y> = IM::CCMap<X, Y>;
}


#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3];

        let res = (&v).map(|i| i.to_string());
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);

        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, vec![&2, &3]);
    }

    #[test]
    fn test_hash_set() {
        let v = hashset![1, 2, 3];

        let res = (&v).map(|i| i.to_string());
        assert_eq!(res, hashset!["1".to_string(), "2".to_string(), "3".to_string()]);

        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, hashset![&2, &3]);
    }

    #[test]
    fn test_hash_map() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];

        let mut res = (&v).map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);

        let mut res = (&v).filter(|(i,_)| i > &&1);
        res.sort();
        assert_eq!(res, vec![(&2, &"b"), (&3, &"c")]);
    }

    #[test]
    fn test_hash_map2() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];

        let res = (&v).map_value(|v| v.to_string());
        assert_eq!(res, hashmap![&1 => "a".to_string(), &2 => "b".to_string(), &3 => "c".to_string()]);

        let res = (&v).map_kv(|(k, v)| (k.to_string(), v.to_string()));
        assert_eq!(res, hashmap!["1".to_string() => "a".to_string(), "2".to_string() => "b".to_string(), "3".to_string() => "c".to_string()]);
    }
}