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

// todo reverse
// #[inline(always)]
// pub fn reverse<A>(xs: impl DoubleEndedIterator<Item = A>) -> Vec<A> {
//     xs.rev().collect()
// }

#[inline(always)]
pub fn concat<A, As>(xss: As) -> std::iter::Flatten<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator,
    As::Item: IntoIterator<Item = A>,
{
    xss.into_iter().flatten()
}

#[inline(always)]
pub fn concat_map<A, As, Bs, F>(
    f: F,
    xs: As,
) -> std::iter::FlatMap<<As as IntoIterator>::IntoIter, Bs, F>
where
    As: IntoIterator<Item = A>,
    Bs: IntoIterator,
    F: Fn(A) -> Bs,
{
    xs.into_iter().flat_map(f)
}

// Building lists

pub fn scanl<A, B, F>(
    f: F,
    init: B,
    xs: impl IntoIterator<Item = A>,
) -> std::iter::FromFn<impl FnMut() -> Option<B>>
where
    F: Fn(&B, A) -> B,
    B: Clone,
{
    let mut acc = Some(init.clone());
    let mut iter = xs.into_iter();

    std::iter::from_fn(move || {
        if let Some(b) = acc.take() {
            acc = iter.next().map(|x| f(&b, x));
            Some(b)
        } else {
            None
        }
    })
}

pub fn scanl1<A, F>(
    f: F,
    xs: impl IntoIterator<Item = A>,
) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    F: Fn(&A, A) -> A,
    A: Clone,
{
    let mut iter = xs.into_iter();
    let mut acc = iter.next();

    std::iter::from_fn(move || {
        if let Some(b) = acc.take() {
            acc = iter.next().map(|x| f(&b, x));
            Some(b)
        } else {
            None
        }
    })
}

// todo scanr, scanr1

// Infinite lists

//  iterate, repeat, replicate, cycle

pub fn iterate<A, F>(f: F, a: A) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    F: Fn(A) -> A,
    A: Clone,
{
    let mut acc = Some(a);
    std::iter::from_fn(move || {
        let a = acc.take()?;
        acc = Some(f(a.clone()));
        Some(a)
    })
}

pub fn repeat<A>(a: A) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    A: Clone,
{
    std::iter::from_fn(move || Some(a.clone()))
}

pub fn replicate<A>(n: usize, a: A) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    A: Clone,
{
    let mut i = 0;
    std::iter::from_fn(move || {
        if i < n {
            i += 1;
            Some(a.clone())
        } else {
            None
        }
    })
}

pub fn cycle<As>(xs: As) -> std::iter::Cycle<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator,
    <As as IntoIterator>::IntoIter: Clone,
{
    xs.into_iter().cycle()
}

// Sublists

// todo take, drop, split_at, take_while, drop_while, span

#[inline(always)]
pub fn take<A, As>(n: usize, xs: As) -> std::iter::Take<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
{
    xs.into_iter().take(n)
}

#[inline(always)]
pub fn drop<A, As>(n: usize, xs: As) -> std::iter::Skip<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
{
    xs.into_iter().skip(n)
}

#[inline(always)]
pub fn take_while<A, As, F>(f: F, xs: As) -> std::iter::TakeWhile<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(&A) -> bool,
{
    xs.into_iter().take_while(f)
}

#[inline(always)]
pub fn drop_while<A, As, F>(f: F, xs: As) -> std::iter::SkipWhile<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(&A) -> bool,
{
    xs.into_iter().skip_while(f)
}

#[inline(always)]
pub fn span<A, As, F>(
    f: F,
    xs: As,
) -> (
    std::iter::TakeWhile<<As as IntoIterator>::IntoIter, F>,
    std::iter::SkipWhile<<As as IntoIterator>::IntoIter, F>,
)
where
    As: IntoIterator<Item = A>,
    <As as IntoIterator>::IntoIter: Clone,
    F: Fn(&A) -> bool + Clone,
{
    let iter = xs.into_iter();
    (iter.clone().take_while(f.clone()), iter.skip_while(f))
}

#[inline(always)]
pub fn split_at<A, As>(
    n: usize,
    xs: As,
) -> (
    std::iter::Take<<As as IntoIterator>::IntoIter>,
    std::iter::Skip<<As as IntoIterator>::IntoIter>,
)
where
    As: IntoIterator<Item = A>,
    <As as IntoIterator>::IntoIter: Clone,
{
    let iter = xs.into_iter();
    (iter.clone().take(n), iter.skip(n))
}

// No break since it is a keyword

// Zipping and unzipping lists

#[inline(always)]
pub fn zip<A, B, As, Bs>(
    xs: As,
    ys: Bs,
) -> std::iter::Zip<<As as IntoIterator>::IntoIter, <Bs as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
    Bs: IntoIterator<Item = B>,
{
    xs.into_iter().zip(ys)
}

