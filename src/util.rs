use std::ops::ControlFlow;

pub trait TryExt {
    type Output;
    type Error;
    type Map<U>: TryExt<Output = U, Error = Self::Error>;

    fn from_output(o: Self::Output) -> Self;
    fn branch<U>(self) -> ControlFlow<Self::Map<U>, Self::Output>;
}

impl<T, E> TryExt for Result<T, E> {
    type Output = T;
    type Error = E;
    type Map<U> = Result<U, E>;

    fn from_output(o: T) -> Self {
        Ok(o)
    }
    fn branch<U>(self) -> ControlFlow<Self::Map<U>, Self::Output> {
        match self {
            Ok(d) => ControlFlow::Continue(d),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

impl<T> TryExt for Option<T> {
    type Output = T;
    type Error = ();
    type Map<U> = Option<U>;

    fn from_output(o: T) -> Self {
        Some(o)
    }

    fn branch<U>(self) -> ControlFlow<Self::Map<U>, Self::Output> {
        match self {
            Some(d) => ControlFlow::Continue(d),
            None => ControlFlow::Break(None),
        }
    }
}
