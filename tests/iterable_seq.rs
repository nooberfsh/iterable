use iterable::IterableSeq;

#[test]
fn test_sorted() {
    let a = vec![5, 10, 3, 0];
    let res = a.sorted();
    assert_eq!(res, vec![0, 3, 5, 10]);
}

#[test]
fn test_sorted_by() {
    let a = vec![5, 10, 3, 0];
    let res = a.sorted_by(|l, r| l.cmp(r));
    assert_eq!(res, vec![0, 3, 5, 10]);
}

#[test]
fn test_sorted_by_key() {
    let a = vec![5, 10, 3, 0];
    let res = a.sorted_by_key(|i|*i);
    assert_eq!(res, vec![0, 3, 5, 10]);
}
