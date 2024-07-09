use crate::{
    base::hkt::Hkt1,
    data::{functor::Functor, monad::Monad},
    impl_hkt1,
};

#[inline(always)]
pub fn map<'a, A, B, F>(f: F, xs: impl IntoIterator<Item = A>) -> impl Iterator<Item = B>
where
    F: for<'b> Fn(&'b A) -> B,
{
    xs.into_iter().map(move |a| f(&a))
}

#[inline(always)]
pub fn filter<'a, A, F>(f: F, xs: impl IntoIterator<Item = A>) -> impl Iterator<Item = A>
where
    F: for<'b> Fn(&'b A) -> bool,
{
    xs.into_iter().filter(move |a| f(&a))
}

#[inline(always)]
pub fn foldl<'a, A, B, F>(f: F, acc: B, xs: impl IntoIterator<Item = A>) -> B
where
    F: for<'b> Fn(B, &'b A) -> B,
{
    xs.into_iter().fold(acc, move |acc, a| f(acc, &a))
}

// Io

// pub trait Io: Monad {
//     type Item;

//     fn run(self) -> Self::Item;
// }

// pub struct IoMap<A, B, F>
// where
//     F: Fn(A::Item) -> B,
//     A: Io,
// {
//     io: A,
//     f: F,
// }

// // impl_hkt1!(IoMap);

// impl<A, B, F> Hkt1 for IoMap<A, B, F>
// where
//     F: Fn(A::Item) -> B,
//     A: Io,
// {
//     type HktOf1 = A::Item;
//     type Of<T1> = IoMap<A, T1, F>;
// }

// impl<A, B, F> Functor for IoMap<A, B, F>
// where
//     F: Fn(A::Item) -> B,
//     A: Io,
// {
//     type Item = B;
//     type HktOf1 = A::HktOf1;

//     fn fmap<G>(self, g: G) -> Self::Of<B>
//     where
//         G: Fn(Self::HktOf1) -> B,
//     {
//         IoMap {
//             io: self.io,
//             f: |x| g(self.f(x)),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_0() {
        let xs = Vec::<i32>::new();
        let ys: Vec<i32> = map(|x| x + 1, xs).collect();

        assert_eq!(ys, Vec::<i32>::new());
    }

    #[test]
    fn test_map_1() {
        let xs = vec![1, 2, 3];
        let ys: Vec<i32> = map(|x| x + 1, xs).collect();

        assert_eq!(ys, vec![2, 3, 4]);
    }

    #[test]
    fn test_filter_0() {
        let xs = Vec::<i32>::new();
        let ys: Vec<i32> = filter(|x| *x > 0, xs).collect();

        assert_eq!(ys, Vec::<i32>::new());
    }

    #[test]
    fn test_filter_1() {
        let xs = vec![1, -2, 3, -4, 5];
        let mut ys: Vec<i32> = filter(|x| *x > 0, xs).collect();

        assert_eq!(ys, vec![1, 3, 5]);
    }

    #[test]
    fn test_foldl_0() {
        let xs = Vec::<i32>::new();
        let acc = 0;
        let sum = foldl(|acc, x| acc + x, acc, xs);

        assert_eq!(sum, 0);
    }

    #[test]
    fn test_foldl_1() {
        let xs = vec![1, 2, 3, 4, 5];
        let acc = 0;
        let sum = foldl(|acc, x| acc + x, acc, xs);

        assert_eq!(sum, 15);
    }
}
