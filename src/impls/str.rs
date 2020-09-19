use crate::{Iterable, Consumer};

impl<'a> Iterable for &'a str {
    type C = String;
    type CC<U> = Vec<U>;
    type CR<'b> = String; // unused
}

impl<'a> Consumer for &'a str {
    type Item = char;
    type IntoIter = std::str::Chars<'a>;

    fn into_iter(self) -> Self::IntoIter {self.chars()}
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
    fn test_cc() {
        let v = "123你我";
        let res = v.map(|_| 1u8);
        assert_eq!(res, vec![1, 1, 1, 1, 1]);
    }
}
