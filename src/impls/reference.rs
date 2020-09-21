use crate::{Iterable, IterableMap, Consumer};

impl<'a, I> Iterable for &'a I
where
    I: Iterable,
    &'a I: Consumer,
{
    type C = I::CR<'a>;
    type CC<U> = I::CC<U>;
    type CF<U> = I::CF<U>;
    type CR<'b> = I::CR<'b>;
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
    fn test_c_rr() {
        let v = vec![1, 2, 3];
        let res = (&&v).filter(|i| i > &&1);
        assert_eq!(res, vec![&2, &3]);
    }

    #[test]
    fn test_cc_rr() {
        let v = vec![1, 2, 3];
        let res = (&&v).map(|i| i.to_string());
        assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_map_value_rr() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = (&&v).map_value(|v| v.to_string());
        assert_eq!(
            res,
            hashmap![&1 => "a".to_string(), &2 => "b".to_string(), &3 => "c".to_string()]
        );
    }

    #[test]
    fn test_map_kv_rr() {
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let res = (&&v).map_kv(|(k, v)| (k.to_string(), v.to_string()));
        assert_eq!(
            res,
            hashmap!["1".to_string() => "a".to_string(), "2".to_string() => "b".to_string(), "3".to_string() => "c".to_string()]
        );
    }
}
