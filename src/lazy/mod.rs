mod lazy_chain;
mod lazy_cloned;
mod lazy_copied;
mod lazy_cycle;
mod lazy_enumerate;
mod lazy_filter;
mod lazy_filter_map;
mod lazy_flat_map;
mod lazy_flatten;
mod lazy_map;
mod lazy_map_while;
mod lazy_rev;
mod lazy_scan;
mod lazy_skip;
mod lazy_skip_while;
mod lazy_step_by;
mod lazy_take;
mod lazy_zip;

pub use self::lazy_chain::*;
pub use self::lazy_cloned::*;
pub use self::lazy_copied::*;
pub use self::lazy_cycle::*;
pub use self::lazy_enumerate::*;
pub use self::lazy_filter::*;
pub use self::lazy_filter_map::*;
pub use self::lazy_flat_map::*;
pub use self::lazy_flatten::*;
pub use self::lazy_map::*;
pub use self::lazy_map_while::*;
pub use self::lazy_rev::*;
pub use self::lazy_scan::*;
pub use self::lazy_skip::*;
pub use self::lazy_skip_while::*;
pub use self::lazy_step_by::*;
pub use self::lazy_take::*;
pub use self::lazy_zip::*;

// only used for test lazy combinator
#[cfg(test)]
fn collect<I: crate::Iterable>(i: I) -> I::F
where
    I::F: crate::Producer<I::Item>,
{
    use crate::Producer;
    I::F::produce(i.consume())
}
