use crate::{Iterable, IterableSeq};

impl<'a, T: 'a> Iterable for &'a [T] {
    type C = Vec<&'a T>;
    type CC<U> = Vec<U>;
}

impl<'a, T: 'a> IterableSeq for &'a [T] {}

delegate_into_iterator!(&'a [T], impl <'a, T: 'a>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let v: &[i32] = &[1, 2, 3];
        let res = v.filter(|i| i > &&1);
        assert_eq!(res, vec![&2, &3]);
    }

    #[test]
    fn test_cc() {
        let v: &[i32] = &[1, 2, 3];
        let res = v.map(|i| i + 1);
        assert_eq!(res, vec![2, 3, 4]);
    }

    #[test]
    fn test_sort() {
        let v: &[i32] = &[1, 2, 3];
        let res = v.sorted();
        assert_eq!(res, vec![&1, &2, &3]);
    }
}
