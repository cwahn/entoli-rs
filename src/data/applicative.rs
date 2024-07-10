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
//         assert_eq!(None::<i32>.apply(None::<fn(i32) -> i32>), None);
//         assert_eq!(None.apply(Some(|x: i32| x + 1)), None);

//         assert_eq!(Some(1).apply(None::<fn(i32) -> i32>), None);
//         assert_eq!(Some(1).apply(Some(|x: i32| x + 1)), Some(2));
//     }
// }

use super::functor::Functor;

pub trait Applicative<A>: Functor<A> {
    type Pure<T>: Applicative<T>;
    type Apf<B, F>: Applicative<F>
    where
        F: Fn(A) -> B + Clone;

    type Ap<B, F>: Applicative<B>
    where
        F: Fn(A) -> B + Clone;

    fn pure(a: A) -> Self::Pure<A>;

    fn apply<B, F>(self, f: Self::Apf<B, F>) -> Self::Ap<B, F>
    where
        F: Fn(A) -> B + Clone;
}

#[cfg(test)]
mod tests {

    use super::*;

    impl<A> Applicative<A> for Option<A> {
        type Pure<T> = Option<T>;
        type Apf<B, F> = Option<F>
        where
            F: Fn(A) -> B + Clone;
        type Ap<B, F> = Option<B>
        where
            F: Fn(A) -> B + Clone;

        fn pure(a: A) -> Self::Pure<A> {
            Some(a)
        }

        fn apply<B, F>(self, f: Self::Apf<B, F>) -> Self::Ap<B, F>
        where
            F: Fn(A) -> B + Clone,
        {
            match (self, f) {
                (Some(x), Some(f)) => Some(f(x)),
                _ => None,
            }
        }
    }

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
