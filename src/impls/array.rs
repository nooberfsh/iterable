use std::array::IntoIter;
use std::mem::MaybeUninit;
use std::cmp::Ordering;

use crate::{Iterable, IterableSeq, Consumer, Producer, GrowableProducer};

macro_rules! unzip {
    () => {
        fn unzip<A, B>(self) -> (Self::CF<A>, Self::CF<B>)
        where
            Self: Sized,
            Self: Iterable<Item=(A, B)>,
        {
            let mut l: [MaybeUninit<A>; N] = MaybeUninit::uninit_array();
            let mut r: [MaybeUninit<B>; N] = MaybeUninit::uninit_array();
            for (i, (a, b)) in self.consume().enumerate() {
                l[i] = MaybeUninit::new(a);
                r[i] = MaybeUninit::new(b);
            }
            unsafe { (transmute(l), transmute(r)) }
        }
    }
}

impl<T, const N: usize> Iterable for [T; N] {
    type C = Vec<T>;
    type CC<U> = Vec<U>;
    type F = [T; N];
    type CF<U> = [U; N];

    unzip!();
}

impl<T, const N: usize> IterableSeq for [T; N] {
    fn sorted(mut self) -> Self::F
    where
        T: Ord
    {
        self.sort();
        self
    }

    fn sorted_by<F>(mut self, f: F) -> Self::F
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering,
    {
        self.sort_by(f);
        self
    }

    fn sorted_by_key<K, F>(mut self, f: F) -> Self::F
    where
        K: Ord,
        F: Fn(&Self::Item) -> K,
    {
        self.sort_by_key(f);
        self
    }
}

impl<'a, T: 'a, const N: usize> Iterable for &'a [T; N] {
    type C = Vec<&'a T>;
    type CC<U> = Vec<U>;
    type F = [&'a T; N];
    type CF<U> = [U; N];

    unzip!();
}

impl<'a, T: 'a, const N: usize> IterableSeq for &'a [T; N] {}

impl<T, const N: usize> Consumer for [T; N] {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn consume(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T, const N: usize> Producer<T> for [T; N] {
    fn produce<IT>(iter: IT) -> Self
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

    fn grow_one(&mut self, _: T) {
        panic!("can not add element to an array!")
    }

    fn grow<C>(&mut self, _: C) where C: Consumer<Item = T> {
        panic!("can not add elements to an array!")
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
    use crate::assert_type;

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

    #[test]
    fn test_f() {
        let v = [1, 2, 3];
        let res = v.rev();
        assert_type::<[i32; 3]>(res);
    }

    #[test]
    fn test_cf() {
        let v = [1, 2, 3];
        let res = Iterable::map(v, |i| i.to_string());
        assert_type::<[String; 3]>(res);
    }

    #[test]
    fn test_unzip() {
        let v = [(1,2), (3, 4), (5,6)];
        let (a, b) = v.unzip();
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
        assert_type::<[&i32; 3]>(res);
    }

    #[test]
    fn test_cf_r() {
        let v = [1, 2, 3];
        let res = Iterable::map(&v, |i| i.to_string());
        assert_type::<[String; 3]>(res);
    }

    #[test]
    #[should_panic]
    fn test_producer1() {
        let v = vec![1, 2, 3];
        <[i32; 4]>::produce(v);
    }

    #[test]
    #[should_panic]
    fn test_producer2() {
        let v = vec![1, 2, 3, 4, 5];
        <[i32; 4]>::produce(v);
    }

    #[test]
    #[should_panic]
    fn test_growable_producer_empty() {
        <[i32; 10] as GrowableProducer<i32>>::empty();
    }

    #[test]
    #[should_panic]
    fn test_growable_producer_grow_one() {
        let a = &mut [1,2,3];
        <[i32; 3] as GrowableProducer<i32>>::grow_one(a, 1);
    }

    #[test]
    #[should_panic]
    fn test_growable_producer_grow() {
        let a = &mut [1,2,3];
        let s = vec![1, 2, 3];
        <[i32; 3] as GrowableProducer<i32>>::grow(a, s);
    }
}
