use crate::base::hkt::Hkt1;

pub trait Functor: Hkt1 + Sized {
    type Map<B, F>: Functor
    where
        F: Fn(Self::HktArg1) -> B + Clone;

    fn fmap<B, F>(self, f: F) -> Self::Map<B, F>
    where
        F: Fn(Self::HktArg1) -> B + Clone;

    fn fmap1<F>(self, f: F) -> Self::Map<Self::HktArg1, F>
    where
        F: Fn(Self::HktArg1) -> Self::HktArg1 + Clone,
    {
        self.fmap(f)
    }
}
