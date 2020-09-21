use std::array::IntoIter;
use std::mem::MaybeUninit;

use crate::{Iterable, Consumer, Producer, GrowableProducer};

impl<T, const N: usize> Iterable for [T; N] {
    type C = Vec<T>;
    type CC<U> = Vec<U>;
    type F = [T; N];
    type CF<U> = [U; N];
    type CR<'a> where T: 'a= Vec<&'a T>;
    type FR<'a> where T: 'a = [&'a T; N];

    fn unzip<A, B>(self) -> (Self::CF<A>, Self::CF<B>)
    where
        Self: Sized,
        Self: Iterable<Item=(A, B)>,
    {
        let mut l: [MaybeUninit<A>; N] = MaybeUninit::uninit_array();
        let mut r: [MaybeUninit<B>; N] = MaybeUninit::uninit_array();
        for (i, (a, b)) in self.into_iter().enumerate() {
            l[i] = MaybeUninit::new(a);
            r[i] = MaybeUninit::new(b);
        }
        unsafe { (transmute(l), transmute(r)) }
    }
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
        unsafe { transmute(arr) }
    }
}

// TODO: can we remove this?
// this is only used for satisfy Iterable::unzip method
// we must not use it.
impl<T, const N: usize> GrowableProducer<T> for [T; N] {
    fn empty() -> Self {
        panic!("can not create empty array!")
    }

    fn add_one(&mut self, _: T) {
        panic!("can not add element to an array!")
    }
}

// TODO: workaround
// unsafe { mem::transmute::<_, [T; N]>(ret) } does not work yet
// https://github.com/rust-lang/rust/issues/61956
#[inline]
unsafe fn transmute<T, U, const N: usize>(mut arr: [MaybeUninit<T>; N]) -> U {
    let ptr = &mut arr as *mut _ as *mut U;
    let res = ptr.read();
    core::mem::forget(arr);
    res
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
    fn test_f() {
        let v = [1, 2, 3];
        let res = v.rev();
        assert_array(&res);
        assert_eq!(res, [3, 2, 1]);
    }

    #[test]
    fn test_cf() {
        let v = [1, 2, 3];
        let res = Iterable::map(v, |i| i.to_string());
        assert_array(&res);
        assert_eq!(res, ["1".to_string(), "2".to_string(), "3".to_string()]);

        let v = [(1,2), (3, 4), (5,6)];
        let (a, b) = v.unzip();
        assert_array(&a);
        assert_array(&b);
        assert_eq!(a, [1, 3, 5]);
        assert_eq!(b, [2, 4, 6]);
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
    fn test_f_r() {
        let v = [1, 2, 3];
        let res = (&v).rev();
        assert_array(&res);
        assert_eq!(res, [&3, &2, &1]);
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

    #[test]
    #[should_panic]
    fn test_growable_producer_empty() {
        <[i32; 10] as GrowableProducer<i32>>::empty();
    }

    #[test]
    #[should_panic]
    fn test_growable_producer_add_one() {
        let a = &mut [1,2,3];
        <[i32; 3] as GrowableProducer<i32>>::add_one(a, 1);
    }
}
