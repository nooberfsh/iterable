use std::ops::Try;

pub trait TryExt: Try {
    type Map<U>: TryExt<Output = U, Residual = Self::Residual>;
}

impl<T, E> TryExt for Result<T, E> {
    type Map<U> = Result<U, E>;
}

impl<T> TryExt for Option<T> {
    type Map<U> = Option<U>;
}
