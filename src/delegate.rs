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

macro_rules! delegate_from_iterator {
    ($it:ty, $item:ty, impl $($args:tt)*) => {
        impl $($args)* crate::Producer<$item> for $it {
            fn from_iter<IT>(iter: IT) -> Self
            where
                IT: IntoIterator<Item = $item>,
            {
                <Self as std::iter::FromIterator<$item>>::from_iter(iter)
            }
        }
    }
}

macro_rules! delegate_extend {
    ($it:ty, $item:ty, impl $($args:tt)*) => {
        impl $($args)* crate::GrowableProducer<$item> for $it {
            fn empty() -> Self {
                Default::default()
            }
            fn add_one(&mut self, a: $item) {
                <Self as std::iter::Extend<$item>>::extend_one(self, a);
            }
        }
    }
}
