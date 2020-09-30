use crate::{Iterable, IterableSeq, Consumer};

impl<'a> Iterable for &'a str {
    type C = String;
    type CC<U> = Vec<U>;
}

impl<'a> IterableSeq for &'a str {}

impl<'a> Consumer for &'a str {
    type Item = char;
    type IntoIter = std::str::Chars<'a>;

    fn consume(self) -> Self::IntoIter {self.chars()}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let v = "123你我";
        let res = v.filter(|c| *c != '你');
        assert_eq!(res, "123我".to_string());
    }

    #[test]
    fn test_f() {
        let v = "123你我";
        let res = v.rev();
        assert_eq!(res, "我你321".to_string());
    }

    #[test]
    fn test_cc() {
        let v = "123你我";
        let res = v.map(|_| 1u8);
        assert_eq!(res, vec![1, 1, 1, 1, 1]);
    }
}
