use std::marker::PhantomData;

pub trait Setter<T> {}

pub struct Set<T>(T);

impl<T> Setter<T> for Set<T> {}

impl<T> Set<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub fn retrieve(self) -> T {
        self.0
    }
}

pub struct UnSet<T>(PhantomData<T>);

impl<T> Setter<T> for UnSet<T> {}

impl<T> UnSet<T> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }
}
