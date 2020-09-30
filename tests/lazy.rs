use maplit::hashmap;

use iterable::*;

#[test]
fn chain_seq() {
    let v = vec![1,2,3];
    let s = v
        .lazy_filter(|x| x > &1)
        .lazy_map(|x| x.to_string())
        .lazy_rev()
        .lazy_rev()
        .rev();
    assert_eq!(s, vec!["3".to_string(), "2".to_string()]);
}

#[test]
fn chain_map() {
    let v = hashmap![1 => "1", 2 => "2"];
    let s = v
        .lazy_filter(|(i, _)| i > &1)
        .map_value(|v| v.to_string());
    assert_eq!(s, hashmap![2 => "2".to_string()])
}
