macro_rules! delegate_into_iterator {
    ($it:ty, impl $($args:tt)*) => {
        impl $($args)* crate::Consumer for $it {
            type Item = <$it as std::iter::IntoIterator>::Item;
            type IntoIter = <$it as std::iter::IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                <Self as IntoIterator>::into_iter(self)
            }
        }
    }
}