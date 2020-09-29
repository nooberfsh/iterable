use crate::{Iterable, Consumer};

#[must_use = "iterable adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct LazyScan<S, I, F> {
    pub(crate) iterable: I,
    pub(crate) state: S,
    pub(crate) f: F,
}

impl<S, I, F> Iterable for LazyScan<S, I, F>
where
    S: Clone,
    I: Iterable,
    F: Fn(S, I::Item) -> S
{
    type C = I::CC<S>;
    type CC<U> = I::CC<U>;
}

impl<S, I, F> Consumer for LazyScan<S, I, F>
where
    S: Clone,
    I: Consumer,
    F: Fn(S, I::Item) -> S
{
    type Item = S;
    type IntoIter = ScanIter<S, I::IntoIter ,F>;
    fn consume(self) -> Self::IntoIter {
        new_scan_iter(self.state, self.iterable, self.f)
    }
}

pub struct ScanIter<S, I, F>
where
    S: Clone,
    I: Iterator,
    F: Fn(S, I::Item) -> S
{
    pub (super) iter: I,
    pub (super) state: Option<S>,
    pub (super) f: F,
}

pub (crate) fn new_scan_iter<S, C, F>(s: S, c: C, f: F) -> ScanIter<S, C::IntoIter ,F>
    where
        S: Clone,
        C: Consumer,
        F: Fn(S, C::Item) -> S
{
    ScanIter {
        iter: c.consume(),
        state: Some(s),
        f
    }
}

impl<S, I ,F> Iterator for ScanIter<S, I, F>
where
    S: Clone,
    I: Iterator,
    F: Fn(S, I::Item) -> S
{
    type Item = S;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state.take() {
            None => None,
            Some(s) => {
                if let Some(a) = self.iter.next() {
                    self.state = Some((self.f)(s.clone(), a));
                }
                Some(s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lazy::collect;

    #[test]
    fn smoke() {
        let v = vec![1,2,3];
        let res = collect(v.lazy_scan(10, |s, a| s + a));
        assert_eq!(res, vec![10, 11, 13, 16]);

        let v: Vec<i32> = vec![];
        let res = collect(v.lazy_scan(10, |s, a| s + a));
        assert_eq!(res, vec![10]);
    }
}
