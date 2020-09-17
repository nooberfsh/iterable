#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod iterable;
mod impls;

pub use iterable::*;
pub use impls::with_filter::*;
pub use impls::*;
