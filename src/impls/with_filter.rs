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
    use super::*;

    #[test]
    fn test_c() {
        let v = vec![1, 2, 3];
        let res = v.with_filter(|i| i > &1).filter(|i| i > &2);
        assert_eq!(res, vec![3]);
    }

    #[test]
    fn test_cc() {
        let v = vec![1, 2, 3];
        let res = v.with_filter(|i| i > &1).map(|i| i.to_string());
        assert_eq!(res, vec!["2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_c_r() {
        let v = vec![1, 2, 3];
        let res = (&v).with_filter(|i| i > &&1).filter(|i| i > &&2);
        assert_eq!(res, vec![&3]);
    }

    #[test]
    fn test_cc_r() {
        let v = vec![1, 2, 3];
        let res = (&v).with_filter(|i| i > &&1).map(|i| i.to_string());
        assert_eq!(res, vec!["2".to_string(), "3".to_string()]);
    }
}
