// use super::applicative::Applicative;

// pub trait Monad: Applicative {
//     fn and_then<B, F>(self, f: F) -> Self::Of<B>
//     where
//         F: Fn(Self::HktOf1) -> Self::Of<B>;

//     fn then<B>(self, b: Self::Of<B>) -> Self::Of<B>
//     where
//         Self::Of<B>: Clone,
//     {
//         self.and_then(|_| b.clone())
//     }
// }

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
