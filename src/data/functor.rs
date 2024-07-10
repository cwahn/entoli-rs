use crate::base::hkt::Hkt1;

pub trait Functor<A>: Sized {
    type Map<B, F>: Functor<B>
    where
        F: Fn(A) -> B + Clone;

    fn fmap<B, F>(self, f: F) -> Self::Map<B, F>
    where
        F: Fn(A) -> B + Clone;

    fn fmap1<F>(self, f: F) -> Self::Map<A, F>
    where
        F: Fn(A) -> A + Clone;
}

#[cfg(test)]
mod tests {

    use std::iter;

    use super::*;
    // use crate::impl_hkt1;

    // impl_hkt1!(Option);

    impl<A> Functor<A> for Option<A> {
        type Map<B, F> = Option<B>
        where
            F: Fn(A) -> B + Clone;

        fn fmap<B, F>(self, f: F) -> Option<B>
        where
            F: Fn(A) -> B,
        {
            match self {
                Some(x) => Some(f(x)),
                None => None,
            }
        }

        fn fmap1<F>(self, f: F) -> Option<A>
        where
            F: Fn(A) -> A,
        {
            match self {
                Some(x) => Some(f(x)),
                None => None,
            }
        }
    }

    // ! If I do this I can not implement Functor trait for any external type.

    // impl<A, I> Functor<A> for I

    // Impl for std::vec::IntoIter<A>

    impl<A> Functor<A> for std::vec::IntoIter<A> {
        type Map<B, F> = std::iter::Map<Self, F>
            where
                F: Fn(A) -> B + Clone;

        fn fmap<B, F>(self, f: F) -> std::iter::Map<Self, F>
        where
            F: Fn(A) -> B + Clone,
        {
            self.map(f)
        }

        fn fmap1<F>(self, f: F) -> std::iter::Map<Self, F>
        where
            F: Fn(A) -> A + Clone,
        {
            self.map(f)
        }
    }

    // Impl for std::slice::Iter<'a, A>

    impl<'a, A> Functor<&'a A> for std::slice::Iter<'a, A> {
        type Map<B, F> = std::iter::Map<Self, F>
            where
                F: Fn(&'a A) -> B + Clone;

        fn fmap<B, F>(self, f: F) -> std::iter::Map<Self, F>
        where
            F: Fn(&'a A) -> B + Clone,
        {
            self.map(f)
        }

        fn fmap1<F>(self, f: F) -> std::iter::Map<Self, F>
        where
            F: Fn(&'a A) -> &'a A + Clone, // ?! Can I implement this for reference type?
        {
            self.map(f)
        }
    }

    // Impl for std::iter::Map<I, F>

    impl<I, A, B, F> Functor<B> for iter::Map<I, F>
    where
        I: Iterator<Item = A>,
        F: Fn(A) -> B + Clone,
    {
        type Map<C, G> = iter::Map<Self, G>
        where
            G: Fn(B) -> C + Clone;

        #[inline(always)]
        fn fmap<C, G>(self, g: G) -> iter::Map<Self, G>
        where
            G: Fn(B) -> C + Clone,
        {
            self.map(g)
        }

        #[inline(always)]
        fn fmap1<G>(self, g: G) -> iter::Map<Self, G>
        where
            G: Fn(B) -> B + Clone,
        {
            self.map(g)
        }
    }

    #[test]
    fn test_option_functor() {
        assert_eq!(None.fmap(&|x: i32| x + 1), None);
        assert_eq!(Some(1).fmap(&|x: i32| x + 1), Some(2));
    }

    #[test]
    fn test_vec_into_iter_functor() {
        assert_eq!(
            Vec::new()
                .into_iter()
                .fmap(&|x: i32| x + 1)
                .collect::<Vec<_>>(),
            Vec::new()
        );

        let v = vec![1, 2, 3];
        assert_eq!(
            v.into_iter().fmap(&|x| x + 1).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_iter_functor() {
        assert_eq!(
            [].iter().fmap(&|x: &i32| x + 1).collect::<Vec<_>>(),
            Vec::new()
        );

        let v = vec![1, 2, 3];
        assert_eq!(v.iter().fmap(&|x| x + 1).collect::<Vec<_>>(), vec![2, 3, 4]);
    }
}
