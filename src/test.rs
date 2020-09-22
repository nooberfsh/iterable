use crate::Iterable;

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