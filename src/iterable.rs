use crate::hkt::Hkt1;

pub trait Iterable: Hkt1 {
    fn it<'a>(&'a self) -> impl Iterator<Item = &'a Self::HktOf1>;
}

pub trait IterableMut: Iterable {
    fn it_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Self::HktOf1>;
}

pub trait IterableOnce: IterableMut {
    fn into_it(self) -> impl Iterator<Item = Self::HktOf1>;
}
