use crate::base::hkt::Hkt1;

pub trait Functor<A>: Sized {
    type Map<T, F>: Functor<T>;

    // fn fmap<B, F>(self, f: &F) -> Self::Of<B>
    // where
    //     F: Fn(Self::HktOf1) -> B;

    fn fmap<B, F>(self, f: &F) -> Self::Map<B, F>
    where
        F: Fn(A) -> B;

    // fn fmap1<F>(self, f: &F) -> Self
    // where
    //     F: Fn(Self::HktOf1) -> Self::HktOf1;

    fn fmap1<F>(self, f: &F) -> Self
    where
        F: Fn(A) -> A;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::impl_hkt1;

    impl_hkt1!(Option);

    impl<A> Functor<A> for Option<A> {
        type Map<B, F> = Option<B>;

        fn fmap<B, F>(self, f: &F) -> Option<B>
        where
            F: Fn(A) -> B,
        {
            match self {
                Some(x) => Some(f(x)),
                None => None,
            }
        }

        fn fmap1<F>(self, f: &F) -> Option<A>
        where
            F: Fn(A) -> A,
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
