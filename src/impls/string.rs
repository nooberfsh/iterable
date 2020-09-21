use crate::{Iterable, Consumer};

impl Iterable for String {
    type C = Self;
    type CC<U> = Vec<U>;
    type CR<'a> = String;
}

impl Consumer for String {
    type Item = char;
    type IntoIter = Chars;

    fn into_iter(self) -> Self::IntoIter {
        Chars {
            // TODO: use String.into_bytes to avoid alloc
            bytes: self.chars().collect(),
            idx: 0
        }
    }
}

impl<'a> Consumer for &'a String {
    type Item = char;
    type IntoIter = std::str::Chars<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

pub struct Chars {
    // TODO: use Vec<u8> to avoid alloc
    bytes: Vec<char>,
    idx: usize,
}

impl Iterator for Chars {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.bytes.len() {
            None
        } else {
            let ret = self.bytes[self.idx];
            self.idx += 1;
            Some(ret)
        }
    }
}

delegate_from_iterator!(String, char, impl);
delegate_extend!(String, char, impl);
delegate_from_iterator!(String, &'a char, impl <'a>);
delegate_extend!(String, &'a char, impl <'a>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let v = "123你我".to_string();
        let res = v.filter(|c| *c != '你');
        assert_eq!(res, "123我".to_string());
    }

    #[test]
    fn test_cc() {
        let v = "123你我".to_string();
        let res = v.map(|_| 1u8);
        assert_eq!(res, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_c_r() {
        let v = "123你我".to_string();
        let res = (&v).filter(|c| *c != '你');
        assert_eq!(res, "123我".to_string());
    }

    #[test]
    fn test_cc_r() {
        let v = "123你我".to_string();
        let res = (&v).map(|_| 1u8);
        assert_eq!(res, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_chars_iterator() {
        let mut chars = Chars {
            bytes: vec!['1'],
            idx: 0,
        };
        let a = chars.next();
        assert_eq!(a, Some('1'));
        let b = chars.next();
        assert_eq!(b, None);
        let c = chars.next();
        assert_eq!(c, None);
    }
}
