// use super::applicative::Applicative;
// use super::functor::Functor;


use super::functor::Functor;

pub trait Monad: Functor {
    type Pure<A_>: Monad<HktArg1 = A_>;

    type M<B_>: Monad<HktArg1 = B_>;

    type Bind<B_, Mf_>: Monad<HktArg1 = B_>
    where
        Mf_: Fn(Self::HktArg1) -> Self::M<B_> + Clone;

    fn pure<A>(a: A) -> Self::Pure<A>;

    fn bind<B, Mf_>(self, mf: Mf_) -> Self::Bind<B, Mf_>
    where
        Mf_: Fn(Self::HktArg1) -> Self::M<B> + Clone;
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     impl<T> Monad for Option<T> {
//         fn and_then<B, F>(self, f: F) -> Self::Of<B>
//         where
//             F: Fn(T) -> Self::Of<B>,
//         {
//             match self {
//                 Some(x) => f(x),
//                 None => None,
//             }
//         }
//     }

//     #[test]
//     fn test_option_monad() {
//         assert_eq!(None.and_then(|x: i32| Some(x + 1)), None);
//         assert_eq!(Some(1).and_then(|x: i32| Some(x + 1)), Some(2));

//         assert_eq!(None::<i32>.then(None::<i32>), None);
//         assert_eq!(None::<i32>.then(Some(1)), None);

//         assert_eq!(Some(1).then(None::<i32>), None);
//         assert_eq!(Some(1).then(Some(2)), Some(2));
//     }
// }
