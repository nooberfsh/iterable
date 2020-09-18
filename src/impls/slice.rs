use crate::Iterable;

impl<'a, T: 'a> Iterable for &'a [T] {
    type C = Vec<&'a T>;
    type CC<U> = Vec<U>;
    // TODO: replace 'a with 'b
    type CR<'b> = Vec<&'a T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice() {
        let v: &[i32] = &[1, 2, 3];
        let res = v.map(|i| i + 1);
        assert_eq!(res, vec![2, 3, 4]);
    }
}
