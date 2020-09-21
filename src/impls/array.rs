use std::array::IntoIter;
use std::mem::MaybeUninit;


use crate::{Iterable, Consumer, Producer};

impl<T, const N: usize> Iterable for [T; N] {
    type C = Vec<T>;
    type CC<U> = Vec<U>;
    type CF<U> = [U; N];
    type CR<'a> where T: 'a= Vec<&'a T>;
}

impl<T, const N: usize> Consumer for [T; N] {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T, const N: usize> Producer<T> for [T; N] {
    fn from_iter<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = T>
    {
        let mut arr: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
        let mut count = 0;
        for (i, t) in iter.into_iter().enumerate() {
            if i >= N {
                panic!("iter's length greater then array's length")
            }
            arr[i] = MaybeUninit::new(t);
            count += 1;
        }
        if count < N {
            panic!("iter's length less than array's length")
        }

        // unsafe { mem::transmute::<_, [T; N]>(ret) }
        // above does not work yet
        // https://github.com/rust-lang/rust/issues/61956
        // workaround
        let ptr = &mut arr as *mut _ as *mut [T; N];
        let res = unsafe { ptr.read() };
        core::mem::forget(arr);
        res
    }
}

delegate_into_iterator!(&'a [T; N], impl <'a, T: 'a, const N: usize>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let v = [1, 2, 3];
        let res = v.filter(|i| i > &1);
        assert_eq!(res, vec![2, 3]);
    }

    #[test]
    fn test_cc() {
        let v = [1, 2, 3];
        let res = v.flat_map(|i| vec![i, 1]);
        assert_eq!(res, vec![1, 1, 2, 1, 3, 1]);
    }

    fn assert_array<T, const N: usize>(_: &[T; N]) {}

    #[test]
    fn test_cf() {
        let v = [1, 2, 3];
        let res = Iterable::map(v, |i| i.to_string());
        assert_array(&res);
        assert_eq!(res, ["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn test_c_r() {
        let v = [1, 2, 3];
        let res = (&v).filter(|i| i > &&1);
        assert_eq!(res, [&2, &3]);
    }

    #[test]
    fn test_cc_r() {
        let v = [1, 2, 3];
        let res = (&v).flat_map(|i| vec![*i, 1]);
        assert_eq!(res, vec![1, 1, 2, 1, 3, 1]);
    }

    #[test]
    fn test_cf_r() {
        let v = [1, 2, 3];
        let res = Iterable::map(&v, |i| i.to_string());
        assert_array(&res);
        assert_eq!(res, ["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    #[should_panic]
    fn test_producer1() {
        let v = vec![1, 2, 3];
        <[i32; 4]>::from_iter(v);
    }

    #[test]
    #[should_panic]
    fn test_producer2() {
        let v = vec![1, 2, 3, 4, 5];
        <[i32; 4]>::from_iter(v);
    }
}
