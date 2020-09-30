use iterable::Iterable;

#[test]
fn test_join() {
    let a = vec!["1", "2", "3"];
    let res = (&a).join(",");
    assert_eq!(res, "1,2,3");
    let res = a.join(",");
    assert_eq!(res, "1,2,3");
}

