mod lazy_map;
mod lazy_filter;
mod lazy_filter_map;

pub use lazy_map::*;
pub use lazy_filter::*;
pub use lazy_filter_map::*;


// only used for test lazy combinator
#[cfg(test)]
fn collect<I: crate::Iterable>(i: I) -> I::F
where I::F: crate::Producer<I::Item>
{
    use crate::Producer;
    I::F::produce(i.consume())
}
