use crate::base::hkt::Hkt1;

// pub trait Functor<A>: Sized {
//     type Map<B, F>: Functor<B>
//     where
//         F: Fn(A) -> B + Clone;

//     fn fmap<B, F>(self, f: F) -> Self::Map<B, F>
//     where
//         F: Fn(A) -> B + Clone;

//     fn fmap1<F>(self, f: F) -> Self::Map<A, F>
//     where
//         F: Fn(A) -> A + Clone;
// }

pub trait Functor: Hkt1 + Sized {
    type Map<B, F>: Functor
    where
        F: Fn(Self::HktArg1) -> B + Clone;

    fn fmap<B, F>(self, f: F) -> Self::Map<B, F>
    where
        F: Fn(Self::HktArg1) -> B + Clone;

    fn fmap1<F>(self, f: F) -> Self::Map<Self::HktArg1, F>
    where
        F: Fn(Self::HktArg1) -> Self::HktArg1 + Clone;
}

#[cfg(test)]
mod tests {

    

    use super::*;
    use crate::impl_hkt1;
    // ? Only needed for endo functor on family of types
    

    // impl<A> Functor<A> for Option<A> {
    //     type Map<B, F> = Option<B>
    //     where
    //         F: Fn(A) -> B + Clone;

    //     fn fmap<B, F>(self, f: F) -> Option<B>
    //     where
    //         F: Fn(A) -> B,
    //     {
    //         match self {
    //             Some(x) => Some(f(x)),
    //             None => None,
    //         }
    //     }

    //     fn fmap1<F>(self, f: F) -> Option<A>
    //     where
    //         F: Fn(A) -> A,
    //     {
    //         match self {
    //             Some(x) => Some(f(x)),
    //             None => None,
    //         }
    //     }
    // }

    impl_hkt1!(Option);

    impl<A> Functor for Option<A> {
        type Map<B, F> = Option<B>
        where
            F: Fn(A) -> B + Clone;

        fn fmap<B, F>(self, f: F) -> Option<B>
        where
            F: Fn(A) -> B + Clone,
        {
            self.map(f)
        }

        fn fmap1<F>(self, f: F) -> Option<A>
        where
            F: Fn(A) -> A + Clone,
        {
            self.map(f)
        }
    }

    // ! If I do this I can not implement Functor trait for any external type.

    // impl<A, I> Functor<A> for I
    // where
    //     I: HktIter + Iterator<Item = A>,
    // {
    //     type Map<B, F> = iter::Map<I, F>
    //     where
    //         F: Fn(A) -> B + Clone;

    //     fn fmap<B, F>(self, f: F) -> iter::Map<I, F>
    //     where
    //         F: Fn(A) -> B + Clone,
    //     {
    //         self.map(f)
    //     }

    //     fn fmap1<F>(self, f: F) -> iter::Map<I, F>
    //     where
    //         F: Fn(A) -> A + Clone,
    //     {
    //         self.map(f)
    //     }
    // }

    // impl<T> HktIter for std::vec::IntoIter<T> {}
    // impl<'a, T> HktIter for std::slice::Iter<'a, T> {}
    // impl<T> HktIter for std::iter::Once<T> {}
    // impl<T, B, F> HktIter for std::iter::Map<T, F>
    // where
    //     T: Iterator,
    //     F: Fn(T::Item) -> B + Clone,
    // {
    // }

    // impl<A, B, I, U, F> HktIter for std::iter::FlatMap<I, U, F>
    // where
    //     I: Iterator<Item = A>,
    //     U: Iterator<Item = B>,
    //     F: Fn(A) -> U + Clone,
    // {
    // }

    // // Impl for std::vec::IntoIter<A>

    // impl<A> Functor<A> for std::vec::IntoIter<A> {
    //     type Map<B, F> = std::iter::Map<Self, F>
    //         where
    //             F: Fn(A) -> B + Clone;

    //     fn fmap<B, F>(self, f: F) -> std::iter::Map<Self, F>
    //     where
    //         F: Fn(A) -> B + Clone,
    //     {
    //         self.map(f)
    //     }

    //     fn fmap1<F>(self, f: F) -> std::iter::Map<Self, F>
    //     where
    //         F: Fn(A) -> A + Clone,
    //     {
    //         self.map(f)
    //     }
    // }

    // impl_hkt1!(std::vec::IntoIter);

    impl<A> Hkt1 for std::vec::IntoIter<A> {
        type HktArg1 = A;
    }

    impl<A> Functor for std::vec::IntoIter<A> {
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

    // // Impl for std::slice::Iter<'a, A>

    // impl<'a, A> Functor<&'a A> for std::slice::Iter<'a, A> {
    //     type Map<B, F> = std::iter::Map<Self, F>
    //         where
    //             F: Fn(&'a A) -> B + Clone;

    //     fn fmap<B, F>(self, f: F) -> std::iter::Map<Self, F>
    //     where
    //         F: Fn(&'a A) -> B + Clone,
    //     {
    //         self.map(f)
    //     }

    //     fn fmap1<F>(self, f: F) -> std::iter::Map<Self, F>
    //     where
    //         F: Fn(&'a A) -> &'a A + Clone, // ?! Can I implement this for reference type?
    //     {
    //         self.map(f)
    //     }
    // }

