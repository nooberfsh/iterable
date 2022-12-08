use crate::{Consumer, Iterable, IterableSeq};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyStepBy<I> {
    pub(crate) iterable: I,
    pub(crate) step: usize,
}

impl<I> Iterable for LazyStepBy<I>
where
    I: Iterable,
{
    type C = I::C;
    type CC<U> = I::CC<U>;
    // remove below after `associated_type_defaults` stabilized
    type F = I::C;
    type CF<U> = I::CC<U>;
}

impl<I> IterableSeq for LazyStepBy<I> where I: IterableSeq {}

impl<I> Consumer for LazyStepBy<I>
where
    I: Consumer,
{
    type Item = I::Item;
    type IntoIter = std::iter::StepBy<I::IntoIter>;
    fn consume(self) -> Self::IntoIter {
        self.iterable.consume().step_by(self.step)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![0, 1, 2, 3, 4, 5];
        let res = collect(v.lazy_step_by(2));
        assert_eq!(res, vec![0, 2, 4]);
    }
}
