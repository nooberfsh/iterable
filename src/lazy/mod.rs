mod lazy_filter;

pub use lazy_filter::*;


// only used for test lazy combinator
#[cfg(test)]
fn collect<I: crate::Iterable>(i: I) -> I::F
where I::F: crate::Producer<I::Item>
{
    use crate::Producer;
    I::F::produce(i.consume())
}