    impl<A> Hkt1 for std::iter::Once<A> {
        type HktArg1 = A;
    }

    impl<A> Functor for std::iter::Once<A> {
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

    // // Impl for std::iter::Map<I, F>

    // impl<I, A, B, F> Functor<B> for std::iter::Map<I, F>
    // where
    //     I: Iterator<Item = A>,
    //     F: Fn(A) -> B + Clone,
    // {
    //     type Map<C, G> = std::iter::Map<Self, G>
    //     where
    //         G: Fn(B) -> C + Clone;

    //     #[inline(always)]
    //     fn fmap<C, G>(self, g: G) -> std::iter::Map<Self, G>
    //     where
    //         G: Fn(B) -> C + Clone,
    //     {
    //         self.map(g)
    //     }

    //     #[inline(always)]
    //     fn fmap1<G>(self, g: G) -> std::iter::Map<Self, G>
    //     where
    //         G: Fn(B) -> B + Clone,
    //     {
    //         self.map(g)
    //     }
    // }

    impl<I, A, B, F> Hkt1 for std::iter::Map<I, F>
    where
        I: Iterator<Item = A>,
        F: Fn(A) -> B + Clone,
    {
        type HktArg1 = B;
    }

    impl<I, A, B, F> Functor for std::iter::Map<I, F>
    where
        I: Iterator<Item = A>,
        F: Fn(A) -> B + Clone,
    {
        type Map<C, G> = std::iter::Map<Self, G>
        where
            G: Fn(Self::HktArg1) -> C + Clone;

        #[inline(always)]
        fn fmap<C, G>(self, g: G) -> std::iter::Map<Self, G>
        where
            G: Fn(Self::HktArg1) -> C + Clone,
        {
            self.map(g)
        }

        #[inline(always)]
        fn fmap1<G>(self, g: G) -> std::iter::Map<Self, G>
        where
            G: Fn(Self::HktArg1) -> Self::HktArg1 + Clone,
        {
            self.map(g)
        }
    }

    // // Impl for std::iter::FlatMap<I, F>

    // impl<A, B, I, U, F> Functor<B> for std::iter::FlatMap<I, U, F>
    // where
    //     I: Iterator<Item = A>,
    //     U: Iterator<Item = B>,
    //     F: Fn(A) -> U + Clone,
    // {
    //     type Map<V, G> = std::iter::Map<Self, G>
    //     where
    //         G: Fn(B) -> V + Clone;

    //     #[inline(always)]
    //     fn fmap<V, G>(self, g: G) -> std::iter::Map<Self, G>
    //     where
    //         G: Fn(B) -> V + Clone,
    //     {
    //         self.map(g)
    //     }

    //     #[inline(always)]
    //     fn fmap1<G>(self, g: G) -> std::iter::Map<Self, G>
    //     where
    //         G: Fn(B) -> B + Clone,
    //     {
    //         self.map(g)
    //     }
    // }

    impl<A, B, I, U, F> Hkt1 for std::iter::FlatMap<I, U, F>
    where
        I: Iterator<Item = A>,
        U: Iterator<Item = B>,
        F: Fn(A) -> U + Clone,
    {
        type HktArg1 = B;
    }

    impl<A, B, I, U, F> Functor for std::iter::FlatMap<I, U, F>
    where
        I: Iterator<Item = A>,
        U: Iterator<Item = B>,
        F: Fn(A) -> U + Clone,
    {
        type Map<V, G> = std::iter::Map<Self, G>
        where
            G: Fn(B) -> V + Clone;

        #[inline(always)]
        fn fmap<V, G>(self, g: G) -> std::iter::Map<Self, G>
        where
            G: Fn(B) -> V + Clone,
        {
            self.map(g)
        }

        #[inline(always)]
        fn fmap1<G>(self, g: G) -> std::iter::Map<Self, G>
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
            Vec::<i32>::new()
                .into_iter()
                .fmap(&|x: i32| x + 1)
                .collect::<Vec<_>>(),
            Vec::<i32>::new()
        );

        let v = vec![1, 2, 3];
        assert_eq!(
            v.into_iter().fmap(&|x| x + 1).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
    }

    // #[test]
    // fn test_iter_functor() {
    //     assert_eq!(
    //         [].iter().fmap(&|x: &i32| x + 1).collect::<Vec<_>>(),
    //         Vec::<i32>::new()
    //     );

    //     let v = vec![1, 2, 3];
    //     assert_eq!(v.iter().fmap(&|x| x + 1).collect::<Vec<_>>(), vec![2, 3, 4]);
    // }

    #[test]
    fn test_map_functor() {
        // let it0 = Vec::<i32>::new().into_iter();
        fn f0(x: i32) -> i32 {
            x + 1
        }

        let f0_ = |x: i32| x + 1;

        let map0 = Vec::<i32>::new().into_iter().map(f0);
        let map1 = vec![1, 2, 3].into_iter().map(f0_);

        assert_eq!(map0.fmap(|x| x + 1).collect::<Vec<_>>(), Vec::<i32>::new());
        assert_eq!(map1.fmap(|x| x + 1).collect::<Vec<_>>(), vec![3, 4, 5]);
    }
}
