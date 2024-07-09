use crate::{hkt::Hkt1, impl_hkt1};

pub trait Functor: Hkt1 + Sized {
    fn fmap<B, F>(self, f: &F) -> Self::Of<B>
    where
        F: Fn(Self::HktOf1) -> B;

    fn fmap1<F>(self, f: &F) -> Self
    where
        F: Fn(Self::HktOf1) -> Self::HktOf1;
}

#[cfg(test)]
mod tests {

    use super::*;

    impl_hkt1!(Option);

    impl<T> Functor for Option<T> {
        fn fmap<B, F>(self, f: &F) -> Self::Of<B>
        where
            F: Fn(T) -> B,
        {
            match self {
                Some(x) => Some(f(x)),
                None => None,
            }
        }

        fn fmap1<F>(self, f: &F) -> Self
        where
            F: Fn(T) -> T,
        {
            match self {
                Some(x) => Some(f(x)),
                None => None,
            }
        }
    }

    #[test]
    fn test_option_functor() {
        assert_eq!(None.fmap(&|x: i32| x + 1), None);
        assert_eq!(Some(1).fmap(&|x: i32| x + 1), Some(2));
    }
}
