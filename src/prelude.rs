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
// pub fn foldr<A, B, F>(f: F, acc: B, xs: impl DoubleEndedIterator<Item = A>) -> B
// where
//     F: Fn(A, B) -> B,
// {
//     xs.rev().fold(acc, move |acc, a| f(a, acc))
// }

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

// Miscellaneous functions

pub fn id<A>(a: A) -> A {
    a
}

// List operations

#[inline(always)]
pub fn map<A, As, B, F>(f: F, xs: As) -> std::iter::Map<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(A) -> B,
{
    xs.into_iter().map(f)
}

// pub fn append<A>(
//     xs: impl IntoIterator<Item = A>,
//     ys: impl IntoIterator<Item = A>,
// ) -> impl IntoIterator<Item = A> {
//     xs.into_iter().chain(ys.into_iter())
// }

pub fn append<A, As1, As2>(
    xs: As1,
    ys: As2,
) -> std::iter::Chain<<As1 as IntoIterator>::IntoIter, <As2 as IntoIterator>::IntoIter>
where
    As1: IntoIterator<Item = A>,
    As2: IntoIterator<Item = A>,
{
    xs.into_iter().chain(ys.into_iter())
}

// #[inline(always)]
// pub fn filter<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> impl Iterator<Item = A>
// where
//     F: Fn(&A) -> bool,
// {
//     xs.into_iter().filter(f)
// }

#[inline(always)]
pub fn filter<A, As, F>(f: F, xs: As) -> std::iter::Filter<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(&A) -> bool,
{
    xs.into_iter().filter(f)
}

#[inline(always)]
pub fn head<A>(xs: impl IntoIterator<Item = A>) -> Option<A> {
    xs.into_iter().next()
}

#[inline(always)]
pub fn last<A>(xs: impl IntoIterator<Item = A>) -> Option<A> {
    xs.into_iter().last()
}

// #[inline(always)]
// pub fn tail<A>(xs: impl IntoIterator<Item = A>) -> impl Iterator<Item = A> {
//     let mut xs = xs.into_iter();
//     xs.next();
//     xs
// }

#[inline(always)]
pub fn tail<A, As>(xs: As) -> std::iter::Skip<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
{
    xs.into_iter().skip(1)
}

#[inline(always)]
pub fn init<A, As>(xs: As) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    As: IntoIterator<Item = A>,
{
    let mut iter = xs.into_iter().peekable();

    std::iter::from_fn(move || {
        let next = iter.next();
        if iter.peek().is_some() {
            next
        } else {
            None
        }
    })
}

#[inline(always)]
pub fn nth<A>(n: usize, xs: impl IntoIterator<Item = A>) -> Option<A> {
    xs.into_iter().nth(n)
}

#[inline(always)]
pub fn length<A>(xs: impl IntoIterator<Item = A>) -> usize {
    xs.into_iter().count()
}

// #[inline(always)]
// pub fn reverse<A>(xs: impl DoubleEndedIterator<Item = A>) -> Vec<A> {
//     xs.rev().collect()
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

    // #[test]
    // fn test_foldr() {
    //     assert_eq!(foldr(|x, acc| acc + x, 0, Vec::<i32>::new()), 0);

    //     assert_eq!(foldr(|x, acc| acc + x, 0, vec![1, 2, 3, 4, 5]), 15);
    // }

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

    // Miscellaneous functions

    #[test]
    fn test_id() {
        assert_eq!(id(1), 1);
    }

    // List operations

    #[test]
    fn test_map() {
        assert_eq!(
            map(|x| x + 1, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            map(|x| x + 1, vec![1, 2, 3]).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_append() {
        assert_eq!(
            append(Vec::<i32>::new(), Vec::<i32>::new())
                .into_iter()
                .collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            append(vec![1, 2, 3], vec![4, 5, 6])
                .into_iter()
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_filter() {
        assert_eq!(
            filter(|x| x > &0, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            filter(|x| x > &0, vec![-1, -2, -3]).collect::<Vec<_>>(),
            Vec::new()
        );
        assert_eq!(
            filter(|x| x > &0, vec![-1, 2, -3]).collect::<Vec<_>>(),
            vec![2]
        );
    }

    #[test]
    fn test_head() {
        assert_eq!(head(Vec::<i32>::new()), None);

        assert_eq!(head(vec![1, 2, 3]), Some(1));
    }

    #[test]
    fn test_last() {
        assert_eq!(last(Vec::<i32>::new()), None);

        assert_eq!(last(vec![1, 2, 3]), Some(3));
    }

    #[test]
    fn test_tail() {
        assert_eq!(tail(Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(tail(vec![1, 2, 3]).collect::<Vec<_>>(), vec![2, 3]);
    }

    #[test]
    fn test_init() {
        assert_eq!(init(Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(init(vec![1, 2, 3]).collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn test_nth() {
        assert_eq!(nth(0, Vec::<i32>::new()), None);

        assert_eq!(nth(0, vec![1, 2, 3]), Some(1));
    }

    #[test]
    fn test_length() {
        assert_eq!(length(Vec::<i32>::new()), 0);

        assert_eq!(length(vec![1, 2, 3]), 3);
    }

    // #[test]
    // fn test_reverse() {
    //     assert_eq!(reverse(Vec::<i32>::new().into_iter()), Vec::<i32>::new());

    //     assert_eq!(reverse(vec![1, 2, 3].into_iter()), vec![3, 2, 1]);
    // }
}
