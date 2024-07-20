use crate::base::hkt::Hkt1;
use crate::impl_hkt1;

use super::functor::Functor;

impl_hkt1!(Option);

impl<A> Functor for Option<A> {
    type Map<B, F> = Option<B>
    where
        F: Fn(A) -> B + Clone;

    #[inline(always)]
    fn fmap<B, F>(self, f: F) -> Option<B>
    where
        F: Fn(A) -> B + Clone,
    {
        self.map(f)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_option_functor() {
        assert_eq!(None.fmap(|x: i32| x + 1), None);
        assert_eq!(Some(1).fmap(|x: i32| x + 1), Some(2));
    }
}
