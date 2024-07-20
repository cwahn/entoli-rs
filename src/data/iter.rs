// use crate::base::hkt::Hkt1;

// use super::functor::Functor;

// ! Not useful until the HKT family is implemented

// impl<A> Hkt1 for std::vec::IntoIter<A> {
//     type HktArg1 = A;
// }

// impl<A> Functor for std::vec::IntoIter<A> {
//     type Map<B, F> = std::iter::Map<Self, F>
//     where
//         F: Fn(A) -> B + Clone;

//     #[inline(always)]
//     fn fmap<B, F>(self, f: F) -> std::iter::Map<Self, F>
//     where
//         F: Fn(A) -> B + Clone,
//     {
//         self.map(f)
//     }

//     #[inline(always)]
//     fn fmap1<F>(self, f: F) -> std::iter::Map<Self, F>
//     where
//         F: Fn(A) -> A + Clone,
//     {
//         self.map(f)
//     }
// }

// impl<A> Hkt1 for std::iter::Once<A> {
//     type HktArg1 = A;
// }

// impl<A> Functor for std::iter::Once<A> {
//     type Map<B, F> = std::iter::Map<Self, F>
//     where
//         F: Fn(A) -> B + Clone;

//     #[inline(always)]
//     fn fmap<B, F>(self, f: F) -> std::iter::Map<Self, F>
//     where
//         F: Fn(A) -> B + Clone,
//     {
//         self.map(f)
//     }

//     #[inline(always)]
//     fn fmap1<F>(self, f: F) -> std::iter::Map<Self, F>
//     where
//         F: Fn(A) -> A + Clone,
//     {
//         self.map(f)
//     }
// }

// impl<I, A, B, F> Hkt1 for std::iter::Map<I, F>
// where
//     I: Iterator<Item = A>,
//     F: Fn(A) -> B + Clone,
// {
//     type HktArg1 = B;
// }

// impl<I, A, B, F> Functor for std::iter::Map<I, F>
// where
//     I: Iterator<Item = A>,
//     F: Fn(A) -> B + Clone,
// {
//     type Map<C, G> = std::iter::Map<Self, G>
//     where
//         G: Fn(Self::HktArg1) -> C + Clone;

//     #[inline(always)]
//     fn fmap<C, G>(self, g: G) -> std::iter::Map<Self, G>
//     where
//         G: Fn(Self::HktArg1) -> C + Clone,
//     {
//         self.map(g)
//     }

//     #[inline(always)]
//     fn fmap1<G>(self, g: G) -> std::iter::Map<Self, G>
//     where
//         G: Fn(Self::HktArg1) -> Self::HktArg1 + Clone,
//     {
//         self.map(g)
//     }
// }

// impl<A, B, I, U, F> Hkt1 for std::iter::FlatMap<I, U, F>
// where
//     I: Iterator<Item = A>,
//     U: Iterator<Item = B>,
//     F: Fn(A) -> U + Clone,
// {
//     type HktArg1 = B;
// }

// impl<A, B, I, U, F> Functor for std::iter::FlatMap<I, U, F>
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

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_vec_into_iter_functor() {
//         assert_eq!(
//             Vec::<i32>::new()
//                 .into_iter()
//                 .fmap(&|x: i32| x + 1)
//                 .collect::<Vec<_>>(),
//             Vec::<i32>::new()
//         );

//         let v = vec![1, 2, 3];
//         assert_eq!(
//             v.into_iter().fmap(&|x| x + 1).collect::<Vec<_>>(),
//             vec![2, 3, 4]
//         );
//     }

//     #[test]
//     fn test_map_functor() {
//         fn f0(x: i32) -> i32 {
//             x + 1
//         }

//         let f0_ = |x: i32| x + 1;

//         let map0 = Vec::<i32>::new().into_iter().map(f0);
//         let map1 = vec![1, 2, 3].into_iter().map(f0_);

//         assert_eq!(map0.fmap(|x| x + 1).collect::<Vec<_>>(), Vec::<i32>::new());
//         assert_eq!(map1.fmap(|x| x + 1).collect::<Vec<_>>(), vec![3, 4, 5]);
//     }

//     #[test]
//     fn test_once_functor() {
//         let once = std::iter::once(1);
//         assert_eq!(once.fmap(|x| x + 1).next(), Some(2));
//     }

//     #[test]
//     fn test_flat_map_functor() {
//         let flat_map = vec![1, 2, 3].into_iter().flat_map(|x| vec![x + 1, x + 2]);
//         assert_eq!(
//             flat_map.fmap(|x| x + 1).collect::<Vec<_>>(),
//             vec![3, 4, 4, 5, 5, 6]
//         );
//     }
// }
