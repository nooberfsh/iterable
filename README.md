# Iterable
An iterable library for Rust collection like types.

## Prerequisites
 - latest rust nightly compiler
 
## Use 

```toml
# Cargo.toml
[dependencies]
iterable = "0.1"
```

## Features

iterate collection like types without `iter()` and `collect()`:
```rust
use iterable::*;

fn main() {
    let int_vec = vec![1,2,3];
    // map
    let string_vec = int_vec.map(|x| format!("{}", x));
    // filter
    let filtered_vec = string_vec.filter(|x| &**x > "1");
    // foreach
    filtered_vec.foreach(|x| println!("{}", x));
}
```

# License
MIT
