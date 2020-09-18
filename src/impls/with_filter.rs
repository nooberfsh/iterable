use crate::Iterable;

pub struct WithFilter<I, F> {
    pub(crate) iterable: I,
    pub(crate) f: F,
}

impl<I, F> Iterable for WithFilter<I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
    type CR<'a> = I::CR<'a>;
}

impl<I, F> IntoIterator for WithFilter<I, F>
where
    I: IntoIterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;
    type IntoIter = std::iter::Filter<I::IntoIter, F>;
    fn into_iter(self) -> Self::IntoIter {
        self.iterable.into_iter().filter(self.f)
    }
}

#[cfg(test)]
mod tests {
    use maplit::*;

    use super::*;

    #[test]
    fn test_reduction() {
        let v = vec![1, 2, 3];
        let expected = vec![3];
        let res = v.with_filter(|x| x > &1).filter(|x| x > &2);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3];
        let expected = vec!["2".to_string(), "3".to_string()];

        let res = (&v).with_filter(|x| x > &&1).map(|i| i.to_string());
        assert_eq!(res, expected);

        let res = v.with_filter(|x| x > &1).map(|i| i.to_string());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_set() {
        let v = hashset![1, 2, 3];
        let expected = hashset!["2".to_string(), "3".to_string()];

        let res = (&v).with_filter(|x| x > &&1).map(|i| i.to_string());
        assert_eq!(res, expected);

        let res = v.with_filter(|x| x > &1).map(|i| i.to_string());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_map() {
        // test Iterable
        let v = hashmap![1 => "a",2 => "b",3 => "c"];
        let expected = vec!["2".to_string(), "3".to_string()];

        let mut res = (&v)
            .with_filter(|(x, _)| x > &&1)
            .map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, expected);

        let mut res = v.with_filter(|(x, _)| x > &1).map(|(i, _)| i.to_string());
        res.sort();
        assert_eq!(res, expected);
    }
}
