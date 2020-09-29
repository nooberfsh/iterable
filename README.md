# Iterable
An iterable library for Rust collection like types.

## Prerequisites
 - latest rust nightly compiler
 
## Installation 

```toml
# Cargo.toml
[dependencies]
iterable = "0.2"
```

## Features

iterate collection like types without `iter()` and `collect()`:

```rust
use iterable::*;

fn main() {
    // convert a vec of i32 to a vec of String
    let v = vec![1, 2, 3];
    // only one `map` function instead of `v.iter().map(|i| i.to_string()).collect()`
    let res = v.map(|i| i.to_string());
    assert_eq!(res, vec!["1".to_string(), "2".to_string(), "3".to_string()]);

    // iterable also support array and string
    let v = [1, 2, 3];
    // res's type: [i32; 3]
    let res = v.rev();
    assert_eq!(res, [3, 2, 1]);
    
    // lazy combinator
    let v = vec![1,2,3];
    let s = v
        .lazy_filter(|x| x > &1)
        .lazy_map(|x| x.to_string())
        .rev();
    assert_eq!(s, vec!["3".to_string(), "2".to_string()]);

    // iterate over reference
    let v = vec![1, 2, 3];
    let res = (&v).filter(|i| i > &&1);
    assert_eq!(res, vec![&2, &3]);
}
```

# License
MIT
