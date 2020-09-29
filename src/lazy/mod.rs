mod lazy_step_by;
mod lazy_chain;
mod lazy_zip;
mod lazy_map;
mod lazy_filter;
mod lazy_filter_map;
mod lazy_enumerate;
mod lazy_flat_map;
mod lazy_flatten;
mod lazy_cycle;
mod lazy_rev;

pub use self::lazy_step_by::*;
pub use self::lazy_chain::*;
pub use self::lazy_zip::*;
pub use self::lazy_map::*;
pub use self::lazy_filter::*;
pub use self::lazy_filter_map::*;
pub use self::lazy_enumerate::*;
pub use self::lazy_flat_map::*;
pub use self::lazy_flatten::*;
pub use self::lazy_cycle::*;
pub use self::lazy_rev::*;


// only used for test lazy combinator
#[cfg(test)]
fn collect<I: crate::Iterable>(i: I) -> I::F
where I::F: crate::Producer<I::Item>
{
    use crate::Producer;
    I::F::produce(i.consume())
}
