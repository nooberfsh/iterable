use iterable::*;

#[test]
fn chain() {
    let v = vec![1,2,3];
    let s = v
        .lazy_filter(|x| x > &1)
        .lazy_map(|x| x.to_string())
        .lazy_rev()
        .lazy_rev()
        .rev();
    assert_eq!(s, vec!["3".to_string(), "2".to_string()]);
}
