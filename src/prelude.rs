use crate::{
    base::hkt::Hkt1,
    data::{
        functor::Functor,
        // monad::Monad
    },
    impl_hkt1,
};

// Tuples

pub fn fst<A, B>((a, _): (A, B)) -> A {
    a
}

pub fn snd<A, B>((_, b): (A, B)) -> B {
    b
}

// todo curry, uncurry

// pub fn curry<A, B, C>(f: impl Fn(A, B) -> C) -> impl Fn(A) -> impl Fn(B) -> C {
//     move |a| move |b| f(a, b)
// }

//  Folds and traversals

pub fn foldl<A, B, F>(f: F, acc: B, xs: impl IntoIterator<Item = A>) -> B
where
    F: Fn(B, A) -> B,
{
    xs.into_iter().fold(acc, move |acc, a| f(acc, a))
}

// todo foldr

pub fn elem<A>(x: A, xs: impl IntoIterator<Item = A>) -> bool
where
    A: PartialEq,
{
    xs.into_iter().any(|a| a == x)
}

// maximum, minimum, sum, product, any, all

pub fn maximum<A>(xs: impl IntoIterator<Item = A>) -> Option<A>
where
    A: Ord,
{
    xs.into_iter().max()
}

pub fn minimum<A>(xs: impl IntoIterator<Item = A>) -> Option<A>
where
    A: Ord,
{
    xs.into_iter().min()
}

// todo Monid
pub fn sum<A>(xs: impl IntoIterator<Item = A>) -> A
where
    A: std::ops::Add<Output = A> + Default,
{
    xs.into_iter().fold(Default::default(), |acc, a| acc + a)
}

// todo Monid
// ! Need to implement unit for Monid with Mul
// pub fn product<A>(xs: impl IntoIterator<Item = A>) -> A
// where
//     A: std::ops::Mul<Output = A> + Default,
// {
//     xs.into_iter().fold(1 as A, |acc, a| acc * a)
// }

pub fn any<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> bool
where
    F: Fn(A) -> bool,
{
    xs.into_iter().any(|a| f(a))
}

pub fn all<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> bool
where
    F: Fn(A) -> bool,
{
    xs.into_iter().all(|a| f(a))
}

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

// #[inline(always)]
// pub fn foldl<'a, A, B, F>(f: F, acc: B, xs: impl IntoIterator<Item = A>) -> B
// where
//     F: for<'b> Fn(B, &'b A) -> B,
// {
//     xs.into_iter().fold(acc, move |acc, a| f(acc, &a))
// }

// todo Io

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fst() {
        assert_eq!(fst((1, 2)), 1);
    }

    #[test]
    fn test_snd() {
        assert_eq!(snd((1, 2)), 2);
    }

    // Fold and traverse

    #[test]
    fn test_foldl() {
        assert_eq!(foldl(|acc, x| acc + x, 0, Vec::<i32>::new()), 0);

        assert_eq!(foldl(|acc, x| acc + x, 0, vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_elem() {
        assert_eq!(elem(1, Vec::<i32>::new()), false);

        assert_eq!(elem(1, vec![2, 3, 4]), false);
        assert_eq!(elem(1, vec![1, 2, 3]), true);
    }

    #[test]
    fn test_maximum() {
        assert_eq!(maximum(Vec::<i32>::new()), None);

        assert_eq!(maximum(vec![1, 2, 3, 4, 5]), Some(5));
    }

    #[test]
    fn test_minimum() {
        assert_eq!(minimum(Vec::<i32>::new()), None);

        assert_eq!(minimum(vec![1, 2, 3, 4, 5]), Some(1));
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(Vec::<i32>::new()), 0);

        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    // #[test]
    // fn test_product() {
    //     assert_eq!(product(Vec::<i32>::new()), 1);

    //     assert_eq!(product(vec![1, 2, 3, 4, 5]), 120);
    // }

    #[test]
    fn test_any() {
        assert_eq!(any(|x| x > 0, Vec::<i32>::new()), false);

        assert_eq!(any(|x| x > 0, vec![-1, -2, -3]), false);
        assert_eq!(any(|x| x > 0, vec![-1, 2, -3]), true);
    }

    #[test]
    fn test_all() {
        assert_eq!(all(|x| x > 0, Vec::<i32>::new()), true);

        assert_eq!(all(|x| x > 0, vec![-1, -2, -3]), false);
        assert_eq!(all(|x| x > 0, vec![1, 2, 3]), true);
    }

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
        let ys: Vec<i32> = filter(|x| *x > 0, xs).collect();

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