#[inline(always)]
pub fn zip_with<A, B, C, As, Bs, F>(
    f: F,
    xs: As,
    ys: Bs,
) -> std::iter::Map<
    std::iter::Zip<<As as IntoIterator>::IntoIter, <Bs as IntoIterator>::IntoIter>,
    impl FnMut((A, B)) -> C,
>
where
    As: IntoIterator<Item = A>,
    Bs: IntoIterator<Item = B>,
    F: Fn(A, B) -> C,
{
    xs.into_iter().zip(ys).map(move |(a, b)| f(a, b))
}

pub fn unzip<A, B, FromA, FromB>(xs: impl IntoIterator<Item = (A, B)>) -> (FromA, FromB)
where
    FromA: Default + Extend<A>,
    FromB: Default + Extend<B>,
{
    let mut as_ = FromA::default();
    let mut bs = FromB::default();

    for (a, b) in xs {
        as_.extend(Some(a));
        bs.extend(Some(b));
    }

    (as_, bs)
}

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

    #[test]
    fn test_concat() {
        assert_eq!(
            concat(Vec::<Vec<i32>>::new()).collect::<Vec<_>>(),
            Vec::<i32>::new()
        );

        assert_eq!(
            concat(vec![vec![1, 2], vec![3, 4]]).collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_concat_map() {
        assert_eq!(
            concat_map(|x| vec![x, x], Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );
        assert_eq!(
            concat_map(|x| vec![x, x], vec![1, 2, 3]).collect::<Vec<_>>(),
            vec![1, 1, 2, 2, 3, 3]
        );
    }

    // Building lists

    #[test]
    fn test_scanl() {
        assert_eq!(
            scanl(|acc, x| *acc && (x % 2 == 0), true, Vec::<i32>::new()).collect::<Vec<_>>(),
            vec![true]
        );

        assert_eq!(
            // Take until value is all even
            scanl(|acc, x| *acc && (x % 2 == 0), true, vec![0, 2, 4, 5, 7]).collect::<Vec<_>>(),
            vec![
                true, true,  // 0
                true,  // 0 && 2
                true,  // 0 && 2 && 4
                false, // 0 && 2 && 4 && 5
                false, // 0 && 2 && 4 && 5 && 7
            ]
        );
    }

    #[test]
    fn test_scanl1() {
        assert_eq!(
            scanl1(|acc, x| acc + x, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            scanl1(|acc, x| acc + x, vec![1, 2, 3, 4, 5]).collect::<Vec<_>>(),
            vec![1, 3, 6, 10, 15]
        );
    }

    // Infinite lists

    #[test]
    fn test_iterate() {
        assert_eq!(
            iterate(|x| x + 1, 0).take(5).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(1).take(5).collect::<Vec<_>>(), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_replicate() {
        assert_eq!(replicate(5, 1).collect::<Vec<_>>(), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_cycle() {
        assert_eq!(
            cycle(vec![1, 2, 3]).take(7).collect::<Vec<_>>(),
            vec![1, 2, 3, 1, 2, 3, 1]
        );
    }

    // Sublists

    #[test]
    fn test_take() {
        assert_eq!(take(0, Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(take(2, vec![1, 2, 3]).collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn test_drop() {
        assert_eq!(drop(0, Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(drop(2, vec![1, 2, 3]).collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn test_take_while() {
        assert_eq!(
            take_while(|x| x < &3, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            take_while(|x| x < &3, vec![1, 2, 3, 4, 5]).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn test_drop_while() {
        assert_eq!(
            drop_while(|x| x < &3, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            drop_while(|x| x < &3, vec![1, 2, 3, 4, 5]).collect::<Vec<_>>(),
            vec![3, 4, 5]
        );
    }

    #[test]
    fn test_span() {
        let (xs, ys) = span(|x| x < &3, vec![1, 2, 3, 4, 5]);

        assert_eq!(xs.collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(ys.collect::<Vec<_>>(), vec![3, 4, 5]);
    }

    #[test]
    fn test_split_at() {
        let (xs, ys) = split_at(2, vec![1, 2, 3, 4, 5]);

        assert_eq!(xs.collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(ys.collect::<Vec<_>>(), vec![3, 4, 5]);
    }

    #[test]
    fn test_zip() {
        assert_eq!(
            zip(vec![1, 2, 3], vec![4, 5, 6]).collect::<Vec<_>>(),
            vec![(1, 4), (2, 5), (3, 6)]
        );
    }

    #[test]
    fn test_zip_with() {
        assert_eq!(
            zip_with(|x, y| x + y, vec![1, 2, 3], vec![4, 5, 6]).collect::<Vec<_>>(),
            vec![5, 7, 9]
        );
    }

    #[test]
    fn test_unzip() {
        let (xs, ys): (Vec<_>, Vec<_>) = unzip(vec![(1, 4), (2, 5), (3, 6)]);

        assert_eq!(xs, vec![1, 2, 3]);
        assert_eq!(ys, vec![4, 5, 6]);
    }

    // No break since it is a keyword
}
