use crate::Iterable;
use std::cmp::Ordering;

#[test]
fn test_growable() {
    let v = vec![1, 2, 3];
    let (l, r) = v.partition(|x| x <= &1);
    assert_eq!(l , vec![1]);
    assert_eq!(r , vec![2, 3]);
}

#[test]
fn test_growable_r() {
    let v = vec![1, 2, 3];
    let (l, r) = (&v).partition(|x| x <= &&1);
    assert_eq!(l , vec![&1]);
    assert_eq!(r , vec![&2, &3]);
}

#[test]
fn test_rev() {
    let v = vec![1, 2, 3];
    let res = v.rev();
    assert_eq!(res, vec![3, 2, 1]);
}

#[test]
fn test_unzip() {
    let v = vec![(1,2), (3, 4), (5,6)];
    let (a, b) = v.unzip();
    assert_eq!(a, vec![1, 3, 5]);
    assert_eq!(b, vec![2, 4, 6]);
}

#[test]
fn test_copied() {
    let v = vec![&1, &2, &3];
    let a = v.copied();
    assert_eq!(a, vec![1, 2, 3])
}

#[test]
fn test_cloned() {
    let s = &"123".to_string();
    let v = vec![s];
    let b = v.cloned();
    assert_eq!(b, vec!["123".to_string()]);
}

#[test]
fn test_sum() {
    let v = vec![1, 2, 3];
    let a: i32 = v.sum();
    assert_eq!(a, 6)
}

#[test]
fn test_product() {
    let v = vec![2, 2, 3];
    let a: i32 = v.product();
    assert_eq!(a, 12)
}

#[test]
fn test_cmp() {
    let l = vec![1, 2, 3];
    let r = [1,2,3];
    let a = l.cmp(r);
    assert_eq!(a, Ordering::Equal)
}

#[test]
fn test_partial_cmp() {
    let l = vec![2, 2, 3];
    let r = [1,2,3];
    let a = l.partial_cmp(r);
    assert_eq!(a, Some(Ordering::Greater))
}

#[test]
fn test_eq() {
    let l = vec![2, 2, 3];
    let r = [1,2,3];
    let a = l.eq(r);
    assert!(!a)
}

#[test]
fn test_ne() {
    let l = vec![2, 2, 3];
    let r = [1,2,3];
    let a = l.ne(r);
    assert!(a)
}

#[test]
fn test_lt() {
    let l = vec![2, 2, 3];
    let r = [1,2,3];
    let a = l.lt(r);
    assert!(!a)
}

#[test]
fn test_le() {
    let l = vec![2, 2, 3];
    let r = [1,2,3];
    let a = l.le(r);
    assert!(!a)
}

#[test]
fn test_gt() {
    let l = vec![2, 2, 3];
    let r = [1,2,3];
    let a = l.gt(r);
    assert!(a)
}

#[test]
fn test_ge() {
    let l = vec![2, 2, 3];
    let r = [1,2,3];
    let a = l.ge(r);
    assert!(a)
}
