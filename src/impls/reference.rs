use crate::{Iterable, IterableMap};

impl<'a, I> Iterable for &'a I
where
    I: Iterable,
    &'a I: IntoIterator,
{
    type C = I::CR<'a>;
    type CC<U> = I::CC<U>;
    type CR<'b> = I::CR<'b>;
}

impl<'a, K, V, IM> IterableMap<&'a K, &'a V> for &'a IM
where
    IM: IterableMap<K, V>,
    &'a IM: Iterable<Item = (&'a K, &'a V)>,
{
    type CCMap<X, Y> = IM::CCMap<X, Y>;
}
