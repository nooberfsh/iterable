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
    assert_eq!(res, vec![3, 2, 1])
}