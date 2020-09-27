use iterable::*;

fn main() {
    let int_vec = vec![1,2,3];
    // with_filter is a lazy combinator
    let string_vec = int_vec.lazy_filter(|x| x > &1).map(|x| format!("{}", x));
    string_vec.foreach(|x| println!("{}", x));
}
