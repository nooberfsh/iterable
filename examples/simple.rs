use iterable::*;

fn main() {
    let int_vec = vec![1, 2, 3];
    let string_vec = int_vec.map(|x| format!("{}", x)); // map
    let filtered_vec = string_vec.filter(|x| &**x > "1"); // filter
    filtered_vec.foreach(|x| println!("{}", x));
}
