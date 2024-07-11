// use super::functor::Functor;

// pub trait Applicative: Functor {
//     fn pure<A>(a: A) -> Self::Of<A>;

//     fn apply<B, F>(self, f: Self::Of<F>) -> Self::Of<B>
//     where
//         F: Fn(Self::HktOf1) -> B;
// }

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use crate::base::hkt::Hkt1;
//     use crate::impl_hkt1;

//     impl<T> Applicative for Option<T> {
//         fn pure<A>(a: A) -> Self::Of<A> {
//             Some(a)
//         }

//         fn apply<B, F>(self, f: Self::Of<F>) -> Self::Of<B>
//         where
//             F: Fn(Self::HktOf1) -> B,
//         {
//             match (self, f) {
//                 (Some(x), Some(f)) => Some(f(x)),
//                 _ => None,
//             }
//         }
//     }

//     #[test]
//     fn test_applicative_pure() {
//         assert_eq!(Option::<i32>::pure(1), Some(1));
//     }

//     #[test]
//     fn test_applicative_apply() {
//         assert_eq!(None::<i32>.apply(None::<Fn(i32) -> i32>), None);
//         assert_eq!(None.apply(Some(|x: i32| x + 1)), None);

//         assert_eq!(Some(1).apply(None::<Fn(i32) -> i32>), None);
//         assert_eq!(Some(1).apply(Some(|x: i32| x + 1)), Some(2));
//     }
// }

use super::functor::Functor;

pub trait Applicative<A>: Functor<A> {
    type Pure<T>: Applicative<T>;
    type Ap<B, F>: Applicative<B>
    where
        F: Fn(A) -> B + Clone;

    fn pure(a: A) -> Self::Pure<A>;

    fn apply<B, F, Af>(self, af: Af) -> Self::Ap<B, F>
    where
        F: Fn(A) -> B + Clone,
        Af: Applicative<F>;
}

#[cfg(test)]
mod tests {

    use crate::{
        base::hkt::{HktIter, HktOption},
        data::option::HktOption,
    };

    use super::*;

    // impl<A> Applicative<A> for Option<A> {
    //     type Pure<T> = Option<T>;
    //     // type Apf<B, F> = Option<F>
    //     // where
    //     //     F: Fn(A) -> B + Clone;
    //     type Ap<B, F> = Option<B>
    //     where
    //         F: Fn(A) -> B + Clone;

    //     fn pure(a: A) -> Self::Pure<A> {
    //         Some(a)
    //     }

    //     // fn apply<B, F>(self, f: Self::Apf<B, F>) -> Self::Ap<B, F>
    //     fn apply<B, F, Af>(self, f: Af) -> Self::Ap<B, F>
    //     where
    //         F: Fn(A) -> B + Clone,
    //         Af: Applicative<F> + HktOption<F>,
    //     {
    //         match (self, f) {
    //             (Some(x), Some(f)) => Some(f(x)),
    //             _ => None,
    //         }
    //     }
    // }

    impl<A, O> Applicative<A> for O
    where
        O: HktOption<Arg1 = A>,
    {
        type Pure<T> = Option<A>;

        type Ap<B, F> = Option<B>
        where
            F: Fn(A) -> B + Clone;

        fn pure(a: A) -> Self::Pure<A> {
            Some(a)
        }

        // fn apply<B, F>(self, f: Self::Apf<B, F>) -> Self::Ap<B, F>
        fn apply<B, F, Af>(self, af: Af) -> Self::Ap<B, F>
        where
            F: Fn(A) -> B + Clone,
            Af: Applicative<F> + HktOption<Arg1 = F>,
        {
            self.fmap_or_else(&|| None, &|x| af.fmap_or_else(&|| None, &|f| Some(f(x))))
        }
    }

    impl<A, I> Applicative<A> for I
    where
        I: HktIter + Iterator<Item = A>,
    {
        type Pure<T> = std::iter::Once<T>;
        // type Apf<B, F> = std::iter::Map<I, F>
        // where
        //     F: Fn(A) -> B + Clone;
        type Ap<B, F> =  std::iter::FlatMap<Self, std::iter::Map<std::vec::IntoIter<F>, F>, F>
        where
            F: Fn(A) -> B + Clone;

        fn pure(a: A) -> Self::Pure<A> {
            std::iter::once(a)
        }

        // fn apply<B, F>(self, f: Self::Apf<B, F>) -> Self::Ap<B, F>
        fn apply<B, F, Af>(self, f: std::iter::Map<std::iter::Once<F>, F>) -> Self::Ap<B, F>
        where
            F: Fn(A) -> B + Clone,
        {
            // Cartesian product
            self.flat_map(move |x| f.clone().map(move |f| f(x)))
            // A -> List<B>
        }
    }

    // Impl for std::vec::IntoIter<A>
    // Map<std::vec::IntoIter<F>

    // impl<A> Applicative<A> for std::vec::IntoIter<A> {
    //     type Pure<T> = std::vec::IntoIter<T>;
    //     type Apf<B, F> = std::vec::IntoIter<F>
    //     where
    //         F: Fn(A) -> B + Clone;

    //     // FlatMap<std::vec::IntoIter<A>, Map<std::vec::IntoIter<F>, {closure@src/data/applicative.rs:120:50: 120:58}>, {closure@src/data/applicative.rs:120:27: 120:35}>
    //     type Ap<B, F> = std::iter::FlatMap<Self, std::iter::Map<std::vec::IntoIter<F>, F>, F>
    //     where
    //         F: Fn(A) -> B + Clone;

    //     fn pure(a: A) -> Self::Pure<A> {
    //         vec![a].into_iter()
    //     }

    //     fn apply<B, F>(self, f: Self::Apf<B, F>) -> Self::Ap<B, F>
    //     where
    //         F: Fn(A) -> B + Clone,
    //     {
    //         // Cartesian product
    //         self.flat_map(move |x| f.clone().map(move |f| f(x)))
    //         // A -> List<B>
    //     }
    // }

    #[test]
    fn test_applicative_pure() {
        assert_eq!(Option::<i32>::pure(1), Some(1));
    }

    #[test]
    fn test_applicative_apply() {
        assert_eq!(None::<i32>.apply(None::<fn(i32) -> i32>), None);
        assert_eq!(None.apply(Some(|x: i32| x + 1)), None);

        assert_eq!(Some(1).apply(None::<fn(i32) -> i32>), None);
        assert_eq!(Some(1).apply(Some(|x: i32| x + 1)), Some(2));
    }
}
