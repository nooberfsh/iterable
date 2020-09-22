use crate::Iterable;
use std::cmp::Ordering;

#[test]
fn test_flat_map() {
    let v = vec![1,2];
    let res = v.flat_map(|x| [x, x]);
    assert_eq!(res, vec![1,1,2,2]);
}

#[test]
fn test_flatten() {
    let v = vec![[1,2], [3,4]];
    let res = v.flatten();
    assert_eq!(res, vec![1,2,3,4]);
}

#[test]
fn test_by_ref() {
    let v = vec![1, 2, 3];
    let res = v.by_ref() as *const _ as usize;
    assert_eq!(res, &v as *const _ as usize);
}

#[test]
fn test_partition() {
    let v = vec![1, 2, 3];
    let (l, r) = v.partition(|x| x < &2);
    assert_eq!(l, vec![1]);
    assert_eq!(r, vec![2, 3]);
}

#[test]
fn test_try_fold() {
    let v = vec![1, 2, 3];
    let f = |s: i32, x: i32| if s > 10 { None } else { Some(s+x)};
    let res = v.try_fold(10, f);
    assert!(res.is_none());

    let v = vec![1, 2, 3];
    let res = v.try_fold(0, f);
    assert_eq!(res, Some(6));
}

#[test]
fn test_try_for_each() {
    let v = vec![1, 2, 3];
    let f = |x: i32| if x > 2 { None } else { Some(()) };
    let res = v.try_for_each(f);
    assert!(res.is_none());

    let v = vec![1, 1, 1];
    let res = v.try_for_each(f);
    assert!(res.is_some());
}

#[test]
fn test_fold() {
    let v = vec![1, 2, 3];
    let res = v.fold(0, |s, a| s + a);
    assert_eq!(res, 6);
}

#[test]
fn test_all() {
    let v = vec![1, 2, 3];
    let res = v.clone().all(|x| x >= 1);
    assert!(res);

    let res = v.all(|x| x >= 2);
    assert!(!res);
}

#[test]
fn test_any() {
    let v = vec![1, 2, 3];
    let res = v.clone().any(|x| x < 1);
    assert!(!res);

    let v = vec![1, 2, 3];
    let res = v.any(|x| x >= 1);
    assert!(res);
}

#[test]
fn test_find() {
    let v = vec![1, 2, 3];
    let res = v.clone().find(|x| x < &1);
    assert_eq!(res, None);

    let res = v.find(|x| x == &1);
    assert_eq!(res, Some(1));
}

#[test]
fn test_find_map() {
    let v = vec![1, 2, 3];
    let res: Option<String> = v.clone().find_map(|_| None);
    assert_eq!(res, None);

    let res = v.find_map(|_| Some("123"));
    assert_eq!(res, Some("123"));
}

#[test]
fn test_position() {
    let v = vec![1, 2, 3];
    let res = v.clone().position(|x| x < 1);
    assert_eq!(res, None);

    let res = v.position(|x| x == 1);
    assert_eq!(res, Some(0));
}

#[test]
fn test_rposition() {
    let v = vec![1, 2, 3];
    let res = v.clone().rposition(|x| x < 1);
    assert_eq!(res, None);

    let res = v.rposition(|x| x == 1);
    assert_eq!(res, Some(0));
}

#[test]
fn test_max() {
    let v = vec![1, 2, 3];
    let res = Iterable::max(v);
    assert_eq!(res, Some(3));
}

#[test]
fn test_min() {
    let v = vec![1, 2, 3];
    let res = Iterable::min(v);
    assert_eq!(res, Some(1));
}

#[test]
fn test_max_by_key() {
    let v = vec![1, 2, 3];
    let res = v.max_by_key(|x| *x);
    assert_eq!(res, Some(3));
}

#[test]
fn test_max_by() {
    let v = vec![1, 2, 3];
    let res = v.max_by(|l, r| l.cmp(r));
    assert_eq!(res, Some(3));
}

#[test]
fn test_min_by_key() {
    let v = vec![1, 2, 3];
    let res = v.min_by_key(|x| *x);
    assert_eq!(res, Some(1));
}

#[test]
fn test_min_by() {
    let v = vec![1, 2, 3];
    let res = v.min_by(|l, r| l.cmp(r));
    assert_eq!(res, Some(1));
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
